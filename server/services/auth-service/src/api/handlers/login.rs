use base64::prelude::*;
use axum::{
    http::StatusCode,
    Json,
};
use std::collections::HashMap;
use jsonwebtoken::{encode, Header, EncodingKey};
use crate::models::login::{AuthResponse, LoginRequest};

use std::sync::Arc;
use crate::core::app_state::AppState;
use axum::extract::State;

const JWT_SECRET: &[u8] = b"my-secret-key";

pub async fn login_handler(State(state): State<Arc<AppState>>, Json(payload): Json<LoginRequest>) -> Result<Json<AuthResponse>, StatusCode> {
    return Err(StatusCode::UNAUTHORIZED);
}