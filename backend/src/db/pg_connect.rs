use sqlx::{postgres::PgPoolOptions,PgPool};
use std::time::Duration;



pub struct Connect;
impl Connect {
    pub async fn create_pool(db_url:&str)->PgPool{
        PgPoolOptions::new()
            .max_connections(15)
            .acquire_timeout(Duration::from_secs(5))
            .connect(db_url)
            .await
            .expect("Postgres bağlantısı kurulamadı")
    }
    pub async fn ping(pool:&PgPool) {
        match sqlx::query("SELECT 1").execute(pool).await {
            Ok(_)=>println!("Postgres db connected"),
            Err(e)=>panic!("the database is not responding:{}",e)
        }
    }
}