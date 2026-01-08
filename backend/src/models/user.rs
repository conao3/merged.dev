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

/// Retrieves a user by their ID.
///
/// # Arguments
///
/// * `pool` - Database connection pool
/// * `id` - The unique identifier of the user to retrieve
///
/// # Returns
///
/// `Some(User)` if a user with the given ID exists, `None` otherwise.
///
/// # Errors
///
/// Returns an error if the database connection fails.
pub async fn get_user_by_id(pool: &PgPool, id: i32) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
        SELECT id, name, email, created_at
        FROM users
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}
