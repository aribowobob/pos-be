use crate::errors::ServiceError;
use crate::models::sales::{SalesCart, SalesCartResponse, NewSalesCart, UpdateSalesCart, SalesOrder, SalesOrderDetail, CreateOrderRequest, OrderResponse,
    SalesReport, SalesReportOrder, SalesReportOrderItem, SkuSummaryItem, SalesSummary, SalesReportQuery, 
    DetailedOrderResponse, DetailedSalesOrder, DetailedSalesOrderDetail};
use crate::services::db_service::DbConnectionManager;
use chrono::Utc;
use log::{error, info};
use sqlx::{Row, Transaction, Postgres, FromRow};
use rust_decimal::Decimal;

// Struct for the order query result
#[derive(FromRow)]
struct OrderQueryResult {
    pub id: i32,
    pub order_number: String,
    pub user_id: i32,
    pub user_initial: String,
    pub store_id: i32,
    pub store_initial: String,
    pub date: chrono::NaiveDate,
    pub grand_total: Decimal,
    pub payment_cash: Decimal,
    pub payment_non_cash: Decimal,
    pub receivable: Decimal,
    pub created_at: chrono::NaiveDateTime,
    pub customer_id: Option<i32>,
    pub creator_company_id: i32,
}

pub async fn add_to_cart(
    db_manager: &DbConnectionManager,
    new_cart_item: NewSalesCart,
    user_id: i32, // User ID from authentication
) -> Result<SalesCart, ServiceError> {
    let pool = match db_manager.get_pool().await {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to get database connection: {:?}", e);
            return Err(ServiceError::DatabaseConnectionError);
        }
    };

    // Calculate sales price if not provided
    let sale_price = new_cart_item.sale_price.unwrap_or_else(|| {
        let discount_type = new_cart_item.discount_type.as_deref().unwrap_or("fixed");
        let discount_value = new_cart_item.discount_value.unwrap_or(0);
        let discount_amount = new_cart_item.discount_amount.unwrap_or_else(|| {
            if discount_type == "percentage" && discount_value > 0 {
                new_cart_item.base_price * rust_decimal::Decimal::new(discount_value as i64, 2)
            } else {
                rust_decimal::Decimal::new(discount_value as i64, 0)
            }
        });
        
        new_cart_item.base_price - discount_amount
    });

    // Execute query to insert new cart item
    let cart_item = match sqlx::query(
        "INSERT INTO sales_cart (
            user_id, store_id, product_id, base_price, qty, 
            discount_type, discount_value, discount_amount, sale_price, 
            created_at, updated_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, NOW(), NOW())
        RETURNING id, user_id, store_id, product_id, base_price, qty, 
                 discount_type, discount_value, discount_amount, sale_price, 
                 created_at, updated_at"
    )
    .bind(user_id) // Authenticated user ID
    .bind(new_cart_item.store_id)
    .bind(new_cart_item.product_id)
    .bind(&new_cart_item.base_price)
    .bind(new_cart_item.qty)
    .bind(new_cart_item.discount_type.unwrap_or_else(|| "fixed".to_string()))
    .bind(new_cart_item.discount_value.unwrap_or(0))
    .bind(new_cart_item.discount_amount.unwrap_or_else(|| rust_decimal::Decimal::new(0, 0)))
    .bind(sale_price)
    .try_map(|row: sqlx::postgres::PgRow| {
        Ok(SalesCart {
            id: row.try_get("id")?,
            user_id: row.try_get("user_id")?,
            store_id: row.try_get("store_id")?,
            product_id: row.try_get("product_id")?,
            base_price: row.try_get("base_price")?,
            qty: row.try_get("qty")?,
            discount_type: row.try_get("discount_type")?,
            discount_value: row.try_get("discount_value")?,
            discount_amount: row.try_get("discount_amount")?,
            sale_price: row.try_get("sale_price")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    })
    .fetch_one(&pool)
    .await {
        Ok(cart_item) => cart_item,
        Err(e) => {
            error!("Database error while adding to cart: {}", e);
            return Err(ServiceError::DatabaseError(e.to_string()));
        }
    };

    info!("Item added to cart successfully with ID: {}", cart_item.id);
    Ok(cart_item)
}

pub async fn delete_from_cart(
    db_manager: &DbConnectionManager,
    cart_item_id: i32,
    user_id: i32, // User ID from authentication
) -> Result<bool, ServiceError> {
    let pool = match db_manager.get_pool().await {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to get database connection: {:?}", e);
            return Err(ServiceError::DatabaseConnectionError);
        }
    };

    // Execute query to delete the cart item, ensuring it belongs to the authenticated user
    let result = match sqlx::query(
        "DELETE FROM sales_cart 
         WHERE id = $1 AND user_id = $2"
    )
    .bind(cart_item_id)
    .bind(user_id)
    .execute(&pool)
    .await {
        Ok(result) => result,
        Err(e) => {
            error!("Database error while deleting from cart: {}", e);
            return Err(ServiceError::DatabaseError(e.to_string()));
        }
    };

    // Check if any row was affected
    let deleted = result.rows_affected() > 0;
    
    if deleted {
        info!("Successfully deleted cart item with ID: {} for user: {}", cart_item_id, user_id);
    } else {
        info!("No cart item found with ID: {} for user: {}", cart_item_id, user_id);
    }
    
    Ok(deleted)
}

