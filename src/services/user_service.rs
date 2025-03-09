use diesel::prelude::*;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};

// Update import to use crate::db
use crate::db::schema::{stores, user_stores, users};
use crate::errors::ServiceError;
use crate::models::user::{Store, User, UserStore, UserWithStores};

pub fn get_user_with_stores(
    conn: &PgConnection,
    user_id: i32,
) -> Result<UserWithStores, ServiceError> {
    // Get the user
    let user = users::table
        .find(user_id)
        .first::<User>(conn)
        .map_err(|_| ServiceError::NotFound("User not found".into()))?;

    // Get store IDs for this user
    let store_ids: Vec<i32> = user_stores::table
        .filter(user_stores::user_id.eq(user_id))
        .select(user_stores::store_id)
        .load::<i32>(conn)
        .map_err(|_| ServiceError::InternalServerError("Failed to load user stores".into()))?;

    // Get store details
    let user_stores: Vec<Store> = stores::table
        .filter(stores::id.eq_any(store_ids))
        .load::<Store>(conn)
        .map_err(|_| ServiceError::InternalServerError("Failed to load stores".into()))?;

    Ok(UserWithStores::from_user_and_stores(user, user_stores))
}
