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
        let cluster_config = NodeTcpConfigBuilder::new()
            .with_contact_point("127.0.0.1:9042".into())
            .build()
            .await?;
        let session = TcpSessionBuilder::new(RoundRobinLoadBalancingStrategy::new(), cluster_config)
            .build()
            .await?;

        let create_ks = "CREATE KEYSPACE IF NOT EXISTS posts WITH REPLICATION = { \
            'class' : 'SimpleStrategy', 
            'replication_factor' : 1 };";
        session
            .query(create_ks)
            .await?;
        Ok(Self {
            
         })
    } 
}