pub mod core;
pub use core::AppState;
pub mod services;
pub mod api;
pub mod models;

use axum::extract::State;
use axum::{
    Router,
    serve,
};
use core::config::Settings;
use std::sync::Arc;
use api::routes;

pub async fn run_server(state: Arc<AppState>, settings:Settings)->Result<(), Box<dyn std::error::Error>>{
    let app = Router::new()
        .merge(routes::router(state.clone()))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind(&settings.addr).await?;

    serve(listener, app).await?;
    Ok(())
}