use crate::core::app_state::AppState;
use axum::extract::State;
use crate::models::user::User;
use sqlx;
use std::sync::Arc;

pub async fn register_user_by_username(
    State(state): State<Arc<AppState>>,
    user:User
) -> Result<(), sqlx::Error> {
    println!("Операция в бд!");
    sqlx::query!(
        "INSERT INTO users (username, hashed_password) VALUES($1, $2)",
        user.username,
        user.hashed_password
    )
    .execute(&state.pool) 
    .await?;
    Ok(())
}

pub async fn register_user_by_email(
    State(state): State<AppState>,
    user:User
) -> Result<(), sqlx::Error> {
    println!("Операция в бд!");
    sqlx::query!(
        "INSERT INTO users (username, hashed_password) VALUES($1, $2)",
        user.username,
        user.hashed_password
    )
    .execute(&state.pool) 
    .await?;
    Ok(())
}

pub async fn register_user_by_role(
    State(state): State<AppState>,
    user:User
) -> Result<(), sqlx::Error> {
    println!("Операция в бд!");
    sqlx::query!(
        "INSERT INTO users (username, hashed_password) VALUES($1, $2)",
        user.username,
        user.hashed_password
    )
    .execute(&state.pool) 
    .await?;
    Ok(())
}
