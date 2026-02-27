use axum::{Router, routing::{ post}};

use crate::{handlers::{login::login_handler, register::register_handler}, state::AppState};




pub fn auth()->Router<AppState>{
    Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
}