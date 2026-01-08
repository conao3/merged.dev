use chrono::{DateTime, Utc};
use sqlx::{FromRow, PgPool};

/// User model representing a row in the users table.
#[derive(Debug, Clone, FromRow)]
pub struct User {
    /// Unique identifier for the user (auto-generated).
    pub id: i32,
    /// User's display name.
    pub name: String,
    /// User's email address (unique).
    pub email: String,
    /// Timestamp when the user was created.
    pub created_at: DateTime<Utc>,
}

/// Creates a new user in the database.
///
/// # Arguments
///
/// * `pool` - Database connection pool
/// * `name` - User's display name
/// * `email` - User's email address (must be unique)
///
/// # Returns
///
/// The created user with auto-generated id and timestamp.
///
/// # Errors
///
/// Returns an error if:
/// - A user with the same email already exists
/// - Database connection fails
pub async fn create_user(pool: &PgPool, name: &str, email: &str) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (name, email)
        VALUES ($1, $2)
        RETURNING id, name, email, created_at
        "#,
        name,
        email
    )
    .fetch_one(pool)
    .await
}
