pub mod core;
pub use core::AppState;
pub mod services;
pub mod api;
pub mod models;

use axum::{
    Router,
    serve,
    extract::State,
};
use core::config::Settings;
use std::sync::Arc;
use api::v1::routes;

use crate::models::user::UserData;

pub async fn run_server(state: Arc<AppState>, settings:Settings)->Result<(), Box<dyn std::error::Error>>{
    let app = Router::new()
        .merge(routes::public_router(state.clone()))
        .merge(routes::protected_router(state.clone()))
        .with_state(state.clone());
    let listener = tokio::net::TcpListener::bind(&settings.addr).await?;
    println!("token is {:?}", crate::services::token_service::login(State(state), UserData{
        username:String::from("Sergei"),
        password:String::from("1234"),
        email:String::from("_"),
        role:String::from("user"),
    }).await);
    serve(listener, app).await?;
    Ok(())
}

