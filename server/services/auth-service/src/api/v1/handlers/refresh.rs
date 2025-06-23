use axum::{
    http::StatusCode,
    Json,
};
use crate::services::token_service::refresh;
use crate::models::{
    auth::RefreshRequest, 
    token::TokenResponse
};
use std::sync::Arc;
use crate::core::app_state::AppState;
use axum::extract::State;

pub async fn refresh_handler(State(state): State<Arc<AppState>>,
    Json(request): Json<RefreshRequest>) -> Result<Json<TokenResponse>, StatusCode> {
    match refresh(State(state), Json(request)).await{
        Ok(Json(response))=>return Ok(Json(response)),
        Err(_)=>return Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}