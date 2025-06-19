use axum::{
    Router,
    routing::{post, get},
    http::{ StatusCode, Request},
    extract::{State, Extension},
};
use crate::api::handlers::{login, refresh, register, logout};
use crate::core::app_state::AppState;
use std::sync::Arc;
use crate::services::token_service::verify_jwt;


pub fn router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/login", post(login::login_handler))
        .route("/register", post(register::register_handler))
        .route("/refresh", post(refresh::register_handler))
        .route("/logout", post(logout::logout_handler))
        .with_state(state.clone())
}

pub async fn auth_middleware(
    State(state): axum::extract::State<Arc<AppState>>,
    mut request: Request<axum::body::Body>,
) -> Result<Request<axum::body::Body>, StatusCode> {
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());

    let token = auth_header
        .and_then(|t| t.strip_prefix("Bearer "))
        .ok_or_else(|| StatusCode::UNAUTHORIZED)?;

    let username = verify_jwt(State(state), token).await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    request.extensions_mut().insert(username);
    Ok(request)
}

async fn protected_route(Extension(username): Extension<String>) -> String {
    format!("Вы авторизованы как: {}", username)
}