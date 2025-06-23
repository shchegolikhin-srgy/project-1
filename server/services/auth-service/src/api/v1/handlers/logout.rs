use axum::{
    http::{StatusCode,HeaderMap},
    Json,
};
use crate::services::{
    auth_service::{delete_user, logout}
};
use crate::models::token::Claims;
use std::sync::Arc;
use crate::core::app_state::AppState;
use axum::extract::State;

pub async fn logout_handler(State(state): State<Arc<AppState>>,
    Json(request): Json<Claims>)->Result<(), StatusCode>{
    Ok(())
}

pub async fn delete_user_handler(State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    )->Result<(), StatusCode>{
    if let Some(username)= headers.get("X-Username"){
        delete_user(State(state), username.to_str().unwrap_or("invalid"))
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(())
    }
    else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}