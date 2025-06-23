use crate::models::user::RegisterUser;
use sqlx;
use sqlx::PgPool;

pub async fn delete_refresh_token(
    pool:&PgPool,
    user:RegisterUser
) -> Result<(), anyhow::Error> {
    sqlx::query(
        "DELETE FROM refresh_tokens WHERE id = $1;")
        
    .execute(pool) 
    .await?;
    Ok(())
}

pub async fn insert_refresh_token(
    pool:&PgPool,
)-> Result<(), anyhow::Error> {
    sqlx::query(
        "DELETE FROM refresh_tokens WHERE id = $1;")
        
    .execute(pool) 
    .await?;
    Ok(())
}