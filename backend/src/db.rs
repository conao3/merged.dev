use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::env;

/// Initialize a PostgreSQL connection pool from DATABASE_URL environment variable.
///
/// # Errors
///
/// Returns an error if:
/// - DATABASE_URL environment variable is not set
/// - Connection to the database fails
pub async fn init_pool() -> Result<PgPool, sqlx::Error> {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}
