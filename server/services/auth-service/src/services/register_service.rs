use crate::core::app_state::AppState;
use axum::extract::State;
use crate::models::user::RegisterUser;
use sqlx;
use std::sync::Arc;
use bcrypt::{hash, verify, DEFAULT_COST};

pub async fn register_user_by_username(
    State(state): State<Arc<AppState>>,
    user:RegisterUser
) -> Result<(), sqlx::Error> {
    let password_hash:String = hash(user.password, DEFAULT_COST).unwrap();
    sqlx::query(
        "INSERT INTO users (username, password_hash) VALUES($1, $2);")
        .bind(user.username)
        .bind(password_hash)
    .execute(&state.pool) 
    .await?;
    Ok(())
}

pub async fn register_user_by_email(
    State(state): State<Arc<AppState>>,
    user:RegisterUser
) -> Result<(), sqlx::Error> {
    let password_hash:String = hash(user.password, DEFAULT_COST).unwrap();
    sqlx::query(
        "INSERT INTO users (username, password_hash, email) VALUES($1, $2, $3);")
    .bind(user.username)
    .bind(password_hash)
    .bind(user.email)
    .execute(&state.pool) 
    .await?;
    Ok(())
}

pub async fn register_user_by_role(
    State(state): State<Arc<AppState>>,
    user:RegisterUser,
) -> Result<(), sqlx::Error> {
    let password_hash:String = hash(user.password, DEFAULT_COST).unwrap();
    sqlx::query(
        "INSERT INTO users (username, password_hash, role, email) VALUES($1, $2, $3, $4);")
    .bind(user.username)
    .bind(password_hash)
    .bind(user.role)
    .bind(user.email)
    .execute(&state.pool) 
    .await?;
    Ok(())
}
