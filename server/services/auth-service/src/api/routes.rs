use axum::{
    Router,
    routing::{post, get},
};
use crate::api::handlers::{login, refresh, register, logout};
use crate::core::app_state::AppState;
use std::sync::Arc;

pub fn router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/login", post(login::login_handler))
        .route("/register", post(register::register_handler))
        .route("/refresh", post(refresh::register_handler))
        .route("/logout", post(logout::logout_handler))
        .with_state(state)
}