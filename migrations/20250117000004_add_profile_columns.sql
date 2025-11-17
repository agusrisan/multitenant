-- Migration: Add profile columns to users table
-- Purpose: Extend users table with profile information (bio, avatar_url)
-- Date: 2025-01-17

-- Add profile columns
ALTER TABLE users
ADD COLUMN IF NOT EXISTS bio TEXT,
ADD COLUMN IF NOT EXISTS avatar_url TEXT;

-- Add constraints for data validation
ALTER TABLE users
ADD CONSTRAINT bio_length CHECK (LENGTH(bio) <= 500);

-- Add comment for documentation
COMMENT ON COLUMN users.bio IS 'User biography/description, max 500 characters';
COMMENT ON COLUMN users.avatar_url IS 'URL to user profile avatar image';

-- Create index for faster profile queries (optional, but recommended)
-- Note: Primary key (id) already indexed, no additional index needed for basic queries
