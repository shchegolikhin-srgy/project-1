use crate::models::user::RegisterUser;
use sqlx;
use sqlx::PgPool;
use crate::models::{
    auth::UpdateRoleData,
    user::{UserData, DbUser}
};

pub async fn register_user_by_email(
    pool:&PgPool,
    user:RegisterUser,
    password_hash:String,
) -> Result<(), anyhow::Error> {
    sqlx::query(
        "INSERT INTO users (username, password_hash, email) VALUES($1, $2, $3);")
    .bind(user.username)
    .bind(password_hash)
    .bind(user.email)
    .execute(pool) 
    .await?;
    Ok(())
}

pub async fn register_user_by_role(
    pool:&PgPool,
    user:RegisterUser,
    password_hash:String,
) -> Result<(), anyhow::Error> {
    sqlx::query(
        "INSERT INTO users (username, password_hash, role, email) VALUES($1, $2, $3, $4);")
    .bind(user.username)
    .bind(password_hash)
    .bind(user.role)
    .bind(user.email)
    .execute(pool) 
    .await?;
    Ok(())
}

pub async fn check_user_by_email(
    pool:&PgPool,
    user:UserData
)-> Result<String, anyhow::Error>{
    let result =sqlx::query_as::<_, DbUser>(
    "SELECT username, role, password_hash FROM users WHERE email= $1;"
    )
    .bind(&user.email)
    .fetch_optional(pool) 
        .await?;
    let Some(db_user) = result.clone() else {
        return Err(anyhow::anyhow!("user not found"))
    };
    Ok(db_user.password_hash)
}

pub async fn check_user_by_username(
    pool:&PgPool,
    user: UserData
)-> Result<String,anyhow::Error>{
    let result = sqlx::query_as::<_, DbUser>(
        "SELECT username, role, password_hash FROM users WHERE username = $1;",
    )
    .bind(&user.username)
    .fetch_optional(pool) 
    .await?;
    let Some(db_user) = result.clone() else {
        return Err(anyhow::anyhow!("user not found"))
    };
    Ok(db_user.password_hash)
}


pub async fn delete_user(
    pool:&PgPool,
    username:&str
)-> Result<(), anyhow::Error>{
    sqlx::query(
        "DELETE FROM users WHERE username =$1;"
    )
    .bind(&username)
    .execute(pool) 
    .await?;
    Ok(())
}

pub async fn update_user_role(
    pool:&PgPool,
    data:UpdateRoleData
)-> Result<(), anyhow::Error>{
    sqlx::query(
        "UPDATE users SET role = $1  WHERE username =$2;")
    .bind(&data.new_role)
    .bind(&data.username)
    .execute(pool) 
    .await?;
    Ok(())
}