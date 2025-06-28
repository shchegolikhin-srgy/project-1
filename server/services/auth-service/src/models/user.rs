use serde::{Deserialize, Serialize};
use sqlx;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegisterUser{
    pub password: String,
    pub username: String,
    pub email: String,
    pub role:String
}

#[derive(Debug, Clone)]
pub struct User{
    pub id:Uuid,
    pub role:String
}

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct DbUser {
    pub external_id:Uuid,
    pub role:String,
    pub password_hash: String,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct UserData {
    pub username: String,
    pub password: String,
    pub email:String,
    pub role:String,
}
