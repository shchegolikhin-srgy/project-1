use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{ decode, encode, Header, Validation};
use axum::{
    http::StatusCode,
    Json,
};
use crate::core::app_state::AppState;
use axum::extract::State;
use crate::models::{
    login::{AuthResponse, Claims, TokenResponse, RefreshRequest}, 
    user::User,
};
use std::sync::Arc;

const ACCESS_TOKEN_EXPIRE_MINUTES: i64 = 30;

pub async fn login(State(state): State<Arc<AppState>>,
    user:User)->Result<Json<TokenResponse>, StatusCode>{
    //проверка
    if user.password_hash !="ds"{
        return Err(StatusCode::UNAUTHORIZED);
    }
    let access_exp = Utc::now()+Duration::minutes(ACCESS_TOKEN_EXPIRE_MINUTES);
    let access_claims:Claims = Claims {
        sub: user.username.clone(),
        exp: access_exp.timestamp() as usize,
    };
    let access_token = encode(&Header::new(state.algorithm), &access_claims, &state.encoding_key)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let refresh_claims:Claims = Claims { 
        sub: user.username.clone(), 
        exp: (Utc::now() + Duration::days(365)).timestamp() as usize, 
    };
    let refresh_token :String = encode(&Header::new(state.algorithm), &refresh_claims, &state.encoding_key)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(TokenResponse {
         access_token: access_token,
          token_type: "Bearer".to_string(),
           refresh_token: refresh_token,
        }))
}

async fn refresh(State(state): State<Arc<AppState>>,
    Json(request): Json<RefreshRequest>,
)-> Result<Json<TokenResponse>, StatusCode>{
    let token_data = decode::<Claims>(
        &request.refresh_token,
        &state.decoding_key,
        &Validation::new(state.algorithm),
    );
    return Err(StatusCode::UNAUTHORIZED);
}