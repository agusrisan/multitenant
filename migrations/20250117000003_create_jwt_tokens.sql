-- Create jwt_tokens table
-- This table stores JWT tokens for API authentication and allows for token revocation

-- Create token_type enum
CREATE TYPE token_type AS ENUM ('access', 'refresh');

-- Create jwt_tokens table
CREATE TABLE jwt_tokens (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_type token_type NOT NULL,
    jti UUID NOT NULL UNIQUE,  -- JWT ID for revocation
    expires_at TIMESTAMPTZ NOT NULL,
    revoked BOOLEAN NOT NULL DEFAULT FALSE,
    revoked_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for better query performance
CREATE INDEX idx_jwt_tokens_user_id ON jwt_tokens(user_id);
CREATE INDEX idx_jwt_tokens_jti ON jwt_tokens(jti);
CREATE INDEX idx_jwt_tokens_expires_at ON jwt_tokens(expires_at);
CREATE INDEX idx_jwt_tokens_revoked ON jwt_tokens(revoked) WHERE NOT revoked;

-- Add check constraint for revoked_at
ALTER TABLE jwt_tokens
ADD CONSTRAINT check_revoked_at
CHECK (
    (revoked = TRUE AND revoked_at IS NOT NULL) OR
    (revoked = FALSE AND revoked_at IS NULL)
);

-- Add comments for documentation
COMMENT ON TABLE jwt_tokens IS 'JWT token storage for API authentication and revocation';
COMMENT ON COLUMN jwt_tokens.id IS 'UUID v7 primary key';
COMMENT ON COLUMN jwt_tokens.user_id IS 'Foreign key to users table';
COMMENT ON COLUMN jwt_tokens.token_type IS 'Type of token (access or refresh)';
COMMENT ON COLUMN jwt_tokens.jti IS 'JWT ID claim for token revocation';
COMMENT ON COLUMN jwt_tokens.expires_at IS 'Token expiration timestamp';
COMMENT ON COLUMN jwt_tokens.revoked IS 'Whether token has been revoked';
COMMENT ON COLUMN jwt_tokens.revoked_at IS 'Timestamp when token was revoked';
COMMENT ON COLUMN jwt_tokens.created_at IS 'Token creation timestamp';
