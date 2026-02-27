use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};


#[derive(Debug,serde::Serialize,serde::Deserialize,sqlx::Type)]
#[sqlx(type_name="user_role",rename_all="lowercase")]
pub enum UserRole{
    Admin,
    User,
    Moderator
}

#[derive(Debug,  Deserialize, Serialize,FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}