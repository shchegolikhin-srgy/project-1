use crate::core::config::Settings;

#[derive(Clone)]
pub struct AppState {

}

impl AppState{
    pub async fn new(settings:Settings)->Result<Self, anyhow::Error>{
        
        Ok(Self {
            
         })
    } 
}