use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct GoogleTokenInfo {
    pub email: String,
    pub name: String,
    pub picture: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TokenRequest {
    #[serde(rename = "token")]
    #[schema(example = "ya29.a0ARrdaM9...")]
    pub access_token: String,
}
