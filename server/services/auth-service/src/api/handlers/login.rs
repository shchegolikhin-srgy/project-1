use axum::{
    http::{HeaderMap, StatusCode},
    Json,
};
use crate::models::{login::{TokenResponse, LoginRequest}, user::{UserData, User}};
use crate::services::token_service::{verify_jwt, login};
use std::sync::Arc;
use crate::core::app_state::AppState;
use axum::extract::State;

pub async fn login_handler(
    State(state): State<Arc<AppState>>,
    Json(request): Json<LoginRequest>
) -> Result<Json<TokenResponse>, StatusCode> {
    let result = login(State(state), UserData{
        username:request.username,
        password:request.password,
        email:String::from("_"),
        role:String::from("user"),
    }).await;
    result
}

pub async fn verify_token(State(state): State<Arc<AppState>>,headers: HeaderMap)->Result<String, StatusCode>{
    let token = headers
        .get("Authorization")
        .ok_or(StatusCode::UNAUTHORIZED)?
        .to_str()
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::BAD_REQUEST)?;
    
    let user:User = verify_jwt(State(state), token).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let response:String = format!("X-Username: {}, X-Role: {}", user.username, user.role);
    return Err(StatusCode::UNAUTHORIZED)
}