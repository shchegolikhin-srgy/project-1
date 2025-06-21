pub mod core;
pub use core::AppState;
pub mod services;
pub mod api;
pub mod models;

use axum::{
    Router,
    serve,
    routing::{get, post},
    middleware::from_fn_with_state,
};
use core::config::Settings;
use std::sync::Arc;
use api::routes;

pub async fn run_server(state: Arc<AppState>, settings:Settings)->Result<(), Box<dyn std::error::Error>>{
    let app = Router::new()
        .route("/protected", get(protected_route))
        .layer(from_fn_with_state(state.clone(), routes::auth_middleware))
        .merge(routes::router(state.clone()))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind(&settings.addr).await?;

    serve(listener, app).await?;
    Ok(())
}

async fn protected_route()->String{
    format!("Private route!!!")
}