pub async fn get_cart_items(
    db_manager: &DbConnectionManager,
    user_id: i32,
    store_id: i32,
) -> Result<Vec<SalesCartResponse>, ServiceError> {
    let pool = match db_manager.get_pool().await {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to get database connection: {:?}", e);
            return Err(ServiceError::DatabaseConnectionError);
        }
    };

    // Execute query to get all cart items for the user and store with product names
    let cart_items = match sqlx::query(
        "SELECT sc.id, sc.user_id, sc.store_id, sc.product_id, p.name as product_name, p.unit_name,
                sc.base_price, sc.qty, sc.discount_type, sc.discount_value, 
                sc.discount_amount, sc.sale_price, sc.created_at, sc.updated_at 
         FROM sales_cart sc
         INNER JOIN products p ON sc.product_id = p.id
         WHERE sc.user_id = $1 AND sc.store_id = $2 
         ORDER BY sc.created_at DESC"
    )
    .bind(user_id)
    .bind(store_id)
    .try_map(|row: sqlx::postgres::PgRow| {
        Ok(SalesCartResponse {
            id: row.try_get("id")?,
            user_id: row.try_get("user_id")?,
            store_id: row.try_get("store_id")?,
            product_id: row.try_get("product_id")?,
            product_name: row.try_get("product_name")?,
            unit_name: row.try_get("unit_name")?,
            base_price: row.try_get("base_price")?,
            qty: row.try_get("qty")?,
            discount_type: row.try_get("discount_type")?,
            discount_value: row.try_get("discount_value")?,
            discount_amount: row.try_get("discount_amount")?,
            sale_price: row.try_get("sale_price")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    })
    .fetch_all(&pool)
    .await {
        Ok(items) => items,
        Err(e) => {
            error!("Database error while fetching cart items: {}", e);
            return Err(ServiceError::DatabaseError(e.to_string()));
        }
    };

    info!("Retrieved {} cart items for user {} in store {}", cart_items.len(), user_id, store_id);
    Ok(cart_items)
}

pub async fn update_cart_item(
    db_manager: &DbConnectionManager,
    cart_item_id: i32,
    user_id: i32, // User ID from authentication
    update_data: UpdateSalesCart,
) -> Result<SalesCart, ServiceError> {
    let pool = match db_manager.get_pool().await {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to get database connection: {:?}", e);
            return Err(ServiceError::DatabaseConnectionError);
        }
    };

    // First, get the current cart item to make calculations
    let current_item = match sqlx::query_as::<_, SalesCart>(
        "SELECT * FROM sales_cart WHERE id = $1 AND user_id = $2"
    )
    .bind(cart_item_id)
    .bind(user_id)
    .fetch_optional(&pool)
    .await {
        Ok(Some(item)) => item,
        Ok(None) => {
            info!("Cart item not found or not owned by user: {}", user_id);
            return Err(ServiceError::NotFound);
        },
        Err(e) => {
            error!("Database error while fetching cart item: {}", e);
            return Err(ServiceError::DatabaseError(e.to_string()));
        }
    };

    // Prepare update values, using current values if new ones aren't provided
    let base_price = update_data.base_price.unwrap_or(current_item.base_price);
    let qty = update_data.qty.unwrap_or(current_item.qty);
    let discount_type = update_data.discount_type.unwrap_or(current_item.discount_type);
    let discount_value = update_data.discount_value.unwrap_or(current_item.discount_value);

    // Calculate discount_amount and sale_price based on the updated values
    let discount_amount = if discount_type == "percentage" && discount_value > 0 {
        base_price * rust_decimal::Decimal::new(discount_value as i64, 2)
    } else {
        rust_decimal::Decimal::new(discount_value as i64, 0)
    };

    let sale_price = base_price - discount_amount;

    // Execute query to update the cart item
    let updated_item = match sqlx::query_as::<_, SalesCart>(
        "UPDATE sales_cart
         SET base_price = $1, qty = $2, discount_type = $3, discount_value = $4,
             discount_amount = $5, sale_price = $6, updated_at = NOW()
         WHERE id = $7 AND user_id = $8
         RETURNING id, user_id, store_id, product_id, base_price, qty,
                 discount_type, discount_value, discount_amount, sale_price,
                 created_at, updated_at"
    )
    .bind(base_price)
    .bind(qty)
    .bind(discount_type)
    .bind(discount_value)
    .bind(discount_amount)
    .bind(sale_price)
    .bind(cart_item_id)
    .bind(user_id)
    .fetch_one(&pool)
    .await {
        Ok(item) => item,
        Err(e) => {
            error!("Database error while updating cart item: {}", e);
            return Err(ServiceError::DatabaseError(e.to_string()));
        }
    };

    info!("Successfully updated cart item with ID: {} for user: {}", cart_item_id, user_id);
    Ok(updated_item)
}

