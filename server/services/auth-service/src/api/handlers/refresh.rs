use axum::{
    http::StatusCode,
    Json,
};
use crate::services::token_service::refresh;
use crate::models::login::{RefreshRequest, TokenResponse};
use std::sync::Arc;
use crate::core::app_state::AppState;
use axum::extract::State;

pub async fn register_handler(State(state): State<Arc<AppState>>,
    Json(request): Json<RefreshRequest>) -> Result<Json<TokenResponse>, StatusCode> {
    return refresh(State(state), Json(request)).await
}