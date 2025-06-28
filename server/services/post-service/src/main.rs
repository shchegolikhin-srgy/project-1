use post_service::{run_server, AppState};
use post_service::core::config::Settings;
use std::sync::Arc;
use dotenv::dotenv;

use cdrs_tokio::cluster::session::{TcpSessionBuilder, SessionBuilder};
use cdrs_tokio::cluster::NodeTcpConfigBuilder;
use cdrs_tokio::load_balancing::RoundRobinLoadBalancingStrategy;
use cdrs_tokio::query::*;

#[tokio::main]
async fn main()-> Result<(), Box<dyn std::error::Error>>{
    dotenv().ok();
    let settings = Settings::new().await?;
    let app_state = AppState::new(settings.clone()).await?; 
    let state = Arc::new(app_state);


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

    run_server(state, settings).await?;
    Ok(())
}
