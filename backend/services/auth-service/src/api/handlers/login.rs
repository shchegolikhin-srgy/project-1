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
    if payload.username != "user" ||payload.password !="1234"{
        return Err(StatusCode::UNAUTHORIZED);
    }
    else{
        let claims =  HashMap::<String, String>::from([
            ("sub".to_string(), BASE64_STANDARD.encode(&payload.username)),
            ("username".to_string(), payload.username),
        ]);
        let token = encode(
            &Header::default(),
            &claims,
        &EncodingKey::from_secret(JWT_SECRET)
        ).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(Json(AuthResponse { token }))
    }
}