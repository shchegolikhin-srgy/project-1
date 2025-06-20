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
use models::user::User;

pub async fn run_server(state: Arc<AppState>, settings:Settings)->Result<(), Box<dyn std::error::Error>>{
    let app = Router::new()
        .merge(routes::router(state.clone()))
        .with_state(state.clone());
    let listener = tokio::net::TcpListener::bind(&settings.addr).await?;

    let result:User = services::token_service::verify_jwt(State(state), "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJTZXJnZXkiLCJleHAiOjE3NTA0MjE1NjUsInJvbGUiOiJwaWRvciJ9.al_npWzh0EuZPkyFtO1xAljBuk98BNVZZT65EhtHtxo").await.unwrap();
    println!("Username is {}, role is {}", result.username, result.role); 
    serve(listener, app).await?;
    Ok(())
}