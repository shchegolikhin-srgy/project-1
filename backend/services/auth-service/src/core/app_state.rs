use sqlx::postgres::PgPoolOptions;
use sqlx::{database, PgPool};


#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    //pub jwt_secret:[u8],
}

impl AppState{
    pub async fn new(database_url:&str)->Result<Self, sqlx::Error>{
        let pool = PgPoolOptions::new()
        .max_connections(40)
        .connect(database_url)
        .await?;
        Ok(Self {
            pool: pool,
            //jwt_secret:b"my-secret-key",
         })
    } 
}