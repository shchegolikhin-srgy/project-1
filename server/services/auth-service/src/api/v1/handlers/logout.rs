use axum::http::{request, HeaderMap, StatusCode};
use crate::services::{
    auth_service::delete_user,
    token_service::logout,
};
use crate::models::{auth::LogoutRequest, token::RefreshTokenData}; 
use std::sync::Arc;
use crate::core::app_state::AppState;
use axum::{extract::State, Json};

pub async fn logout_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(request): Json<LogoutRequest>,
)->Result<(), StatusCode>{
    if let Some(username)= headers.get("X-Username"){
        let data:RefreshTokenData =RefreshTokenData {
            refresh_token: request.refresh_token, 
            username: username.to_str().unwrap_or("invalid").to_string(),
       };
       match logout(State(state), data).await{
           Ok(())=>return Ok(()),
           Err(_)=>return Err(StatusCode::INTERNAL_SERVER_ERROR)
       }
    }
    return Err(StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn delete_user_handler(State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    )->Result<(), StatusCode>{
    if let Some(username)= headers.get("X-Username"){
        delete_user(State(state), username.to_str().unwrap_or("invalid"))
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(())
    }
    else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}