use dotenvy::dotenv;
use sqlx::{ PgPool, Pool, Postgres };

pub async fn establish_connection() -> Result<Pool<Postgres>, sqlx::Error> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgPool::connect(&database_url).await
}
