use log::{error, info, debug};
use sqlx::postgres::PgRow;
use sqlx::Row;
use chrono::{DateTime, Utc, NaiveDateTime};

use crate::errors::ServiceError;
use crate::models::user::{Store, User, UserWithStores};
use crate::services::db_service::DbConnectionManager;

pub async fn get_user_with_stores(
    db_manager: &DbConnectionManager,
    email: String,
) -> Result<UserWithStores, ServiceError> {
    // Connect to database on-demand
    let pool = db_manager.get_pool().await?;
    
    debug!("Executing query to find user with email: {}", email);
    
    // Use explicit column types and conversion for timestamps - but don't include them in result
    let user_row = sqlx::query(
        "SELECT u.id, u.email, u.company_id, u.full_name, u.initial, c.name as company_name
         FROM users u
         LEFT JOIN companies c ON u.company_id = c.id
         WHERE u.email = $1",
    )
    .bind(&email)
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        error!("Database error when fetching user {}: {}", email, e);
        ServiceError::DatabaseQueryError(e.to_string())
    })?;
    
    if let Some(row) = user_row {
        // Manually map the row to User with the correct types - handle errors explicitly
        let user_id = row.try_get::<i32, _>("id").map_err(|e| {
            error!("Failed to extract user id: {}", e);
            ServiceError::DatabaseQueryError(e.to_string())
        })?;
        
        let user_email = row.try_get::<String, _>("email").map_err(|e| {
            error!("Failed to extract user email: {}", e);
            ServiceError::DatabaseQueryError(e.to_string())
        })?;
        
        let company_id = row.try_get::<i32, _>("company_id").map_err(|e| {
            error!("Failed to extract company_id: {}", e);
            ServiceError::DatabaseQueryError(e.to_string())
        })?;
        
        let company_name = row.try_get::<Option<String>, _>("company_name").map_err(|e| {
            error!("Failed to extract company_name: {}", e);
            ServiceError::DatabaseQueryError(e.to_string())
        })?;
        
        let full_name = row.try_get::<String, _>("full_name").map_err(|e| {
            error!("Failed to extract full_name: {}", e);
            ServiceError::DatabaseQueryError(e.to_string())
        })?;
        
        let initial = row.try_get::<String, _>("initial").map_err(|e| {
            error!("Failed to extract initial: {}", e);
            ServiceError::DatabaseQueryError(e.to_string())
        })?;
        
        info!("Found user: {} with id: {}", user_email, user_id);
        
        // Fetch stores for user - removed timestamp fields
        info!("Executing query to find stores for user: {}", user_id);
        
        let stores_rows = sqlx::query(
            r#"SELECT 
                  s.id::int4 as id, 
                  s.name, 
                  s.company_id,
                  s.initial
               FROM stores s 
               JOIN user_stores us ON s.id = us.store_id 
               WHERE us.user_id = $1"#
        )
        .bind(user_id)
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            error!("Error fetching stores for user {}: {}", user_email, e);
            ServiceError::DatabaseQueryError(e.to_string())
        })?;
        
        // Manually map rows to Store objects with the correct types - removed timestamp handling
        let stores = stores_rows
            .iter()
            .map(|row: &PgRow| {
                Store {
                    id: row.get::<i32, _>("id"),
                    name: row.get("name"),
                    company_id: row.get("company_id"),
                    initial: row.get("initial"),
                }
            })
            .collect();
        
        info!("Successfully fetched user {} with {} stores", user_email, stores_rows.len());
        
        // Create UserWithStores by directly populating all fields - removed timestamp fields
        Ok(UserWithStores {
            id: user_id,
            email: user_email,
            company_id,
            company_name,
            full_name,
            initial,
            stores
        })
    } else {
        info!("User with email {} not found", email);
        Err(ServiceError::NotFound)
    }
}
