


use axum::{Router, routing::get};

use crate::state::AppState;



pub fn main_routes()->Router<AppState>{
    Router::new()
        .route("/register", get(|| async {"ok"}))
        
}