use crate::core::config::Settings;
use cdrs_tokio::cluster::session::{TcpSessionBuilder, SessionBuilder};
use cdrs_tokio::cluster::NodeTcpConfigBuilder;
use cdrs_tokio::load_balancing::RoundRobinLoadBalancingStrategy;
use cdrs_tokio::query::*;

#[derive(Clone)]
pub struct AppState {
    
}

impl AppState{
    pub async fn new(settings:Settings)->Result<Self, anyhow::Error>{
        
        Ok(Self {
            
         })
    } 
}