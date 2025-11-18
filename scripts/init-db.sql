-- AnimaForge Database Initialization Script
-- This script creates the complete database schema

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Users table
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    display_name VARCHAR(100),
    bio TEXT,
    avatar_url TEXT,
    is_verified BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Animations table
CREATE TABLE IF NOT EXISTS animations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(200) NOT NULL,
    description TEXT,
    file_url TEXT NOT NULL,
    thumbnail_url TEXT,
    source_code TEXT,
    duration DECIMAL(5,2),
    view_count INTEGER DEFAULT 0,
    download_count INTEGER DEFAULT 0,
    like_count INTEGER DEFAULT 0,
    is_public BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Animation tags table
CREATE TABLE IF NOT EXISTS animation_tags (
    animation_id UUID REFERENCES animations(id) ON DELETE CASCADE,
    tag VARCHAR(50) NOT NULL,
    PRIMARY KEY (animation_id, tag)
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_animations_user ON animations(user_id);
CREATE INDEX IF NOT EXISTS idx_animations_public ON animations(is_public) WHERE is_public = TRUE;
CREATE INDEX IF NOT EXISTS idx_animations_created ON animations(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_tags ON animation_tags(tag);
CREATE INDEX IF NOT EXISTS idx_animations_likes ON animations(like_count DESC);
CREATE INDEX IF NOT EXISTS idx_animations_views ON animations(view_count DESC);

-- Function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Triggers for automatic timestamp updates
DROP TRIGGER IF EXISTS update_users_updated_at ON users;
CREATE TRIGGER update_users_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS update_animations_updated_at ON animations;
CREATE TRIGGER update_animations_updated_at
    BEFORE UPDATE ON animations
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
