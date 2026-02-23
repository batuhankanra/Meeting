use sqlx::{PgPool,Result};
use crate::models::user::User;
use uuid::Uuid;


pub struct UserRepository;


impl UserRepository {
    pub async fn create(pool:&PgPool,data:User)->Result<User> {
        sqlx::query_as::<_,User>(
            "INSERT INTO users (username,email,password_hash)
            VALUES ($1,$2,$3) RETURNING *"
        )
        .bind(data.username)
        .bind(data.email)
        .bind(data.password_hash)
        .fetch_one(pool)
        .await
    }
    pub async fn find_by_email(pool:&PgPool,email:&str)->Result<Option<User>>{
        sqlx::query_as::<_,User>("SELECT * FROM users WHERE email=$1")
        .bind(email)
        .fetch_optional(pool)
        .await
    }
    pub async fn find_by_id(pool:&PgPool,id:Uuid)->Result<User>{
        sqlx::query_as::<_,User>("SELECT * FROM users WHERE id=$1")
        .bind(id)
        .fetch_one(pool)
        .await
    }
}


