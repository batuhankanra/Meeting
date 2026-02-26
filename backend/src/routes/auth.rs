use axum::{Router, routing::{ post}};

use crate::{handlers::register::register_handler, state::AppState};




pub fn auth()->Router<AppState>{
    Router::new()
        .route("/register", post(register_handler))
}