use log::error;
use sqlx::PgPool;

use crate::errors::ServiceError;
use crate::models::user::{Store, User, UserWithStores};

pub async fn get_user_with_stores(
    pool: &PgPool,
    email: String,
) -> Result<UserWithStores, ServiceError> {
    // Get the user by email
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT u.id, u.email, u.company_id, u.full_name, u.initial, 
               u.created_at, u.updated_at, c.name as company_name
        FROM users u
        JOIN companies c ON u.company_id = c.id
        WHERE u.email = $1
        "#,
        email
    )
    .fetch_one(pool)
    .await
    .map_err(|e| {
        error!("Database error fetching user: {:?}", e);
        ServiceError::NotFound(format!("User not found: {}", e))
    })?;

    // Get store IDs for this user
    let store_ids = sqlx::query!(
        r#"
        SELECT store_id 
        FROM user_stores 
        WHERE user_id = $1
        "#,
        user.id
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        error!("Database error fetching store IDs: {:?}", e);
        ServiceError::InternalServerError(format!("Failed to load user stores: {}", e))
    })?
    .into_iter()
    .map(|record| record.store_id)
    .collect::<Vec<i32>>();

    if store_ids.is_empty() {
        return Ok(UserWithStores::from_user_and_stores(user, vec![]));
    }

    // Get store details
    let user_stores = sqlx::query_as!(
        Store,
        r#"
        SELECT id, name, company_id, initial, created_at, updated_at
        FROM stores 
        WHERE id = ANY($1)
        "#,
        &store_ids[..]
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        error!("Database error fetching stores: {:?}", e);
        ServiceError::InternalServerError(format!("Failed to load stores: {}", e))
    })?;

    Ok(UserWithStores::from_user_and_stores(user, user_stores))
}
