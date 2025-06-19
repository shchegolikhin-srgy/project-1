use axum::http::request;
use axum::{
    http::StatusCode,
    Json,
};
use crate::models::{login::{TokenResponse, LoginRequest}, user::UserData};
use crate::services::token_service::login;
use std::sync::Arc;
use crate::core::app_state::AppState;
use axum::extract::State;

pub async fn login_handler(
    State(state): State<Arc<AppState>>,
    Json(request): Json<LoginRequest>
) -> Result<Json<TokenResponse>, StatusCode> {
    return login(State(state), UserData{
        username:request.username,
        password:request.password,
        email:String::from("_"),
        role:String::from("user"),
    }).await
}