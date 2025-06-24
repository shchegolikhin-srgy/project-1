use sqlx;
use sqlx::PgPool;
use crate::models::token::*;

pub async fn delete_refresh_token(
    pool:&PgPool,
    data:RefreshTokenData,
) -> Result<(), anyhow::Error> {
    sqlx::query(
        "DELETE FROM refresh_tokens WHERE id = $1;")
        
        .execute(pool) 
        .await?;
    Ok(())
}

pub async fn insert_refresh_token(
    pool:&PgPool,
    data:RefreshTokenData,
)-> Result<(), anyhow::Error> {
    sqlx::query(
        "INSERT INTO refresh_tokens(token, user_id) SELECT $1, users.id FROM users WHERE users.username = $2;")
        .bind(&data.refresh_token)
        .bind(&data.username)
        .execute(pool) 
        .await?;
    Ok(())
}

pub async fn check_refresh_token(
    pool:&PgPool,
    data:RefreshTokenData,
)-> Result<(), anyhow::Error> {
    let current_token = sqlx::query_as::<_, RefreshToken>(
        "SELECT refresh_tokens.token FROM refresh_tokens JOIN users ON refresh_tokens.user_id = users.id WHERE users.username =$1;")
        .bind(&data.refresh_token)
        .fetch_optional(pool) 
        .await?;
    if let Some(token) = current_token.clone(){
        if token.refresh_token == data.refresh_token{ 
            return Ok(()) 
        }
        else{ 
            return Err(anyhow::anyhow!("token not found"))
        }
    }
    else { return Err(anyhow::anyhow!("token not found"))};
}