use chrono::{DateTime, Utc};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct UserDetail {
    pub user_id: Uuid,
    pub bio: Option<String>,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

pub async fn create_user_detail(
    pool: &PgPool,
    user_id: Uuid,
    bio: Option<&str>,
) -> Result<UserDetail, sqlx::Error> {
    sqlx::query_as!(
        UserDetail,
        r#"
        INSERT INTO user_detail (user_id, bio)
        VALUES ($1, $2)
        RETURNING user_id, bio, created_at, modified_at
        "#,
        user_id,
        bio
    )
    .fetch_one(pool)
    .await
}

pub async fn get_user_detail_by_user_id(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Option<UserDetail>, sqlx::Error> {
    sqlx::query_as!(
        UserDetail,
        r#"
        SELECT user_id, bio, created_at, modified_at
        FROM user_detail
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_optional(pool)
    .await
}

pub async fn update_user_detail(
    pool: &PgPool,
    user_id: Uuid,
    bio: Option<&str>,
) -> Result<UserDetail, sqlx::Error> {
    sqlx::query_as!(
        UserDetail,
        r#"
        UPDATE user_detail
        SET bio = $2, modified_at = NOW()
        WHERE user_id = $1
        RETURNING user_id, bio, created_at, modified_at
        "#,
        user_id,
        bio
    )
    .fetch_one(pool)
    .await
}

pub async fn delete_user_detail(pool: &PgPool, user_id: Uuid) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        DELETE FROM user_detail
        WHERE user_id = $1
        "#,
        user_id
    )
    .execute(pool)
    .await?;
    Ok(result.rows_affected())
}
