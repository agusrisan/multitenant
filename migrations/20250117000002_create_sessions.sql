-- Create sessions table
-- This table stores active user sessions for web authentication

CREATE TABLE sessions (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    csrf_token VARCHAR(255) NOT NULL,
    ip_address INET,
    user_agent TEXT,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for better query performance
CREATE INDEX idx_sessions_user_id ON sessions(user_id);
CREATE INDEX idx_sessions_expires_at ON sessions(expires_at);
CREATE INDEX idx_sessions_csrf_token ON sessions(csrf_token);

-- Add comments for documentation
COMMENT ON TABLE sessions IS 'User session storage for web authentication';
COMMENT ON COLUMN sessions.id IS 'UUID v7 primary key (session identifier)';
COMMENT ON COLUMN sessions.user_id IS 'Foreign key to users table';
COMMENT ON COLUMN sessions.csrf_token IS 'CSRF protection token';
COMMENT ON COLUMN sessions.ip_address IS 'IP address of the client';
COMMENT ON COLUMN sessions.user_agent IS 'User agent string of the client';
COMMENT ON COLUMN sessions.expires_at IS 'Session expiration timestamp';
COMMENT ON COLUMN sessions.created_at IS 'Session creation timestamp';
COMMENT ON COLUMN sessions.updated_at IS 'Last update timestamp';
