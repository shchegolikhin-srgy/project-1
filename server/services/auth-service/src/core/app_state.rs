use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey};
use crate::core::config::Settings;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub encoding_key: EncodingKey,
    pub  decoding_key: DecodingKey,
    pub algorithm:Algorithm,
}

impl AppState{
    pub async fn new(settings:Settings)->Result<Self, anyhow::Error>{
        let pool = PgPoolOptions::new()
        .max_connections(settings.max_pool_connections)
        .connect(&settings.database_url)
        .await?;
        Ok(Self {
            pool: pool,
            encoding_key: EncodingKey::from_secret(settings.secret_key.as_bytes()),
            decoding_key: DecodingKey::from_secret(settings.secret_key.as_bytes()),
            algorithm:Algorithm::HS256,
         })
    } 
}