use axum::{
    http::StatusCode,
    Json,
};
use crate::models::login::RegisterRequest;
use std::sync::Arc;
use crate::core::app_state::AppState;
use axum::extract::State;
use crate::services::register_service;
use crate::models::user::User;


pub async fn register_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(), StatusCode> {
    register_service::register_user_by_username(State(state.clone()), User{
        password_hash: String::from("password"),
        username: String::from("pidor@"),
        email: String::from("example@"),
    }).await;
    Ok(())
}

pub async fn register_by_role_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(), StatusCode> {
    register_service::register_user_by_role(State(state.clone()), User{
        password_hash: String::from("password"),
        username: String::from("pidor@"),
        email: String::from("example@"),
    }, String::from("admin")).await;
    Ok(())
}