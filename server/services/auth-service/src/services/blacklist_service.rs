use crate::core::app_state::AppState;
use axum::extract::State;
use crate::models::user::RegisterUser;
use sqlx;
use std::sync::Arc;
use bcrypt::{hash, DEFAULT_COST};


pub async fn delete_refresh_token(
    State(state): State<Arc<AppState>>,
    user:RegisterUser
) -> Result<(), anyhow::Error> {
    sqlx::query(
        "DELETE FROM refresh_tokens WHERE id = $1;")
        
    .execute(&state.pool) 
    .await?;
    Ok(())
}