use axum::Error;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

pub struct Settings{
    pub database_url:String,
    pub max_pool_connections:u8,
    pub addr:SocketAddr,
}

impl Settings{
    pub async fn new()->Result<Self, Error>{
        Ok(Self {
             database_url: String::from("postgres://user:password@localhost:5432/projectdb"),
              max_pool_connections:  40,
              addr:SocketAddr::from(([0, 0, 0, 0], 4002)),
        })
    }
}