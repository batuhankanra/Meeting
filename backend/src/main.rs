mod db;
mod state;
mod config;
mod app;

use state::AppState;
use std::sync::Arc;
use crate::{config::Config, db::pg_connect::Connect};
#[tokio::main]
async fn main() {
    let config =Config::from_env();

    let pool=Connect::create_pool(&config.database_url).await;

    Connect::ping(&pool).await;
    println!("✅ PostgreSQL bağlantısı OK");

    let state = AppState {
        db: Arc::new(pool),
    };

    let app = app::create_app(state);
    let addr=format!("0.0.0.0:{}",config.server_port);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap();

    println!("🚀 Server running on http://localhost:{}",config.server_port);
    axum::serve(listener, app).await.unwrap();
}