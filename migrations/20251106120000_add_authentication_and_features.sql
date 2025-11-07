-- Create users table for authentication
CREATE TABLE users (
    id              BIGSERIAL PRIMARY KEY,
    username        TEXT UNIQUE NOT NULL,
    email           TEXT UNIQUE NOT NULL,
    password_hash   TEXT NOT NULL,
    display_name    TEXT NOT NULL,
    bio             TEXT,
    avatar_url      TEXT,
    role            TEXT NOT NULL DEFAULT 'user' CHECK (role IN ('user', 'author', 'admin')),
    email_verified  BOOLEAN NOT NULL DEFAULT FALSE,
    is_active       BOOLEAN NOT NULL DEFAULT TRUE,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create categories table
CREATE TABLE categories (
    id              BIGSERIAL PRIMARY KEY,
    name            TEXT UNIQUE NOT NULL,
    slug            TEXT UNIQUE NOT NULL,
    description     TEXT,
    color           TEXT DEFAULT '#3B82F6',
    is_active       BOOLEAN NOT NULL DEFAULT TRUE,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create tags table
CREATE TABLE tags (
    id              BIGSERIAL PRIMARY KEY,
    name            TEXT UNIQUE NOT NULL,
    slug            TEXT UNIQUE NOT NULL,
    description     TEXT,
    usage_count     INTEGER NOT NULL DEFAULT 0,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Add author and category references to posts
ALTER TABLE posts 
ADD COLUMN author_id BIGINT REFERENCES users(id) ON DELETE SET NULL,
ADD COLUMN category_id BIGINT REFERENCES categories(id) ON DELETE SET NULL,
ADD COLUMN featured BOOLEAN NOT NULL DEFAULT FALSE,
ADD COLUMN views_count INTEGER NOT NULL DEFAULT 0,
ADD COLUMN likes_count INTEGER NOT NULL DEFAULT 0,
ADD COLUMN meta_title TEXT,
ADD COLUMN meta_description TEXT,
ADD COLUMN reading_time_minutes INTEGER;

-- Create post_tags junction table
CREATE TABLE post_tags (
    post_id     BIGINT NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    tag_id      BIGINT NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (post_id, tag_id)
);

-- Create user sessions table
CREATE TABLE user_sessions (
    id              TEXT PRIMARY KEY,
    user_id         BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    expires_at      TIMESTAMPTZ NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_accessed   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    user_agent      TEXT,
    ip_address      INET
);

-- Create password reset tokens table
CREATE TABLE password_reset_tokens (
    id              BIGSERIAL PRIMARY KEY,
    user_id         BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token           TEXT UNIQUE NOT NULL,
    expires_at      TIMESTAMPTZ NOT NULL,
    used            BOOLEAN NOT NULL DEFAULT FALSE,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create email verification tokens table
CREATE TABLE email_verification_tokens (
    id              BIGSERIAL PRIMARY KEY,
    user_id         BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token           TEXT UNIQUE NOT NULL,
    expires_at      TIMESTAMPTZ NOT NULL,
    used            BOOLEAN NOT NULL DEFAULT FALSE,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create content revisions for version history
CREATE TABLE post_revisions (
    id                  BIGSERIAL PRIMARY KEY,
    post_id             BIGINT NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    title               TEXT NOT NULL,
    summary             TEXT NOT NULL,
    body_markdown       TEXT NOT NULL,
    body_html           TEXT NOT NULL,
    revision_number     INTEGER NOT NULL,
    change_summary      TEXT,
    created_by          BIGINT REFERENCES users(id) ON DELETE SET NULL,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(post_id, revision_number)
);

-- Indexes for performance
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_role ON users(role);
CREATE INDEX idx_users_is_active ON users(is_active);

CREATE INDEX idx_posts_author_id ON posts(author_id);
CREATE INDEX idx_posts_category_id ON posts(category_id);
CREATE INDEX idx_posts_featured ON posts(featured);
CREATE INDEX idx_posts_views_count ON posts(views_count DESC);
CREATE INDEX idx_posts_likes_count ON posts(likes_count DESC);

CREATE INDEX idx_categories_slug ON categories(slug);
CREATE INDEX idx_categories_is_active ON categories(is_active);

CREATE INDEX idx_tags_slug ON tags(slug);
CREATE INDEX idx_tags_usage_count ON tags(usage_count DESC);

CREATE INDEX idx_post_tags_post_id ON post_tags(post_id);
CREATE INDEX idx_post_tags_tag_id ON post_tags(tag_id);

CREATE INDEX idx_user_sessions_user_id ON user_sessions(user_id);
CREATE INDEX idx_user_sessions_expires_at ON user_sessions(expires_at);

CREATE INDEX idx_password_reset_tokens_user_id ON password_reset_tokens(user_id);
CREATE INDEX idx_password_reset_tokens_token ON password_reset_tokens(token);
CREATE INDEX idx_password_reset_tokens_expires_at ON password_reset_tokens(expires_at);

CREATE INDEX idx_email_verification_tokens_user_id ON email_verification_tokens(user_id);
CREATE INDEX idx_email_verification_tokens_token ON email_verification_tokens(token);

CREATE INDEX idx_post_revisions_post_id ON post_revisions(post_id);

-- Full-text search index for posts
CREATE INDEX idx_posts_search ON posts USING GIN(to_tsvector('english', title || ' ' || summary || ' ' || body_markdown));

-- Insert default admin user (password: admin123 - should be changed immediately)
INSERT INTO users (username, email, password_hash, display_name, role, email_verified) 
VALUES (
    'admin', 
    'admin@taleji.com', 
    '$2b$12$LQv3c1yqBWVHxkd0LQ4YNu5JDgYJm5Z8X9vQ2Kx7Y8WZvQ3Nx5dYW', -- bcrypt hash of 'admin123'
    'Administrator', 
    'admin', 
    true
);

-- Insert default categories
INSERT INTO categories (name, slug, description, color) VALUES
    ('Technology', 'technology', 'Articles about technology, programming, and innovation', '#3B82F6'),
    ('Science', 'science', 'Scientific research and discoveries', '#10B981'),
    ('Opinion', 'opinion', 'Opinion pieces and editorial content', '#F59E0B'),
    ('Tutorial', 'tutorial', 'How-to guides and tutorials', '#8B5CF6'),
    ('Research', 'research', 'Research papers and academic content', '#EF4444');

-- Insert default tags
INSERT INTO tags (name, slug, description) VALUES
    ('Rust', 'rust', 'Content related to Rust programming language'),
    ('WebAssembly', 'webassembly', 'WebAssembly technology and applications'),
    ('Leptos', 'leptos', 'Leptos web framework'),
    ('Performance', 'performance', 'Performance optimization and benchmarks'),
    ('Security', 'security', 'Security best practices and vulnerabilities'),
    ('Tutorial', 'tutorial', 'Step-by-step learning guides'),
    ('Beginner', 'beginner', 'Content suitable for beginners'),
    ('Advanced', 'advanced', 'Advanced technical content');