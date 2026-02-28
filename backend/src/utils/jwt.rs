use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Serialize,Deserialize};
use uuid::Uuid;

use crate::{config, models::user::{ UserRole}};


#[derive(Debug,Serialize,Deserialize)]
pub struct Claims{
    pub sub:Uuid,
    pub role:UserRole,
    pub exp:i64,
    pub iat:i64

}

impl  Claims {
    pub fn new(user_id:Uuid,role:UserRole)->Self{
        let iat =Utc::now();
        let exp=iat+Duration::hours(24);
        Self {
            sub:user_id,
            iat:iat.timestamp(),
            exp:exp.timestamp(),
            role
        }
    }
}

pub fn create_token(user_id:Uuid,role:UserRole)->Result<String, jsonwebtoken::errors::Error>{
    let claims: Claims=Claims::new(user_id, role);
    let secret: String=config::Config::from_env().jwt_secret;
    encode(
        &Header::default(),
         &claims,
    &EncodingKey::from_secret(secret.as_ref()))
}

