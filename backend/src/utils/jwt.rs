use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Serialize,Deserialize};
use uuid::Uuid;

use crate::{config, models::user::{ UserRole}};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims{
    pub sub:Uuid,
    pub role:UserRole,
    pub exp:i64,
    pub iat:i64

}

impl  Claims {
    pub fn new(user_id:Uuid,role:UserRole)->Self{
        let iat =Utc::now();
        let exp=iat+Duration::hours(2);
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

pub fn verify_token(token:&str)->Result<Claims,jsonwebtoken::errors::Error>{
    let secret =config::Config::from_env().jwt_secret;
    let validation=Validation::default();
    let token_data=decode(token,&DecodingKey::from_secret(secret.as_ref()), &validation)?;

    Ok(token_data.claims)
}