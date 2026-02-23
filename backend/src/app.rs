use axum::{Router, http::{header::{AUTHORIZATION, CONTENT_TYPE},Method}, middleware};
use tower_http::cors::CorsLayer;

use crate::{middleware::logger::logger_middleware, routes::main_routes, state::AppState};




pub fn create_app(state:AppState)->Router {
    let cors =CorsLayer::new()
        .allow_methods([Method::GET,Method::POST,Method::PUT,Method::DELETE])
        .allow_origin("http://localhost:3000".parse::<axum::http::HeaderValue>().unwrap())
        .allow_headers([CONTENT_TYPE,AUTHORIZATION]);
     
    Router::new()
        .nest("/api", main_routes())
        .layer(middleware::from_fn(logger_middleware))
        .layer(cors)
        .with_state(state)
}
