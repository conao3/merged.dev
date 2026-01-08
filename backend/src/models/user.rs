use chrono::{DateTime, Utc};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
}

pub async fn create_user(pool: &PgPool) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
        INSERT INTO user_mst DEFAULT VALUES
        RETURNING user_id, created_at
        "#
    )
    .fetch_one(pool)
    .await
}

pub async fn get_user_by_id(pool: &PgPool, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
        SELECT user_id, created_at
        FROM user_mst
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_optional(pool)
    .await
}
