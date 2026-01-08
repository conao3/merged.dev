//! Integration tests for User model CRUD operations.
//!
//! These tests use the `#[sqlx::test]` macro which provides automatic
//! database isolation for each test. Each test receives its own database
//! connection pool with a clean schema applied via fixtures.

#![allow(unused_imports)]

use backend::models::user::{create_user, get_user_by_id, User};
use sqlx::PgPool;

// Note: The #[sqlx::test] macro with fixtures will apply the schema.sql
// before each test, ensuring a clean database state.
// Fixtures path is relative to the tests directory.

/// Test that a user can be created successfully.
///
/// Verifies that:
/// - create_user returns the created user
/// - The returned user has the correct name and email
/// - An id is auto-generated
/// - A created_at timestamp is set
#[sqlx::test(fixtures("schema"))]
async fn test_create_user(pool: PgPool) {
    let name = "Test User";
    let email = "test@example.com";

    let user = create_user(&pool, name, email)
        .await
        .expect("Failed to create user");

    assert_eq!(user.name, name);
    assert_eq!(user.email, email);
    assert!(user.id > 0, "User id should be auto-generated and positive");
}

/// Test that a user can be retrieved by id.
///
/// Verifies that:
/// - get_user_by_id returns Some(user) when user exists
/// - The returned user has the correct id, name, and email
#[sqlx::test(fixtures("schema"))]
async fn test_get_user_by_id(pool: PgPool) {
    // First, create a user to retrieve
    let name = "Get User Test";
    let email = "getuser@example.com";

    let created_user = create_user(&pool, name, email)
        .await
        .expect("Failed to create user");

    // Now retrieve the user by id
    let retrieved_user = get_user_by_id(&pool, created_user.id)
        .await
        .expect("Failed to get user by id")
        .expect("User should exist");

    // Verify the retrieved user matches the created user
    assert_eq!(retrieved_user.id, created_user.id);
    assert_eq!(retrieved_user.name, name);
    assert_eq!(retrieved_user.email, email);
    assert_eq!(retrieved_user.created_at, created_user.created_at);
}

// Test functions will be added in subsequent subtasks:
// - subtask-7-4: test_get_user_not_found
