use serde::{Deserialize, Serialize};
use sqlx;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User{
    pub password: String,
    pub username: String,
    pub email: String,
    pub role:String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UserRole{
    pub role:String,
}

#[derive(sqlx::FromRow)]
pub struct DbUser {
    pub username: String,
    pub role:String,
    pub password_hash: String,
}
