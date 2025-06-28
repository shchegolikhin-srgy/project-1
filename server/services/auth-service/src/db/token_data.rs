use sqlx;
use sqlx::PgPool;
use crate::models::token::*;
use uuid::Uuid;

pub async fn delete_refresh_token(
    pool:&PgPool,
    data:RefreshTokenData,
) -> Result<(), anyhow::Error> {
    sqlx::query(
        "DELETE FROM refresh_tokens
        USING users
        WHERE refresh_tokens.user_id = users.id
        AND users.external_id = $1
        AND refresh_tokens.token = $2;")
        .bind(&data.id)
        .bind(&data.refresh_token)
        .execute(pool) 
        .await?;
    Ok(())
}

pub async fn insert_refresh_token(
    pool:&PgPool,
    data:RefreshTokenData,
)-> Result<(), anyhow::Error> {
    sqlx::query(
        "INSERT INTO refresh_tokens(token, user_id) 
            SELECT $1, users.id 
            FROM users 
            WHERE users.external_id = $2;")
        .bind(&data.refresh_token)
        .bind(&data.id)
        .execute(pool) 
        .await?;
    Ok(())
}

pub async fn check_refresh_token(
    pool:&PgPool,
    data:RefreshTokenData,
)-> Result<(), anyhow::Error> {
    let tokens = sqlx::query_as::<_, RefreshToken>(
        "SELECT refresh_tokens.token 
            FROM refresh_tokens 
            JOIN users ON refresh_tokens.user_id = users.id 
            WHERE users.external_id = $1")
        .bind(&data.id)
        .fetch_all(pool) 
        .await?;
    
    if tokens.iter().any(|t| t.token == data.refresh_token) {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Token not found"))
    }
}