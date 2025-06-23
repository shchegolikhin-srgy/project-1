pub mod core;
pub use core::AppState;
pub mod services;
pub mod api;
pub mod models;
pub mod db;

use axum::{
    Router,
    serve,
    extract::State,
};
use core::config::Settings;
use std::sync::Arc;
use api::v1::routes;

pub async fn run_server(state: Arc<AppState>, settings:Settings)->Result<(), Box<dyn std::error::Error>>{
    let app = Router::new()
        .merge(routes::protected_router(state.clone()))
        .merge(routes::public_router(state.clone()))
        .with_state(state.clone());
    let listener = tokio::net::TcpListener::bind(&settings.addr).await?;
    serve(listener, app).await?;
    Ok(())
}