pub async fn clear_cart(
    db_manager: &DbConnectionManager,
    user_id: i32, // User ID from authentication
    store_id: i32,
) -> Result<bool, ServiceError> {
    let pool = match db_manager.get_pool().await {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to get database connection: {:?}", e);
            return Err(ServiceError::DatabaseConnectionError);
        }
    };

    // Execute query to delete all cart items for this user and store
    let result = match sqlx::query(
        "DELETE FROM sales_cart 
         WHERE user_id = $1 AND store_id = $2"
    )
    .bind(user_id)
    .bind(store_id)
    .execute(&pool)
    .await {
        Ok(result) => result,
        Err(e) => {
            error!("Database error while clearing cart: {}", e);
            return Err(ServiceError::DatabaseError(e.to_string()));
        }
    };

    // Check if any row was affected
    let deleted = result.rows_affected() > 0;
    
    if deleted {
        info!("Successfully cleared cart for user: {} in store: {}", user_id, store_id);
    } else {
        info!("No cart items found for user: {} in store: {}", user_id, store_id);
    }
    
    Ok(deleted)
}

pub async fn create_sales_order(
    db_manager: &DbConnectionManager,
    user_id: i32, // User ID from authentication
    order_request: CreateOrderRequest,
) -> Result<OrderResponse, ServiceError> {
    let pool = match db_manager.get_pool().await {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to get database connection: {:?}", e);
            return Err(ServiceError::DatabaseConnectionError);
        }
    };

    // Start a transaction
    let mut transaction = match pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            error!("Failed to start transaction: {:?}", e);
            return Err(ServiceError::DatabaseError(e.to_string()));
        }
    };

    // 1. Get cart items for the user and store
    let cart_items = match get_cart_items_tx(&mut transaction, user_id, order_request.store_id).await {
        Ok(items) => {
            if items.is_empty() {
                return Err(ServiceError::ValidationError("No items in cart".to_string()));
            }
            items
        },
        Err(e) => return Err(e),
    };

    // 2. Calculate grand total from cart items
    let grand_total = cart_items.iter().fold(Decimal::new(0, 0), |acc, item| {
        acc + (item.sale_price * Decimal::new(item.qty as i64, 0))
    });

    // 3. Calculate receivable (grand_total - payment_cash - payment_non_cash)
    let total_payment = order_request.payment_cash + order_request.payment_non_cash;
    let receivable = if grand_total > total_payment {
        grand_total - total_payment
    } else {
        Decimal::new(0, 0)
    };

    // 4. Insert into sales_orders
    let order = match insert_sales_order(
        &mut transaction, 
        user_id, 
        &order_request, 
        grand_total, 
        receivable
    ).await {
        Ok(order) => order,
        Err(e) => {
            // If there's an error, rollback and return
            if let Err(rollback_err) = transaction.rollback().await {
                error!("Failed to rollback transaction: {:?}", rollback_err);
            }
            return Err(e);
        }
    };

    // 5. Insert order details
    let mut order_details = Vec::new();
    for cart_item in &cart_items {
        let total_price = cart_item.sale_price * Decimal::new(cart_item.qty as i64, 0);
        
        let detail = match insert_sales_order_detail(
            &mut transaction,
            order.id,
            cart_item,
            total_price
        ).await {
            Ok(detail) => detail,
            Err(e) => {
                // If there's an error, rollback and return
                if let Err(rollback_err) = transaction.rollback().await {
                    error!("Failed to rollback transaction: {:?}", rollback_err);
                }
                return Err(e);
            }
        };
        
        order_details.push(detail);
    }

    // 6. Clear the cart
    if let Err(e) = clear_cart_tx(&mut transaction, user_id, order_request.store_id).await {
        // If there's an error, rollback and return
        if let Err(rollback_err) = transaction.rollback().await {
            error!("Failed to rollback transaction: {:?}", rollback_err);
        }
        return Err(e);
    }

    // Commit the transaction
    if let Err(e) = transaction.commit().await {
        error!("Failed to commit transaction: {:?}", e);
        return Err(ServiceError::DatabaseError(e.to_string()));
    }

    info!("Successfully created sales order with ID: {}", order.id);
    
    Ok(OrderResponse {
        order,
        details: order_details,
    })
}

