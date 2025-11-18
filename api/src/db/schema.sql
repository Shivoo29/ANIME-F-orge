-- AnimaForge Database Schema
-- PostgreSQL Database Schema for Animation Sharing Platform

-- ============================================
-- Users Table
-- ============================================
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    display_name VARCHAR(100),
    bio TEXT,
    avatar_url TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- ============================================
-- Animations Table
-- ============================================
CREATE TABLE IF NOT EXISTS animations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
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

-- ============================================
-- Animation Tags Table
-- ============================================
CREATE TABLE IF NOT EXISTS animation_tags (
    animation_id UUID REFERENCES animations(id) ON DELETE CASCADE,
    tag VARCHAR(50) NOT NULL,
    PRIMARY KEY (animation_id, tag)
);

-- ============================================
-- Indexes for Performance
-- ============================================

-- User indexes
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);

-- Animation indexes
CREATE INDEX IF NOT EXISTS idx_animations_user ON animations(user_id);
CREATE INDEX IF NOT EXISTS idx_animations_public ON animations(is_public);
CREATE INDEX IF NOT EXISTS idx_animations_created_at ON animations(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_animations_views ON animations(view_count DESC);
CREATE INDEX IF NOT EXISTS idx_animations_likes ON animations(like_count DESC);

-- Tag indexes
CREATE INDEX IF NOT EXISTS idx_tags ON animation_tags(tag);

-- Full-text search index
CREATE INDEX IF NOT EXISTS idx_animations_search
    ON animations USING gin(to_tsvector('english', title || ' ' || COALESCE(description, '')));

-- ============================================
-- Triggers for Auto-updating Timestamps
-- ============================================

-- Function to update the updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Trigger for users table
CREATE TRIGGER IF NOT EXISTS update_users_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Trigger for animations table
CREATE TRIGGER IF NOT EXISTS update_animations_updated_at
    BEFORE UPDATE ON animations
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
