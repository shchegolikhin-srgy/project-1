use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User{
    pub password: String,
    pub username: String,
    pub email: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UserRole{
    pub role:String,
}