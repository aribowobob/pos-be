use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub sub: String,          // Google's unique identifier
    pub name: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub picture: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub locale: Option<String>,
}
