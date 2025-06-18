
use auth_service::{AppState, run_server};
use auth_service::core::config::Settings;
use std::sync::Arc;

#[tokio::main]
async fn main()-> Result<(), Box<dyn std::error::Error>>{
    let settings = Settings::new().await?;

    let app_state = AppState::new(&settings.database_url).await?; 
    let state = Arc::new(app_state);
    run_server(state, settings).await?;

    Ok(())
}
