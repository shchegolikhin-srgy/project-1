use chrono::{Duration, Utc};
use jsonwebtoken::{ decode, encode, Header, TokenData, Validation};
use axum::{
    http::StatusCode,
    Json,
};
use crate::services::auth_service;
use crate::core::app_state::AppState;
use axum::extract::State;
use crate::models::{
    login::{Claims, TokenResponse, RefreshRequest}, 
    user::{UserData, DbUser},
};
use std::sync::Arc;

const ACCESS_TOKEN_EXPIRE_MINUTES: i64 = 30;

pub async fn login(State(state): State<Arc<AppState>>,
    user:UserData)->Result<Json<TokenResponse>, StatusCode>{
        let db_user = match auth_service::check_user_by_username(State(state.clone()), user).await {
            Ok(Some(user)) => (user),
            Ok(None)=>return Err(StatusCode::UNAUTHORIZED),
            Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };
    let access_exp = Utc::now()+Duration::minutes(ACCESS_TOKEN_EXPIRE_MINUTES);

    let access_claims:Claims = Claims {
        sub: db_user.username.clone(),
        exp: access_exp.timestamp() as usize,
        role: db_user.role.clone()
    };
    let access_token = encode(&Header::new(state.algorithm), &access_claims, &state.encoding_key)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    let refresh_claims:Claims = Claims { 
        sub:db_user.username, 
        exp: (Utc::now() + Duration::days(365)).timestamp() as usize, 
        role:db_user.role,  
    };
    let refresh_token :String = encode(&Header::new(state.algorithm), &refresh_claims, &state.encoding_key)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    Ok(Json(TokenResponse {
        access_token: access_token,
        token_type: "Bearer".to_string(),
        refresh_token: refresh_token,
    }))
}

pub async fn refresh(State(state): State<Arc<AppState>>,
    Json(request): Json<RefreshRequest>,
)-> Result<Json<TokenResponse>, StatusCode>{
    let token_data:TokenData<Claims> = decode::<Claims>(
        &request.refresh_token,
        &state.decoding_key,
        &Validation::new(state.algorithm),
    ).unwrap();

     //  Проверки жизни токена через бд

    let access_exp = Utc::now()+Duration::minutes(ACCESS_TOKEN_EXPIRE_MINUTES);
    let access_claims = Claims {
        sub: token_data.claims.sub,  
        exp: access_exp.timestamp() as usize,
        role: token_data.claims.role,
    };
    let access_token = encode(&Header::new(state.algorithm), &access_claims, &state.encoding_key)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(TokenResponse {
        access_token: access_token,
        token_type: "Bearer".to_string(),
        refresh_token: request.refresh_token,
        }))
}

pub async fn logout(State(state): State<Arc<AppState>>,
    Json(claims):Json<Claims>, 
)-> Result<(), StatusCode>{
    Ok(())
}
async fn protected_route(claims: Claims) -> String {
    format!("Hello, {}! This is a protected route.", claims.sub)
}