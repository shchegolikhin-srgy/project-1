use axum::http:: {HeaderMap, StatusCode};
use crate::services::{
    auth_service::delete_user,
    token_service::logout,
};
use crate::models::{auth::LogoutRequest, token::RefreshTokenData}; 
use std::sync::Arc;
use crate::core::app_state::AppState;
use axum::{extract::State, Json};
use uuid::Uuid;
pub async fn logout_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(request): Json<LogoutRequest>,
)->Result<(), StatusCode>{
    if let Some(id)= headers.get("X-User-Id"){
        let data:RefreshTokenData =RefreshTokenData {
            refresh_token: request.refresh_token, 
            id: Uuid::parse_str(
                &id.to_str()
                .unwrap_or("invalid")
                .to_string())
                .map_err(|_| return StatusCode::UNAUTHORIZED)?,
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
    if let Some(username)= headers.get("X-User-Id"){
        delete_user(State(state), username.to_str().unwrap_or("invalid"))
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(())
    }
    else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}