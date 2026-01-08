#![allow(unused_imports)]

use backend::models::user::{User, create_user};
use backend::models::user_detail::{
    UserDetail, create_user_detail, delete_user_detail, get_user_detail_by_user_id,
    update_user_detail,
};
use sqlx::PgPool;
use uuid::Uuid;

#[sqlx::test(fixtures("schema"))]
async fn test_create_user_detail(pool: PgPool) {
    let user = create_user(&pool).await.expect("Failed to create user");

    let bio = Some("Hello, I am a test user!");

    let user_detail = create_user_detail(&pool, user.user_id, bio)
        .await
        .expect("Failed to create user detail");

    assert_eq!(user_detail.user_id, user.user_id);
    assert_eq!(user_detail.bio, bio.map(String::from));
}

#[sqlx::test(fixtures("schema"))]
async fn test_get_user_detail_by_user_id(pool: PgPool) {
    let user = create_user(&pool).await.expect("Failed to create user");

    let bio = Some("Hello!");

    let created_detail = create_user_detail(&pool, user.user_id, bio)
        .await
        .expect("Failed to create user detail");

    let retrieved_detail = get_user_detail_by_user_id(&pool, user.user_id)
        .await
        .expect("Failed to get user detail")
        .expect("User detail should exist");

    assert_eq!(retrieved_detail.user_id, created_detail.user_id);
    assert_eq!(retrieved_detail.bio, created_detail.bio);
}

#[sqlx::test(fixtures("schema"))]
async fn test_update_user_detail(pool: PgPool) {
    let user = create_user(&pool).await.expect("Failed to create user");

    create_user_detail(&pool, user.user_id, Some("Old bio"))
        .await
        .expect("Failed to create user detail");

    let new_bio = Some("Updated bio");

    let updated_detail = update_user_detail(&pool, user.user_id, new_bio)
        .await
        .expect("Failed to update user detail");

    assert_eq!(updated_detail.bio, new_bio.map(String::from));
}

#[sqlx::test(fixtures("schema"))]
async fn test_delete_user_detail(pool: PgPool) {
    let user = create_user(&pool).await.expect("Failed to create user");

    create_user_detail(&pool, user.user_id, Some("Bio"))
        .await
        .expect("Failed to create user detail");

    let rows_affected = delete_user_detail(&pool, user.user_id)
        .await
        .expect("Failed to delete user detail");

    assert_eq!(rows_affected, 1);

    let result = get_user_detail_by_user_id(&pool, user.user_id)
        .await
        .expect("Database query should succeed");

    assert!(result.is_none());
}

#[sqlx::test(fixtures("schema"))]
async fn test_cascade_delete_user_detail(pool: PgPool) {
    let user = create_user(&pool).await.expect("Failed to create user");

    create_user_detail(&pool, user.user_id, Some("Bio"))
        .await
        .expect("Failed to create user detail");

    sqlx::query!("DELETE FROM user_mst WHERE user_id = $1", user.user_id)
        .execute(&pool)
        .await
        .expect("Failed to delete user");

    let result = get_user_detail_by_user_id(&pool, user.user_id)
        .await
        .expect("Database query should succeed");

    assert!(result.is_none());
}
