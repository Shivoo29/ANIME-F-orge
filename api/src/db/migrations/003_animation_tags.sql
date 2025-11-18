-- Create animation_tags table
CREATE TABLE IF NOT EXISTS animation_tags (
    animation_id UUID NOT NULL REFERENCES animations(id) ON DELETE CASCADE,
    tag VARCHAR(50) NOT NULL,
    PRIMARY KEY (animation_id, tag)
);

-- Create index on tag for faster tag-based searches
CREATE INDEX IF NOT EXISTS idx_animation_tags_tag ON animation_tags(tag);

-- Create index on animation_id for faster animation tag lookups
CREATE INDEX IF NOT EXISTS idx_animation_tags_animation_id ON animation_tags(animation_id);
