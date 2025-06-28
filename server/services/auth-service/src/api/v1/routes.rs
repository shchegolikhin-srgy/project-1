use axum::{
    body::Body, extract::{Request, State},
    http::{HeaderValue, StatusCode, HeaderMap},
    middleware::{from_fn_with_state, Next}, 
    response::Response, 
    routing::{get, post},
    Router
};
use crate::api::v1::handlers::*;
use crate::core::app_state::AppState;
use std::sync::Arc;
use crate::services::token_service::verify_jwt;

fn extract_token(headers: &HeaderMap) -> Result<String, StatusCode> {
    let auth_header = headers.get("Authorization").ok_or(StatusCode::UNAUTHORIZED)?;
    let auth_str = auth_header.to_str().map_err(|_| StatusCode::UNAUTHORIZED)?;
    let token = auth_str.strip_prefix("Bearer ").ok_or(StatusCode::UNAUTHORIZED)?;
    Ok(token.to_string())
}

pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    
    let token = extract_token(request.headers())?;
    let user = verify_jwt(State(state), &token)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    let mut response = next.run(request).await;

    response.headers_mut().insert(
        "X-User-Id",
        HeaderValue::from_str(&user.id.to_string()).map_err(|_| StatusCode::UNAUTHORIZED)?,
    );
    response.headers_mut().insert(
        "X-User-Role",
        HeaderValue::from_str(&user.role.to_string()).map_err(|_| StatusCode::UNAUTHORIZED)?,
    );
    Ok(response)
}

pub fn protected_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/v1/logout", post(logout::logout_handler))
        .route("/v1/delete_user", post(logout::delete_user_handler))
        .route("/v1/update_role", post(register::update_user_role_handler))
        .with_state(state.clone())
        .layer(from_fn_with_state(state, auth_middleware))
}

pub fn public_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/v1/login", post(login::login_handler))
        .route("/v1/register", post(register::register_handler))
        .route("/v1/refresh", post(refresh::refresh_handler))
        .with_state(state)
}