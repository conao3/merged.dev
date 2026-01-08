//! Integration tests for User model CRUD operations.
//!
//! These tests use the `#[sqlx::test]` macro which provides automatic
//! database isolation for each test. Each test receives its own database
//! connection pool with a clean schema applied via fixtures.

#![allow(unused_imports)]

use backend::models::user::{create_user, get_user_by_id, User};
use sqlx::PgPool;
use uuid::Uuid;

// Note: The #[sqlx::test] macro with fixtures will apply the schema.sql
// before each test, ensuring a clean database state.
// Fixtures path is relative to the tests directory.

/// Test that a user can be created successfully.
///
/// Verifies that:
/// - create_user returns the created user
/// - A user_id is auto-generated
#[sqlx::test(fixtures("schema"))]
async fn test_create_user(pool: PgPool) {
    let user = create_user(&pool).await.expect("Failed to create user");

    assert!(!user.user_id.is_nil());
}

/// Test that a user can be retrieved by user_id.
///
/// Verifies that:
/// - get_user_by_id returns Some(user) when user exists
/// - The returned user has the correct user_id
#[sqlx::test(fixtures("schema"))]
async fn test_get_user_by_id(pool: PgPool) {
    let created_user = create_user(&pool).await.expect("Failed to create user");

    let retrieved_user = get_user_by_id(&pool, created_user.user_id)
        .await
        .expect("Failed to get user by id")
        .expect("User should exist");

    assert_eq!(retrieved_user.user_id, created_user.user_id);
}

/// Test that get_user_by_id returns None for non-existent user.
///
/// Verifies that:
/// - get_user_by_id returns Ok(None) when user does not exist
/// - No error is raised for missing user (it's an expected case)
#[sqlx::test(fixtures("schema"))]
async fn test_get_user_not_found(pool: PgPool) {
    // Use a UUID that definitely doesn't exist
    let non_existent_id = Uuid::new_v4();

    let result = get_user_by_id(&pool, non_existent_id)
        .await
        .expect("Database query should succeed");

    assert!(
        result.is_none(),
        "get_user_by_id should return None for non-existent user"
    );
}

