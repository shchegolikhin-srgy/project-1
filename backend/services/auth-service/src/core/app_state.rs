use sqlx::postgres::PgPoolOptions;
use sqlx::{database, PgPool};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

impl AppState{
    pub async fn new(database_url:&str)->Result<Self, sqlx::Error>{
        let pool = PgPoolOptions::new()
        .max_connections(40)
        .connect(database_url)
        .await?;
        Ok(Self {
            pool: pool,
         })
    } 
}