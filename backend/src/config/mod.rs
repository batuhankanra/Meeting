use std::env;
use dotenvy::dotenv;


pub struct Config{
    pub database_url :String,
    pub server_port:String,
    pub jwt_secret:String
}

impl Config{
    pub fn from_env()->Self{
        dotenv().ok();
        Self { 
            database_url: env::var("DATABASE_URL").expect("Database url not found"), 
            server_port: env::var("PORT").unwrap_or_else(|_| "3000".to_string()),
            jwt_secret:env::var("JWT_SECRET").expect("jwt_none")
        }
    }
}