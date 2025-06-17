use axum::{
    http::StatusCode,
    Json,
};
use crate::models::login::{RegisterRequest, AuthResponse};
use std::sync::Arc;
use crate::core::app_state::AppState;
use axum::extract::State;
use crate::services::register_service;

pub async fn register_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    Err(StatusCode::UNAUTHORIZED)
}

pub async fn register_by_role_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    Err(StatusCode::UNAUTHORIZED)
}