// Helper function to get cart items within a transaction
async fn get_cart_items_tx(
    transaction: &mut Transaction<'_, Postgres>, 
    user_id: i32, 
    store_id: i32
) -> Result<Vec<SalesCart>, ServiceError> {
    let cart_items = match sqlx::query_as::<_, SalesCart>(
        "SELECT id, user_id, store_id, product_id, base_price, qty, 
                discount_type, discount_value, discount_amount, sale_price, 
                created_at, updated_at 
         FROM sales_cart 
         WHERE user_id = $1 AND store_id = $2 
         ORDER BY created_at DESC"
    )
    .bind(user_id)
    .bind(store_id)
    .fetch_all(&mut **transaction)
    .await {
        Ok(items) => items,
        Err(e) => {
            error!("Database error while fetching cart items: {}", e);
            return Err(ServiceError::DatabaseError(e.to_string()));
        }
    };

    info!("Retrieved {} cart items for user {} in store {}", cart_items.len(), user_id, store_id);
    Ok(cart_items)
}

// Helper function to insert into sales_orders within a transaction
async fn insert_sales_order(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: i32,
    order_request: &CreateOrderRequest,
    grand_total: Decimal,
    receivable: Decimal,
) -> Result<SalesOrder, ServiceError> {
    // Use the provided date or default to today
    let date = order_request.date.unwrap_or_else(|| chrono::Local::now().date_naive());

    let order = match sqlx::query_as::<_, SalesOrder>(
        "INSERT INTO sales_orders (
            order_number, user_id, store_id, date, grand_total, 
            payment_cash, payment_non_cash, receivable, created_at, customer_id
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW(), $9)
        RETURNING id, order_number, user_id, store_id, date, grand_total, 
                 payment_cash, payment_non_cash, receivable, created_at, customer_id"
    )
    .bind(&order_request.order_number)
    .bind(user_id)
    .bind(order_request.store_id)
    .bind(date)
    .bind(grand_total)
    .bind(order_request.payment_cash)
    .bind(order_request.payment_non_cash)
    .bind(receivable)
    .bind(order_request.customer_id)
    .fetch_one(&mut **transaction)
    .await {
        Ok(order) => order,
        Err(e) => {
            error!("Database error while creating sales order: {}", e);
            return Err(ServiceError::DatabaseError(e.to_string()));
        }
    };

    info!("Created sales order with ID: {}", order.id);
    Ok(order)
}

// Helper function to insert into sales_order_details within a transaction
async fn insert_sales_order_detail(
    transaction: &mut Transaction<'_, Postgres>,
    order_id: i32,
    cart_item: &SalesCart,
    total_price: Decimal,
) -> Result<SalesOrderDetail, ServiceError> {
    let detail = match sqlx::query_as::<_, SalesOrderDetail>(
        "INSERT INTO sales_order_details (
            order_id, product_id, qty, base_price, 
            discount_type, discount_value, discount_amount, sale_price, total_price
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING id, order_id, product_id, qty, base_price, 
                 discount_type, discount_value, discount_amount, sale_price, total_price"
    )
    .bind(order_id)
    .bind(cart_item.product_id)
    .bind(cart_item.qty)
    .bind(cart_item.base_price)
    .bind(&cart_item.discount_type)
    .bind(Decimal::new(cart_item.discount_value as i64, 0)) // Convert i32 to Decimal
    .bind(cart_item.discount_amount)
    .bind(cart_item.sale_price)
    .bind(total_price)
    .fetch_one(&mut **transaction)
    .await {
        Ok(detail) => detail,
        Err(e) => {
            error!("Database error while creating sales order detail: {}", e);
            return Err(ServiceError::DatabaseError(e.to_string()));
        }
    };

    info!("Created sales order detail with ID: {}", detail.id);
    Ok(detail)
}

