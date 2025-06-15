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
use sqlx::Error;
use models::user::User;


pub async fn run_server(state: Arc<AppState>, settings:Settings)->Result<(), Box<dyn std::error::Error>>{
    let app:Router = Router::new()
        .with_state(state.clone())
        .merge(routes::router());
    let listener = tokio::net::TcpListener::bind(&settings.addr).await?;
    use services::register_service;
    let conn:Result<(), Error> = register_service::register_user_by_username(State(state.clone()), User{
        hashed_password: String::from("password"),
        username: String::from("pidor@"),
        email: String::from("example@"),
    }).await;
    match conn {
        Ok(())=>println!("Ok"),
        _ =>println!("Failed!!!!"),
    }
    

    serve(listener, app).await?;
    Ok(())
}
