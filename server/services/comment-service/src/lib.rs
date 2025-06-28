pub mod core;
pub mod db;
pub mod services;
pub mod api;
pub mod models;
pub use core::AppState;

use axum::{
    Router,
    serve,
    extract::State,
};
use core::config::Settings;
use std::sync::Arc;


pub async fn run_server(state: Arc<AppState>, settings:Settings)->Result<(), Box<dyn std::error::Error>>{
    let app = Router::new()
        .with_state(state.clone());
    let listener = tokio::net::TcpListener::bind(&settings.addr).await?;
    serve(listener, app).await?;
    Ok(())
}
