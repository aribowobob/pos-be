use log::{error, warn};
use sqlx::Error as SqlxError;
use std::error::Error;

/// Helper function to handle database errors with appropriate logging and response
#[allow(dead_code)]
pub fn handle_db_error(err: &SqlxError) -> (bool, String) {
    let error_message = err.to_string();
    
    // Check for common benign errors
    if error_message.contains("already exists") || 
       error_message.contains("duplicate key") {
        warn!("Non-fatal database error: {}", error_message);
        return (false, format!("Constraint already satisfied: {}", error_message));
    }
    
    // Check for foreign key violations
    if error_message.contains("foreign key constraint") {
        error!("Foreign key constraint violation: {}", error_message);
        return (true, format!("Foreign key constraint violation: {}", error_message));
    }
    
    // Check for syntax errors
    if error_message.contains("syntax error") {
        error!("SQL syntax error: {}", error_message);
        return (true, format!("SQL syntax error: {}", error_message));
    }
    
    // Default case for other errors
    error!("Database error: {}", error_message);
    (true, format!("Database operation failed: {}", error_message))
}

/// Helper function to log detailed error information
#[allow(dead_code)]
pub fn log_detailed_error<E: Error>(context: &str, err: &E) {
    error!("{} - Error type: {}", context, std::any::type_name::<E>());
    
    let mut source_opt = err.source();
    let mut depth = 0;
    
    while let Some(source) = source_opt {
        error!("Cause {}: {}", depth, source);
        source_opt = source.source();
        depth += 1;
    }
}
