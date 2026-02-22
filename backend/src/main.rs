mod db;
mod state;

use axum::{routing::get, Router};
use dotenvy::dotenv;
use std::env;
use state::AppState;
use std::sync::Arc;
use crate::db::pg_connect::{ create_pool};
#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL env değişkeni yok");

    let pool = create_pool(&database_url).await;

    // Test query (çok önemli)
    sqlx::query("SELECT 1")
        .execute(&pool)
        .await
        .expect("❌ DB test query failed");

    println!("✅ PostgreSQL bağlantısı OK");

    let state = AppState {
        db: Arc::new(pool),
    };

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("🚀 Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}