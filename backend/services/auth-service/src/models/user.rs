use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User{
    pub password_hash: String,
    pub username: String,
    pub email: String,
}