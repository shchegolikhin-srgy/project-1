use axum::{
    http::StatusCode,
    Json,
};
use crate::models::login::RegisterRequest;
use std::sync::Arc;
use crate::core::app_state::AppState;
use axum::extract::State;
use crate::services::register_service;
use crate::models::user::RegisterUser;


pub async fn register_handler(
    State(state): State<Arc<AppState>>,
    Json(request): Json<RegisterRequest>,
) -> Result<(), StatusCode> {
    register_service::register_user_by_username(State(state.clone()),RegisterUser{
        username:request.username,
        password: request.password,
        email:String::from("_"),
        role:String::from("user")
    }).await.unwrap();
    Ok(())
}