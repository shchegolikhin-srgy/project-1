use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub encoding_key: EncodingKey,
    pub  decoding_key: DecodingKey,
    pub algorithm:Algorithm,
}

const SECRET_KEY: &str = "your-secret-key-here";


impl AppState{
    pub async fn new(database_url:&str)->Result<Self, sqlx::Error>{
        let pool = PgPoolOptions::new()
        .max_connections(40)
        .connect(database_url)
        .await?;
        Ok(Self {
            pool: pool,
            encoding_key: EncodingKey::from_secret(SECRET_KEY.as_bytes()),
            decoding_key: DecodingKey::from_secret(SECRET_KEY.as_bytes()),
            algorithm:Algorithm::HS256,
         })
    } 
}