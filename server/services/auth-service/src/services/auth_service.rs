use crate::core::app_state::AppState;
use axum::{extract::State, http::StatusCode};
use crate::models::user::{UserData, DbUser};
use sqlx;
use std::sync::Arc;
use bcrypt::{verify, DEFAULT_COST};

pub async fn user_exists_by_username(
    State(state): State<Arc<AppState>>,
    user:&UserData
)-> Result<Option<DbUser>, sqlx::Error>{
    let result = sqlx::query_as::<_, DbUser>(
        "SELECT username, role, password_hash FROM users WHERE username= $1;",
    )
    .bind(&user.username)
    .fetch_optional(&state.pool) 
    .await?;
    Ok(result)
}

pub async fn check_user_by_email(
    State(state): State<Arc<AppState>>,
    user:UserData
)-> Result<Option<DbUser>, anyhow::Error>{
    let result =sqlx::query_as::<_, DbUser>(
        "SELECT username, role, password_hash FROM users WHERE email= $1;"
    )
    .bind(&user.email)
    .fetch_optional(&state.pool) 
    .await?;
    Ok(result)
}

pub async fn check_user_by_username(
    State(state): State<Arc<AppState>>,
    user: UserData
)-> Result<Option<DbUser>, sqlx::Error>{
    let result = sqlx::query_as::<_, DbUser>(
        "SELECT username, role, password_hash FROM users WHERE username = $1;",
    )
    .bind(&user.username)
    .fetch_optional(&state.pool) 
    .await?;
    Ok(result)
}


pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    user:&UserData
)-> Result<(), sqlx::Error>{
    sqlx::query(
        "DELETE FROM users WHERE username =$1;"
    )
    .bind(&user.username)
    .execute(&state.pool) 
    .await?;
    Ok(())
}

pub async fn update_user_role(
    State(state): State<Arc<AppState>>,
    user:&UserData, new_role:String
)-> Result<(), sqlx::Error>{
    sqlx::query(
        "UPDATE users SET role = $1  WHERE username =$2 AND password_hash = $3;")
    .bind(new_role)
    .bind(&user.username)
    .bind(&user.password)
    .execute(&state.pool) 
    .await?;
    Ok(())
}