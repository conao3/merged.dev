use chrono::{DateTime, Utc};
use sqlx::FromRow;

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