// Helper function to clear cart within a transaction
async fn clear_cart_tx(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: i32,
    store_id: i32,
) -> Result<bool, ServiceError> {
    let result = match sqlx::query(
        "DELETE FROM sales_cart WHERE user_id = $1 AND store_id = $2"
    )
    .bind(user_id)
    .bind(store_id)
    .execute(&mut **transaction)
    .await {
        Ok(result) => result,
        Err(e) => {
            error!("Database error while clearing cart: {}", e);
            return Err(ServiceError::DatabaseError(e.to_string()));
        }
    };

    let deleted = result.rows_affected() > 0;
    if deleted {
        info!("Successfully cleared cart for user: {} in store: {}", user_id, store_id);
    } else {
        info!("No cart items found for user: {} in store: {}", user_id, store_id);
    }
    
    Ok(deleted)
}

pub async fn generate_sales_report(
    db_manager: &DbConnectionManager,
    _user_id: i32,
    company_id: i32,
    query: SalesReportQuery,
) -> Result<SalesReport, ServiceError> {
    let pool = match db_manager.get_pool().await {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to get database connection: {:?}", e);
            return Err(ServiceError::DatabaseConnectionError);
        }
    };

    // 1. Get orders based on date range and store_id
    let mut orders_query_builder = String::from(
        "SELECT so.id, so.order_number, so.user_id, u.initial as user_initial, 
        so.store_id, s.initial as store_initial, so.date, so.grand_total, 
        so.payment_cash, so.payment_non_cash, so.receivable, so.created_at, so.customer_id
        FROM sales_orders so
        JOIN users u ON so.user_id = u.id
        JOIN stores s ON so.store_id = s.id
        WHERE so.date BETWEEN $1 AND $2
        AND u.company_id = $3"
    );

    // Add store filter if specified (not 0)
    if query.store_id > 0 {
        orders_query_builder.push_str(" AND so.store_id = $4");
    }
    
    orders_query_builder.push_str(" ORDER BY so.date DESC, so.id DESC");

    // Create the query
    let mut query_builder = sqlx::query_as::<_, SalesReportOrder>(&orders_query_builder);
    
    // Bind parameters
    query_builder = query_builder.bind(query.start_date);
    query_builder = query_builder.bind(query.end_date);
    query_builder = query_builder.bind(company_id);
    
    // Add store binding if specified
    if query.store_id > 0 {
        query_builder = query_builder.bind(query.store_id);
    }
    
    let orders = match query_builder.fetch_all(&pool).await {
        Ok(mut orders) => {
            // For each order, we need to populate the items
            for order in &mut orders {
                let items = match sqlx::query_as::<_, SalesReportOrderItem>(
                    "SELECT sod.id, sod.order_id, sod.product_id, p.name as product_name, p.sku,
                            sod.qty, sod.base_price, sod.discount_type, sod.discount_value,
                            sod.discount_amount, sod.sale_price, sod.total_price
                     FROM sales_order_details sod
                     JOIN products p ON sod.product_id = p.id
                     WHERE sod.order_id = $1
                     ORDER BY sod.id"
                )
                .bind(order.id)
                .fetch_all(&pool)
                .await {
                    Ok(items) => items,
                    Err(e) => {
                        error!("Error fetching order items for order {}: {:?}", order.id, e);
                        return Err(ServiceError::DatabaseError(e.to_string()));
                    }
                };
                order.items = items;
            }
            orders
        },
        Err(e) => {
            error!("Error fetching orders for report: {:?}", e);
            return Err(ServiceError::DatabaseError(e.to_string()));
        }
    };

    // Get the order IDs for SKU summary
    let order_ids: Vec<i32> = orders.iter().map(|o| o.id).collect();
    
    if order_ids.is_empty() {
        // If no orders found, return an empty report
        return Ok(SalesReport {
            orders: vec![],
            sku_summary: vec![],
            summary: SalesSummary {
                total_payment_cash: Decimal::new(0, 0),
                total_payment_non_cash: Decimal::new(0, 0),
                total_receivable: Decimal::new(0, 0),
                total_orders: 0,
            },
        });
    }

    // 2. Get SKU summary for the selected orders
    let sku_summary = match sqlx::query_as::<_, SkuSummaryItem>(
        "SELECT sod.product_id, 
                p.name as product_name, 
                p.sku, 
                SUM(sod.qty) as total_qty, 
                SUM(sod.total_price) as total_price 
         FROM sales_order_details sod
         JOIN products p ON sod.product_id = p.id
         WHERE sod.order_id = ANY($1)
         GROUP BY sod.product_id, p.name, p.sku
         ORDER BY total_qty DESC"
    )
    .bind(&order_ids)
    .fetch_all(&pool)
    .await {
        Ok(summary) => summary,
        Err(e) => {
            error!("Error fetching SKU summary: {:?}", e);
            return Err(ServiceError::DatabaseError(e.to_string()));
        }
    };

    // 3. Calculate the total summary
    let summary = SalesSummary {
        total_payment_cash: orders.iter().fold(Decimal::new(0, 0), |acc, order| acc + order.payment_cash),
        total_payment_non_cash: orders.iter().fold(Decimal::new(0, 0), |acc, order| acc + order.payment_non_cash),
        total_receivable: orders.iter().fold(Decimal::new(0, 0), |acc, order| acc + order.receivable),
        total_orders: orders.len() as i32,
    };

    // Return the complete sales report
    Ok(SalesReport {
        orders,
        sku_summary,
        summary,
    })
}

