
use auth_service::{run_server, AppState};
use auth_service::core::config::Settings;
use std::sync::Arc;
use dotenv::dotenv;

#[tokio::main]
async fn main()-> Result<(), Box<dyn std::error::Error>>{
    dotenv().ok();
    let settings = Settings::new().await?;
    let app_state = AppState::new(settings.clone()).await?; 
    let state = Arc::new(app_state);
    run_server(state.clone(), settings).await?;
    Ok(())
}
