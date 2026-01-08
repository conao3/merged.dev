-- Schema fixture for integration tests
-- This is applied before each test by sqlx::test macro

CREATE TABLE user_mst (
    user_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL
);

CREATE TABLE user_detail (
    user_id UUID PRIMARY KEY REFERENCES user_mst(user_id) ON DELETE CASCADE,
    bio TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL,
    modified_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL
);
