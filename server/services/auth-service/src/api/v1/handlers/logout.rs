use axum::{
    http::StatusCode,
    Json,
};
use crate::services::token_service::logout;
use crate::models::login::Claims;
use std::sync::Arc;
use crate::core::app_state::AppState;
use axum::extract::State;

pub async fn logout_handler(State(state): State<Arc<AppState>>,
    Json(request): Json<Claims>)->Result<(), StatusCode>{
    return logout(State(state), Json(request)).await
}

pub async fn delete_user(State(state): State<Arc<AppState>>)->Result<(), StatusCode>{
    Ok(())
}