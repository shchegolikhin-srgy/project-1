use axum::{
    http::StatusCode,
    Json,
};
use crate::models::{
    auth::LoginRequest, 
    user::UserData,
    token::TokenResponse,
};
use crate::services::token_service::login;
use std::sync::Arc;
use crate::core::app_state::AppState;
use axum::extract::State;

pub async fn login_handler(
    State(state): State<Arc<AppState>>,
    Json(request): Json<LoginRequest>
) -> Result<Json<TokenResponse>, StatusCode> {
    login(State(state), UserData{
        username:request.username,
        password:request.password,
        email:String::from("_"),
        role:String::from("user"),
    }).await.map_err(|_| StatusCode::UNAUTHORIZED)
}