use axum::Error;
use std::net::SocketAddr;
use std::env;

#[derive(Debug, Clone)]
pub struct Settings{
    pub database_url:String,
    pub max_pool_connections:u8,
    pub addr:SocketAddr,
    pub secret_key:String,
}

impl Settings{
    pub async fn new()->Result<Self, Error>{
        Ok(Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set."),
            max_pool_connections:  40,
            addr:SocketAddr::from(([0, 0, 0, 0], 4002)),
            secret_key: env::var("SECRET_KEY").expect("SECRET_KEY must be set."),
        })
    }
}