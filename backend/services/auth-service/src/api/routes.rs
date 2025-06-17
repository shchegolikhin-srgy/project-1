use axum::{
    Router,
    routing::{post, get},
};
use crate::api::handlers::{login, refresh, register, logout};
use crate::core::app_state::AppState;
use axum::extract::State;
use std::sync::Arc;

pub fn router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/login", post(login::login_handler))
        .route("/register", post(register::register_handler))
        .route("/refresh", post(register::register_handler))
        .route("/logout", post(register::register_handler))
}