use log::{error, info};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

use crate::errors::ServiceError;

// Use Arc<Mutex<>> for thread-safe lazy initialization
pub struct DbConnectionManager {
    connection_string: String,
    pool: Arc<Mutex<Option<PgPool>>>,
}

impl DbConnectionManager {
    pub fn new(connection_string: String) -> Self {
        DbConnectionManager {
            connection_string,
            pool: Arc::new(Mutex::new(None)),
        }
    }

    // Get a connection pool, creating it if it doesn't exist
    pub async fn get_pool(&self) -> Result<PgPool, ServiceError> {
        let mut pool_lock = self.pool.lock().await;

        // If pool exists and is still valid, return it
        if let Some(pool) = &*pool_lock {
            if let Ok(_) = pool.acquire().await {
                return Ok(pool.clone());
            }
            info!("Existing database connection is no longer valid, recreating...");
        }

        info!("Creating new database connection pool...");
        info!("Connection string host: {}", 
            if self.connection_string.contains("@db:") { "db (docker service)" } 
            else if self.connection_string.contains("@localhost:") { "localhost" }
            else { "unknown" }
        );

        // Create a new pool with longer timeouts for Docker environment
        match PgPoolOptions::new()
            .max_connections(10)
            .min_connections(1)
            .acquire_timeout(Duration::from_secs(30))  // Increased timeout
            .idle_timeout(Duration::from_secs(600))    // 10 minutes idle timeout
            .max_lifetime(Duration::from_secs(1800))   // 30 minutes max lifetime
            .connect_timeout(Duration::from_secs(30))  // 30 seconds to establish connection
            .connect(&self.connection_string)
            .await
        {
            Ok(new_pool) => {
                info!("Successfully established database connection pool");
                
                // Test the connection immediately
                match new_pool.acquire().await {
                    Ok(_conn) => {
                        info!("Connection pool test successful");
                        *pool_lock = Some(new_pool.clone());
                        Ok(new_pool)
                    },
                    Err(e) => {
                        error!("Failed to acquire connection from new pool: {}", e);
                        Err(ServiceError::DatabaseConnectionError)
                    }
                }
            }
            Err(e) => {
                error!("Failed to create database connection pool: {}", e);
                error!("Connection string format check: {}", 
                    if self.connection_string.starts_with("postgres://") { "valid postgres:// prefix" }
                    else { "invalid connection string format" }
                );
                Err(ServiceError::DatabaseConnectionError)
            }
        }
    }
}

// Helper function to get a database pool
pub async fn get_db_pool(connection_string: &str) -> Result<PgPool, ServiceError> {
    let manager = DbConnectionManager::new(connection_string.to_string());
    manager.get_pool().await
}
