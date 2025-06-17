
use axum::{
    http::StatusCode,
    Json,
};

use crate::models::login::{LoginRequest, AuthResponse};


pub async fn register_handler(token:String) -> Result<Json<AuthResponse>, StatusCode> {
    Err(StatusCode::UNAUTHORIZED)
}