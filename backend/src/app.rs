use axum::{Router, routing::get};

use crate::state::AppState;




pub fn create_app(state:AppState)->Router {
    Router::new()
        .route("/api", get(|| async {"ok"}))
        .with_state(state)
}
