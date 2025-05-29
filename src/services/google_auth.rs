use crate::models::user::UserInfo;
use log::{debug, error, info, warn};
use std::env;

/// Verifies a Google OAuth access token and returns user information
pub async fn verify_google_token(token: &str) -> Result<UserInfo, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    
    // Get client ID from environment (for validation)
    let client_id = env::var("GOOGLE_CLIENT_ID").unwrap_or_else(|_| {
        warn!("GOOGLE_CLIENT_ID not set in environment");
        String::new()
    });

    info!("Verifying Google access token");
    
    // Fetch user info from Google
    match client
        .get("https://www.googleapis.com/oauth2/v3/userinfo")
        .bearer_auth(token)
        .send()
        .await
    {
        Ok(response) => {
            if !response.status().is_success() {
                let error_text = response.text().await?;
                error!("Google API error: {}", error_text);
                return Err(format!("Failed to verify token: {error_text}").into());
            }

            let response_text = response.text().await?;
            debug!("Google API response: {}", response_text);

            // Parse user info from response
            match serde_json::from_str::<serde_json::Value>(&response_text) {
                Ok(json_data) => {
                    // Validate token audience if client_id is set
                    if !client_id.is_empty() {
                        if let Some(aud) = json_data.get("aud").and_then(|v| v.as_str()) {
                            if aud != client_id {
                                error!("Token audience mismatch: expected {}, got {}", client_id, aud);
                                return Err("Invalid token audience".into());
                            }
                        } else {
                            warn!("Could not verify token audience - 'aud' claim missing");
                        }
                    }
                    
                    // Extract user info from JSON
                    let email = match json_data.get("email").and_then(|v| v.as_str()) {
                        Some(email) => email.to_string(),
                        None => {
                            error!("Email missing from Google response");
                            return Err("Email missing from token info".into());
                        }
                    };
                    
                    let name = json_data.get("name")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                        
                    let sub = json_data.get("sub")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                        .unwrap_or_default();
                        
                    let picture = json_data.get("picture")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    
                    // Get the full name from name or empty string
                    let full_name = name.clone().unwrap_or_default();
                    
                    // Create user info object
                    let user_info = UserInfo {
                        id: 0, // Will be populated from database if user exists
                        email,
                        name,
                        picture,
                        sub,
                        full_name,
                        company_id: 0, // Will be populated from database if user exists
                    };
                    
                    info!("Google authentication successful for {}", user_info.email);
                    Ok(user_info)
                },
                Err(e) => {
                    error!("Failed to parse Google response: {}", e);
                    Err(Box::new(e))
                }
            }
        },
        Err(e) => {
            error!("Request to Google API failed: {}", e);
            Err(Box::new(e))
        }
    }
}
