use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use sqlx::FromRow;
#[cfg(feature = "ssr")]
use validator::{Validate, ValidationError};

/// User roles for authorization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "text"))]
pub enum UserRole {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "author")]
    Author,
    #[serde(rename = "admin")]
    Admin,
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::User => write!(f, "user"),
            UserRole::Author => write!(f, "author"),
            UserRole::Admin => write!(f, "admin"),
        }
    }
}

/// User model
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub display_name: String,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub role: UserRole,
    pub email_verified: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Public user profile (without sensitive data)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: i64,
    pub username: String,
    pub display_name: String,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub role: UserRole,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserProfile {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            display_name: user.display_name,
            bio: user.bio,
            avatar_url: user.avatar_url,
            role: user.role,
            created_at: user.created_at,
        }
    }
}

/// Category model
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub color: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Tag model
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub usage_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Enhanced Post model
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Post {
    pub id: i64,
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub body_markdown: String,
    pub body_html: String,
    pub author_id: Option<i64>,
    pub category_id: Option<i64>,
    pub featured: bool,
    pub views_count: i32,
    pub likes_count: i32,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub reading_time_minutes: Option<i32>,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Post with related data for display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostWithMetadata {
    pub post: Post,
    pub author: Option<UserProfile>,
    pub category: Option<Category>,
    pub tags: Vec<Tag>,
}

/// Post revision for version history
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct PostRevision {
    pub id: i64,
    pub post_id: i64,
    pub title: String,
    pub summary: String,
    pub body_markdown: String,
    pub body_html: String,
    pub revision_number: i32,
    pub change_summary: Option<String>,
    pub created_by: Option<i64>,
    pub created_at: DateTime<Utc>,
}

/// Input model for creating new posts with validation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(Validate))]
pub struct CreatePostInput {
    #[cfg_attr(feature = "ssr", validate(length(min = 1, max = 100, message = "Slug must be between 1 and 100 characters")))]
    #[cfg_attr(feature = "ssr", validate(regex(path = "SLUG_REGEX", message = "Slug can only contain lowercase letters, numbers, and hyphens")))]
    pub slug: String,

    #[cfg_attr(feature = "ssr", validate(length(min = 1, max = 200, message = "Title must be between 1 and 200 characters")))]
    pub title: String,

    #[cfg_attr(feature = "ssr", validate(length(min = 1, max = 500, message = "Summary must be between 1 and 500 characters")))]
    pub summary: String,

    #[cfg_attr(feature = "ssr", validate(length(min = 1, max = 100000, message = "Body must be between 1 and 100000 characters")))]
    pub body_markdown: String,

    pub published: bool,
}

/// Input model for updating posts with validation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(Validate))]
pub struct UpdatePostInput {
    #[cfg_attr(feature = "ssr", validate(length(min = 1, max = 200, message = "Title must be between 1 and 200 characters")))]
    pub title: Option<String>,

    #[cfg_attr(feature = "ssr", validate(length(min = 1, max = 500, message = "Summary must be between 1 and 500 characters")))]
    pub summary: Option<String>,

    #[cfg_attr(feature = "ssr", validate(length(min = 1, max = 100000, message = "Body must be between 1 and 100000 characters")))]
    pub body_markdown: Option<String>,

    pub published: Option<bool>,
}

/// Authentication input models
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(Validate))]
pub struct LoginInput {
    #[cfg_attr(feature = "ssr", validate(length(min = 1, message = "Email or username is required")))]
    pub email_or_username: String,

    #[cfg_attr(feature = "ssr", validate(length(min = 1, message = "Password is required")))]
    pub password: String,

    pub remember_me: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(Validate))]
pub struct RegisterInput {
    #[cfg_attr(feature = "ssr", validate(length(min = 3, max = 50, message = "Username must be 3-50 characters")))]
    #[cfg_attr(feature = "ssr", validate(regex(path = "USERNAME_REGEX", message = "Username can only contain letters, numbers, and underscores")))]
    pub username: String,

    #[cfg_attr(feature = "ssr", validate(email(message = "Invalid email address")))]
    pub email: String,

    #[cfg_attr(feature = "ssr", validate(length(min = 8, message = "Password must be at least 8 characters")))]
    pub password: String,

    #[cfg_attr(feature = "ssr", validate(must_match(other = "password", message = "Passwords do not match")))]
    pub confirm_password: String,

    #[cfg_attr(feature = "ssr", validate(length(min = 1, max = 100, message = "Display name must be 1-100 characters")))]
    pub display_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(Validate))]
pub struct UpdateProfileInput {
    #[cfg_attr(feature = "ssr", validate(length(min = 1, max = 100, message = "Display name must be 1-100 characters")))]
    pub display_name: Option<String>,

    #[cfg_attr(feature = "ssr", validate(length(max = 500, message = "Bio must be less than 500 characters")))]
    pub bio: Option<String>,

    #[cfg_attr(feature = "ssr", validate(url(message = "Invalid avatar URL")))]
    pub avatar_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(Validate))]
pub struct ChangePasswordInput {
    #[cfg_attr(feature = "ssr", validate(length(min = 1, message = "Current password is required")))]
    pub current_password: String,

    #[cfg_attr(feature = "ssr", validate(length(min = 8, message = "New password must be at least 8 characters")))]
    pub new_password: String,

    #[cfg_attr(feature = "ssr", validate(must_match(other = "new_password", message = "Passwords do not match")))]
    pub confirm_password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub user: UserProfile,
    pub token: String,
    pub expires_at: DateTime<Utc>,
}

#[cfg(feature = "ssr")]
lazy_static::lazy_static! {
    static ref SLUG_REGEX: regex::Regex = regex::Regex::new(r"^[a-z0-9\-]+$").unwrap();
    static ref USERNAME_REGEX: regex::Regex = regex::Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
}

impl Post {
    /// Generate HTML from markdown content
    pub fn generate_html(&mut self) {
        self.body_html = crate::markdown::markdown_to_html(&self.body_markdown);
    }

    /// Check if post is published
    pub fn is_published(&self) -> bool {
        self.published_at.is_some()
    }

    /// Get formatted publication date
    pub fn formatted_date(&self) -> Option<String> {
        self.published_at.map(|dt| dt.format("%B %d, %Y").to_string())
    }
}
