// file: /Users/catalyst/Documents/playground/pos-be/src/services/product_service.rs
use crate::errors::ServiceError;
use crate::models::product::{ProductCategory, PaginatedResponse};
use crate::services::db_service::DbConnectionManager;
use sqlx::postgres::PgPool;
use sqlx::Row;
use log::{error, info};

pub async fn get_product_categories(
    db_manager: &DbConnectionManager,
    search: Option<String>,
    page: Option<i32>,
    size: Option<i32>,
) -> Result<PaginatedResponse<ProductCategory>, ServiceError> {
    let pool = match db_manager.get_pool().await {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to get database connection: {:?}", e);
            return Err(ServiceError::DatabaseConnectionError);
        }
    };

    // Default pagination values
    let page = page.unwrap_or(1);
    let size = size.unwrap_or(10);
    let offset = (page - 1) * size;

    // Build the query based on whether search is provided
    let mut count_query = String::from("SELECT COUNT(*) FROM product_categories");
    let mut query = String::from("SELECT id, name, description, parent_id, created_at, updated_at FROM product_categories");
    
    // Add search condition if provided
    if let Some(search_term) = &search {
        let where_clause = format!(" WHERE name ILIKE '%{}%'", search_term.replace('\'', "''"));
        count_query.push_str(&where_clause);
        query.push_str(&where_clause);
    }
    
    // Add order by and pagination
    query.push_str(" ORDER BY name ASC LIMIT $1 OFFSET $2");
    
    // Get total count
    let total: i64 = match sqlx::query_scalar(&count_query)
        .fetch_one(&pool)
        .await
    {
        Ok(count) => count,
        Err(e) => {
            error!("Database error while counting product categories: {}", e);
            return Err(ServiceError::DatabaseError(e.to_string()));
        }
    };
    
    // Get paginated results
    // Use query_builder instead of query_as because we have a dynamic query string
    let rows = match sqlx::query(&query)
        .bind(size)
        .bind(offset)
        .try_map(|row: sqlx::postgres::PgRow| {
            Ok(ProductCategory {
                id: row.try_get("id")?,
                name: row.try_get("name")?,
                description: row.try_get("description")?,
                parent_id: row.try_get("parent_id")?,
                created_at: row.try_get("created_at")?,
                updated_at: row.try_get("updated_at")?,
            })
        })
        .fetch_all(&pool)
        .await
    {
        Ok(rows) => rows,
        Err(e) => {
            error!("Database error while fetching product categories: {}", e);
            return Err(ServiceError::DatabaseError(e.to_string()));
        }
    };
    
    info!("Retrieved {} product categories", rows.len());
    let paginated = PaginatedResponse::new(page, size, total, rows);
    
    Ok(paginated)
}
