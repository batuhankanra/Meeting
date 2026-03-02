pub mod auth;
pub mod meeting;

use axum::{Router};

use crate::{ routes::{auth::auth, meeting::meeting_route}, state::AppState};



pub fn main_routes()->Router<AppState>{
    Router::new()
        .nest("/auth",auth() )
        .nest("/meeting", meeting_route())
        
        
}