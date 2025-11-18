-- Create animations table
CREATE TABLE IF NOT EXISTS animations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(200) NOT NULL,
    description TEXT,
    file_url TEXT NOT NULL,
    thumbnail_url TEXT,
    source_code TEXT,
    duration DECIMAL(5,2),
    views INTEGER DEFAULT 0 NOT NULL,
    downloads INTEGER DEFAULT 0 NOT NULL,
    likes INTEGER DEFAULT 0 NOT NULL,
    is_public BOOLEAN DEFAULT TRUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- Create index on user_id for faster user animation lookups
CREATE INDEX IF NOT EXISTS idx_animations_user_id ON animations(user_id);

-- Create index on created_at for sorting by date
CREATE INDEX IF NOT EXISTS idx_animations_created_at ON animations(created_at DESC);

-- Create index on views for trending animations
CREATE INDEX IF NOT EXISTS idx_animations_views ON animations(views DESC);

-- Create index on likes for featured animations
CREATE INDEX IF NOT EXISTS idx_animations_likes ON animations(likes DESC);

-- Create index on is_public for filtering public animations
CREATE INDEX IF NOT EXISTS idx_animations_public ON animations(is_public);

-- Create index on downloads for tracking popular downloads
CREATE INDEX IF NOT EXISTS idx_animations_downloads ON animations(downloads DESC);

-- Create index for full-text search on title and description
CREATE INDEX IF NOT EXISTS idx_animations_search ON animations USING gin(to_tsvector('english', title || ' ' || COALESCE(description, '')));

-- Create trigger to automatically update updated_at
CREATE TRIGGER update_animations_updated_at
    BEFORE UPDATE ON animations
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
