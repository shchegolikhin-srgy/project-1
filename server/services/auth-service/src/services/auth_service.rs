use crate::{core::app_state::AppState};
use axum::{extract::State};
use crate::models::{
    user::*,
    auth::*,
};
use std::sync::Arc;
use bcrypt::{hash, DEFAULT_COST, verify};
use crate::db::user_data;

pub async fn register_user_by_email(
    State(state): State<Arc<AppState>>,
    user:RegisterUser
) -> Result<(), anyhow::Error> {
    let password_hash:String = hash(user.password.clone(), DEFAULT_COST)?;
    match user_data::register_user_by_email(&state.pool, user, password_hash).await{
        Ok(())=>Ok(()),
        _=>Err(anyhow::anyhow!("User already exists"))
    }
}

pub async fn register_user_by_role(
    State(state): State<Arc<AppState>>,
    user:RegisterUser,
) -> Result<(), anyhow::Error> {
    let password_hash:String = hash(user.password.clone(), DEFAULT_COST)?;
    match user_data::register_user_by_role(&state.pool, user, password_hash).await{
        Ok(())=>Ok(()),
        _=>Err(anyhow::anyhow!("User already exists"))
    }
}

pub async fn check_user_by_email(
    State(state): State<Arc<AppState>>,
    user:UserData
)-> Result<User, anyhow::Error>{
    match user_data::check_user_by_username(&state.pool, user.clone()).await {
        Ok((password_hash, id))=>{
            if !verify(&user.password.clone(), &password_hash)? {
                Err(anyhow::anyhow!("Invalid password"))
            }
            else{
                Ok(User{
                    id:id,
                    role:user.role,
                })
            }
        },
        _=>Err(anyhow::anyhow!("Invalide username or password"))
    }
}

pub async fn check_user_by_username(
    State(state): State<Arc<AppState>>,
    user: UserData
)-> Result<User,anyhow::Error>{
    match user_data::check_user_by_username(&state.pool, user.clone()).await {
        Ok((password_hash, id))=>{
            if !verify(&user.password.clone(), &password_hash)? {
                Err(anyhow::anyhow!("Invalid password"))
            }
            else{
                Ok(User{
                    id:id,
                    role:user.role,
                })
            }
        },
        _=>Err(anyhow::anyhow!("Invalide username or password"))
    }
}

pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    username:&str
)-> Result<(), anyhow::Error>{
    match user_data::delete_user(&state.pool, username).await {
        Ok(())=>Ok(()),
        _=>Err(anyhow::anyhow!("couldn't delete user"))
    }
}

pub async fn update_user_role(
    State(state): State<Arc<AppState>>,
    data:UpdateRoleData,
)-> Result<(), anyhow::Error>{
    match user_data::update_user_role(&state.pool, data).await {
        Ok(())=>Ok(()),
        _=>Err(anyhow::anyhow!("couldn't delete user"))
    }
}