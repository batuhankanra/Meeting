pub mod auth;


use axum::{Router};

use crate::{routes::auth::auth, state::AppState};



pub fn main_routes()->Router<AppState>{
    Router::new()
        .nest("/auth",auth() )
        
}