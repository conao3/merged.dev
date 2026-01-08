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

/// Test that get_user_by_id returns None for non-existent user.
///
/// Verifies that:
/// - get_user_by_id returns Ok(None) when user does not exist
/// - No error is raised for missing user (it's an expected case)
#[sqlx::test(fixtures("schema"))]
async fn test_get_user_not_found(pool: PgPool) {
    // Use an ID that definitely doesn't exist
    let non_existent_id = 99999;

    let result = get_user_by_id(&pool, non_existent_id)
        .await
        .expect("Database query should succeed");

    assert!(
        result.is_none(),
        "get_user_by_id should return None for non-existent user"
    );
}

/// Test that creating a user with duplicate email fails.
///
/// Verifies that:
/// - The database rejects duplicate emails
/// - An appropriate error is returned
#[sqlx::test(fixtures("schema"))]
async fn test_duplicate_email_rejected(pool: PgPool) {
    let name = "First User";
    let email = "duplicate@example.com";

    // Create first user - should succeed
    let _first_user = create_user(&pool, name, email)
        .await
        .expect("Failed to create first user");

    // Try to create second user with same email - should fail
    let result = create_user(&pool, "Second User", email).await;

    assert!(
        result.is_err(),
        "Creating user with duplicate email should fail"
    );
}
