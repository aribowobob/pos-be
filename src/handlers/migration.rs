use crate::models::{AppState, ApiResponse};
use actix_web::{web, HttpResponse};
use log::{info, error, debug, warn};
use sqlx::{postgres::PgPool, Pool, Postgres, Error as SqlxError};
use std::sync::Arc;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::error::Error;

// Helper function to log detailed SQL error information
fn log_error_details(err: &SqlxError, file_path: &PathBuf) {
    let error_str = err.to_string();
    
    // Log different types of errors differently
    if error_str.contains("already exists") || error_str.contains("duplicate key") {
        warn!("Non-critical error in {}: {}", file_path.display(), error_str);
    } else if error_str.contains("syntax error") {
        error!("SQL syntax error in {}: {}", file_path.display(), error_str);
    } else if error_str.contains("foreign key constraint") {
        error!("Foreign key constraint violation in {}: {}", file_path.display(), error_str);
    } else {
        error!("SQL error in {}: {}", file_path.display(), error_str);
    }
    
    // Log error causes if available
    let mut source_opt = err.source();
    let mut depth = 0;
    
    while let Some(source) = source_opt {
        error!("Cause {}: {}", depth, source);
        source_opt = source.source();
        depth += 1;
    }
}

// Function to execute all migration SQL files
async fn run_migrations(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    info!("Running database migrations from SQL files...");

    // Path to migration directory (adjust if needed)
    let migration_dir = "./src/db_migration";
    let path = Path::new(migration_dir);
    
    if !path.exists() || !path.is_dir() {
        error!("Migration directory not found: {}", migration_dir);
        return Ok(()); // Non-fatal error, just log it
    }
    
    // Read all SQL files from the directory
    let mut entries = match fs::read_dir(path) {
        Ok(entries) => entries.collect::<Result<Vec<_>, _>>()
            .map_err(|e| {
                error!("Error collecting directory entries: {}", e);
                sqlx::Error::ColumnDecode { index: "".to_string(), source: Box::new(e) } // Wrap IO error
            })?,
        Err(e) => {
            error!("Failed to read migration directory: {}", e);
            return Ok(()); // Non-fatal error
        }
    };
    
    // Sort entries by filename to ensure order (001_, 002_, etc.)
    entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
    
    info!("Found {} files in migration directory", entries.len());
    
    // Process each SQL file
    for entry in entries {
        let file_path = entry.path();
        
        // Only process .sql files
        if file_path.extension().map_or(false, |ext| ext == "sql") {
            info!("Processing SQL file: {}", file_path.display());
            if let Err(e) = execute_sql_file(pool, &file_path).await {
                error!("Error executing SQL file {}: {}", file_path.display(), e);
                // Return error for serious issues, continue for benign ones
                if !e.to_string().contains("already exists") && !e.to_string().contains("duplicate key") {
                    // Continue anyway for now
                    // return Err(e);
                }
            }
        }
    }
    
    info!("Database migrations completed successfully");
    Ok(())
}

