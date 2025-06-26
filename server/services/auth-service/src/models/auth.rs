use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub email:String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateRoleRequest{
    pub new_role:String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateRoleData{
    pub new_role:String,
    pub username:String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LogoutRequest {
    pub refresh_token: String,
}