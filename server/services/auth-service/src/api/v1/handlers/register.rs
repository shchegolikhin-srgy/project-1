use axum::{
    http::StatusCode,
    Json,
};
use crate::models::auth::RegisterRequest;
use std::sync::Arc;
use crate::core::app_state::AppState;
use axum::extract::State;
use crate::services::{
    auth_service::*,
};
use crate::models::user::RegisterUser;

pub async fn register_handler(
    State(state): State<Arc<AppState>>,
    Json(request): Json<RegisterRequest>,
) -> Result<(), StatusCode> {
    register_user_by_email(State(state.clone()),RegisterUser{
        username:request.username,
        password: request.password,
        email:request.email,
        role:String::from("user")
    }).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}

pub async fn update_user_role_handler(State(state): State<Arc<AppState>>)->Result<(), StatusCode>{
    
    Ok(())
}