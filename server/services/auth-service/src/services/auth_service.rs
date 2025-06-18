use crate::core::app_state::AppState;
use axum::extract::State;
use crate::models::user::User;
use sqlx;
use std::sync::Arc;

pub async fn user_exists_by_username(
    State(state): State<Arc<AppState>>,
    user:User
)-> Result<bool, sqlx::Error>{
    let user = sqlx::query!(
        "SELECT username, role FROM users WHERE username = $1;",
        user.username
    )
    .fetch_one(&state.pool) 
    .await?;
    Ok(false)
}



pub async fn check_user_by_email(
    State(state): State<Arc<AppState>>,
    user:User
)-> Result<bool, sqlx::Error>{
    let user = sqlx::query!(
        "SELECT username, role, password_hash FROM users WHERE email= $1;",
        user.email
    )
    .fetch_one(&state.pool) 
    .await?;
    Ok(false)
}

pub async fn check_user_by_username(
    State(state): State<Arc<AppState>>,
    user:User
)-> Result<bool, sqlx::Error>{
    let user = sqlx::query!(
        "SELECT username, role, password_hash FROM users WHERE username= $1;",
        user.username
    )
    .fetch_one(&state.pool) 
    .await?;
    Ok(false)
}


pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    user:User
)-> Result<bool, sqlx::Error>{
    sqlx::query!(
        "DELETE FROM users WHERE username =$1 AND password_hash = $2;",
        user.username,
        user.password
    )
    .execute(&state.pool) 
    .await?;
    Ok(false)
}

pub async fn update_user_role(
    State(state): State<Arc<AppState>>,
    user:User, new_role:String
)-> Result<bool, sqlx::Error>{
    sqlx::query!(
        "UPDATE users SET role = $1  WHERE username =$2 AND password_hash = $3;",
        new_role,
        user.username,
        user.password
    )
    .execute(&state.pool) 
    .await?;
    Ok(false)
}