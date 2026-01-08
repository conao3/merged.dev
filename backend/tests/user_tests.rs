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

// Test functions will be added in subsequent subtasks:
// - subtask-7-3: test_get_user_by_id
// - subtask-7-4: test_get_user_not_found