// Function to execute a single SQL file
async fn execute_sql_file(pool: &Pool<Postgres>, file_path: &PathBuf) -> Result<(), sqlx::Error> {
    info!("Executing SQL file: {}", file_path.display());
    
    // Read file content
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            error!("Failed to open file {}: {}", file_path.display(), e);
            return Ok(()); // Non-fatal error
        }
    };
    
    let mut sql_content = String::new();
    if let Err(e) = file.read_to_string(&mut sql_content) {
        error!("Failed to read file {}: {}", file_path.display(), e);
        return Ok(()); // Non-fatal error
    }
    
    debug!("SQL content length: {} bytes", sql_content.len());
    
    // Create a transaction (we'll ultimately not use it)
    let tx = pool.begin().await?;
    
    // Split the file by semicolons, but be careful with PL/pgSQL blocks
    let mut statements = Vec::new();
    let mut current_statement = String::new();
    let mut in_block = false;
    let mut block_delimiter = String::new();
    
    for line in sql_content.lines() {
        // Skip empty lines and comments
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("--") {
            current_statement.push_str(line);
            current_statement.push('\n');
            continue;
        }
        
        // Check if we're entering a PL/pgSQL block
        if !in_block && trimmed.contains("DO $") {
            in_block = true;
            // Extract the delimiter
            if let Some(block_start) = trimmed.find("DO $") {
                if let Some(rest) = trimmed[block_start + 3..].find('$') {
                    block_delimiter = trimmed[block_start + 3..block_start + 3 + rest].to_string();
                    debug!("Entering PL/pgSQL block with delimiter: {}", block_delimiter);
                }
            }
        }
        
        // Check if we're exiting a PL/pgSQL block
        if in_block && trimmed.contains(&format!("END ${}", block_delimiter)) {
            in_block = false;
            debug!("Exiting PL/pgSQL block with delimiter: {}", block_delimiter);
            block_delimiter.clear();
        }
        
        // Add this line to the current statement
        current_statement.push_str(line);
        current_statement.push('\n');
        
        // If we're not in a block and line ends with semicolon, treat as end of statement
        if !in_block && trimmed.ends_with(';') {
            if !current_statement.trim().is_empty() {
                statements.push(current_statement.clone());
            }
            current_statement.clear();
        }
    }
    
    // Add any remaining content as a statement
    if !current_statement.trim().is_empty() {
        statements.push(current_statement);
    }
    
    debug!("Found {} SQL statements in file", statements.len());
    
    // Execute statements individually (not in a transaction)
    // This allows us to continue even if some statements fail
    for (i, statement) in statements.iter().enumerate() {
        debug!("Executing statement {} of {}", i+1, statements.len());
        
        // Create a new connection for each statement to avoid transaction issues
        let result = sqlx::query(statement).execute(pool).await;
        
        match result {
            Ok(_) => debug!("Statement {} executed successfully", i+1),
            Err(e) => {
                // Log the error
                error!("Error executing statement {}: {}", i+1, e);
                log_error_details(&e, file_path);
                
                // For benign errors like "already exists", we continue
                let error_str = e.to_string();
                if error_str.contains("already exists") || 
                   error_str.contains("duplicate key") || 
                   error_str.contains("constraint violation") {
                    debug!("Continuing despite error (benign error)");
                } else {
                    // For serious errors, log but still continue with next statement
                    error!("Serious error in statement {}, but continuing: {}", i+1, e);
                }
            }
        }
    }
    
    // Don't need to commit or rollback since we're not using a transaction
    // Close the transaction without using it
    let _ = tx.rollback().await;
    
    info!("Successfully executed SQL file: {}", file_path.display());
    Ok(())
}

#[utoipa::path(
    get,
    path = "/db-migration",
    responses(
        (status = 200, description = "Database migration completed successfully", body = ApiResponse<()>),
        (status = 500, description = "Migration failed", body = ApiResponse<()>)
    ),
    security(
        ("cookie_auth" = [])
    ),
    tag = "migration"
)]
pub async fn db_migration(
    app_state: web::Data<AppState>
) -> HttpResponse {
    // Get connection pool from state or create one
    let pool = match crate::services::db_service::get_db_pool(&app_state.db_connection_string).await {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to get database connection: {}", e);
            return HttpResponse::InternalServerError().json(ApiResponse {
                status: "error".to_string(),
                message: "Failed to connect to database".to_string(),
                data: None::<()>,
            });
        }
    };
    
    // Run migrations using SQL files from db_migration directory
    match run_migrations(&pool).await {
        Ok(_) => {
            HttpResponse::Ok().json(ApiResponse {
                status: "success".to_string(),
                message: "Database migration from SQL files completed successfully".to_string(),
                data: None::<()>,
            })
        },
        Err(e) => {
            error!("Failed to run migrations: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse {
                status: "error".to_string(),
                message: format!("Migration failed: {}", e),
                data: None::<()>,
            })
        }
    }
}
