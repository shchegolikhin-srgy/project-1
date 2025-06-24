use chrono::{Duration, Utc};
use jsonwebtoken::{ decode, encode, Header, TokenData, Validation};
use crate::services::auth_service;
use crate::core::app_state::AppState;
use axum::{extract::State, Json};
use crate::models::{
    auth::RefreshRequest, 
    user::{UserData, User},
    token::*,
};
use crate::db::token_data::*;
use std::sync::Arc;

const ACCESS_TOKEN_EXPIRE_MINUTES: i64 = 30;

pub async fn login(State(state): State<Arc<AppState>>,
    user:UserData)->Result<Json<TokenResponse>, anyhow::Error>{
        let db_user:User = match auth_service::check_user_by_username(State(state.clone()), user).await {
            Ok(user) => user,
            _=>return Err(anyhow::anyhow!("Invalid username or password")),
    };
    let access_exp = Utc::now()+Duration::minutes(ACCESS_TOKEN_EXPIRE_MINUTES);

    let access_claims:Claims = Claims {
        sub: db_user.username.clone(),
        exp: access_exp.timestamp() as usize,
        role: db_user.role.clone()
    };
    let access_token = encode(&Header::new(state.algorithm), &access_claims, &state.encoding_key)?;
    let refresh_claims:Claims = Claims { 
        sub:db_user.username.clone(), 
        exp: (Utc::now() + Duration::days(365)).timestamp() as usize, 
        role:db_user.role,  
    };
    let refresh_token :String = encode(&Header::new(state.algorithm), &refresh_claims, &state.encoding_key)?;
    let refresh_token_data:RefreshTokenData= RefreshTokenData {
        refresh_token: refresh_token.clone(), 
        username: db_user.username
    };
    match insert_refresh_token(&state.pool, refresh_token_data).await{
        Ok(())=>return Ok(Json(TokenResponse {
            access_token: access_token,
            token_type: "Bearer".to_string(),
            refresh_token: refresh_token, 
        })),
        Err(_)=>return Err(anyhow::anyhow!("server error"))
    }
}

pub async fn refresh(State(state): State<Arc<AppState>>,
    Json(request): Json<RefreshRequest>,
)-> Result<Json<TokenResponse>, anyhow::Error>{
    let token_data:TokenData<Claims> = decode::<Claims>(
        &request.refresh_token,
        &state.decoding_key,
        &Validation::new(state.algorithm),
    ).unwrap();
    let access_exp = Utc::now()+Duration::minutes(ACCESS_TOKEN_EXPIRE_MINUTES);
    let access_claims = Claims {
        sub: token_data.claims.sub.clone(),  
        exp: access_exp.timestamp() as usize,
        role: token_data.claims.role,
    };
    let access_token = encode(&Header::new(state.algorithm), &access_claims, &state.encoding_key)?;
    let refresh_token_data:RefreshTokenData= RefreshTokenData {
        refresh_token: request.refresh_token.clone(), 
        username: token_data.claims.sub
    };
    match check_refresh_token(&state.pool, refresh_token_data).await{
        Ok(())=>return Ok(Json(TokenResponse {
            access_token: access_token,
            token_type: "Bearer".to_string(),
            refresh_token: request.refresh_token.clone(),
        })),
        Err(_)=>return Err(anyhow::anyhow!("server error"))
    }
}

pub async fn verify_jwt(State(state): State<Arc<AppState>>, token: &str) -> Result<User, anyhow::Error> {
    let claims = decode::<Claims>(
        token,
        &state.decoding_key,
        &Validation::new(state.algorithm))?;
    Ok(User{
        username:claims.claims.sub, 
        role: claims.claims.role,
    })
}

pub async fn logout(State(state): State<Arc<AppState>>,
    Json(claims):Json<Claims>, 
)-> Result<(), anyhow::Error>{
    Ok(())
}