pub async fn get_sales_order_by_id(
    db_manager: &DbConnectionManager,
    order_id: i32,
    requester_company_id: i32, // Authenticated user's company_id
) -> Result<DetailedOrderResponse, ServiceError> {
    let pool = match db_manager.get_pool().await {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to get database connection: {:?}", e);
            return Err(ServiceError::DatabaseConnectionError);
        }
    };

    // Fetch order and creator's company_id
    let order_row = sqlx::query_as::<_, OrderQueryResult>(
        r#"
        SELECT so.id, so.order_number, so.user_id, u.initial as user_initial, 
               so.store_id, s.initial as store_initial, so.date, so.grand_total, 
               so.payment_cash, so.payment_non_cash, so.receivable, so.created_at, so.customer_id,
               u.company_id as creator_company_id
        FROM sales_orders so
        JOIN users u ON so.user_id = u.id
        JOIN stores s ON so.store_id = s.id
        WHERE so.id = $1
        "#,
    )
    .bind(order_id)
    .fetch_optional(&pool)
    .await;
    let order_row = match order_row {
        Ok(Some(row)) => row,
        Ok(None) => {
            info!("Sales order ID {} not found", order_id);
            return Err(ServiceError::NotFound);
        },
        Err(e) => {
            error!("Database error while fetching sales order: {}", e);
            return Err(ServiceError::DatabaseError(e.to_string()));
        }
    };

    // Check company_id
    let creator_company_id = order_row.creator_company_id;
    if creator_company_id != requester_company_id {
        info!("User with company_id {} tried to access order {} from company_id {}", requester_company_id, order_id, creator_company_id);
        return Err(ServiceError::NotFound);
    }

    // Map order_row to DetailedSalesOrder
    let order = DetailedSalesOrder {
        id: order_row.id,
        order_number: order_row.order_number,
        user_id: order_row.user_id,
        user_initial: order_row.user_initial,
        store_id: order_row.store_id,
        store_initial: order_row.store_initial,
        date: order_row.date,
        grand_total: order_row.grand_total,
        payment_cash: order_row.payment_cash,
        payment_non_cash: order_row.payment_non_cash,
        receivable: order_row.receivable,
        created_at: order_row.created_at,
        customer_id: order_row.customer_id,
    };

    // Now, get all the details with product information
    let details = match sqlx::query_as::<_, DetailedSalesOrderDetail>(
        "SELECT sod.id, sod.order_id, sod.product_id, p.name as product_name, p.sku, 
                sod.qty, sod.base_price, sod.discount_type, sod.discount_value, 
                sod.discount_amount, sod.sale_price, sod.total_price
         FROM sales_order_details sod
         JOIN products p ON sod.product_id = p.id
         WHERE sod.order_id = $1
         ORDER BY sod.id"
    )
    .bind(order_id)
    .fetch_all(&pool)
    .await {
        Ok(details) => details,
        Err(e) => {
            error!("Database error while fetching order details: {}", e);
            return Err(ServiceError::DatabaseError(e.to_string()));
        }
    };

    info!("Successfully retrieved sales order ID: {} with {} detail items", order_id, details.len());
    
    Ok(DetailedOrderResponse {
        order,
        details,
    })
}
