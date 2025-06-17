
use axum::{
    http::StatusCode,
    Json,
};
use crate::services::auth_service;
use crate::models::login::RefreshRequest;


pub async fn register_handler(token:String) -> Result<Json<RefreshRequest>, StatusCode> {
    Err(StatusCode::UNAUTHORIZED)
}