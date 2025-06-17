use crate::errors::ServiceError;
use crate::models::sales::{SalesCart, NewSalesCart};
use crate::services::db_service::DbConnectionManager;
use log::{error, info};
use sqlx::Row;

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
) -> Result<Vec<SalesCart>, ServiceError> {
    let pool = match db_manager.get_pool().await {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to get database connection: {:?}", e);
            return Err(ServiceError::DatabaseConnectionError);
        }
    };

    // Execute query to get all cart items for the user and store
    let cart_items = match sqlx::query(
        "SELECT id, user_id, store_id, product_id, base_price, qty, 
                discount_type, discount_value, discount_amount, sale_price, 
                created_at, updated_at 
         FROM sales_cart 
         WHERE user_id = $1 AND store_id = $2 
         ORDER BY created_at DESC"
    )
    .bind(user_id)
    .bind(store_id)
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
