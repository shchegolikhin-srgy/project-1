use std::net::SocketAddr;
use std::env;

#[derive(Debug, Clone)]
pub struct Settings{
    pub addr:SocketAddr,
}

impl Settings{
    pub async fn new()->Result<Self, anyhow::Error>{
        Ok(Self {
            addr:SocketAddr::from(([0, 0, 0, 0], 4000)),
        })
    }
}