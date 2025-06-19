use crate::core::app_state::AppState;
use axum::extract::State;
use crate::models::user::User;
use sqlx;
use std::sync::Arc;
use bcrypt::{hash, verify, DEFAULT_COST};

pub async fn register_user_by_username(
    State(state): State<Arc<AppState>>,
    user:User
) -> Result<(), sqlx::Error> {
    println!("Операция в бд!");
    sqlx::query(
        "INSERT INTO users (username, password_hash) VALUES($1, $2);")
        .bind(user.username)
        .bind(user.password)
    .execute(&state.pool) 
    .await?;
    Ok(())
}

pub async fn register_user_by_email(
    State(state): State<Arc<AppState>>,
    user:User
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO users (username, password_hash, email) VALUES($1, $2, $3);")
    .bind(user.username)
    .bind(user.password)
    .bind(user.email)
    .execute(&state.pool) 
    .await?;
    Ok(())
}

pub async fn register_user_by_role(
    State(state): State<Arc<AppState>>,
    user:User,
    role:String
) -> Result<(), sqlx::Error> {
    println!("Операция в бд!");
    sqlx::query(
        "INSERT INTO users (username, password_hash, role, email) VALUES($1, $2, $3, $4);")
    .bind(user.username)
    .bind(user.password)
    .bind(user.role)
    .bind(user.email)
    .execute(&state.pool) 
    .await?;
    Ok(())
}
