use serde::{Deserialize, Serialize};

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