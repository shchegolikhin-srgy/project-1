use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub role:String
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RefreshTokenData{
    pub refresh_token:String,
    pub username:String,
}
#[derive(sqlx::FromRow, Clone)]
pub struct  RefreshToken{
    pub refresh_token:String,
}