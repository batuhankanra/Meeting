pub mod auth;


use axum::{Router, routing::get};

use crate::{handlers::all_services::all_services, routes::auth::auth, state::AppState};



pub fn main_routes()->Router<AppState>{
    Router::new()
        .nest("/auth",auth() )
        .route("/all", get(all_services))
        
}