use sqlx::{postgres::PgPoolOptions,PgPool};
use std::time::Duration;


pub async fn create_pool(db_url:&str)->PgPool{
    PgPoolOptions::new()
        .max_connections(15)
        .acquire_timeout(Duration::from_secs(5))
        .connect(db_url)
        .await
        .expect("Postgres bağlantısı kurulamadı")
}