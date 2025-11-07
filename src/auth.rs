#[cfg(feature = "ssr")]
use crate::{
    db::Db,
    error::{AppError, AppResult},
    models::{User, UserProfile, UserRole, LoginInput, RegisterInput, AuthResponse},
};
#[cfg(feature = "ssr")]
use bcrypt::{hash, verify, DEFAULT_COST};
#[cfg(feature = "ssr")]
use chrono::{DateTime, Utc, Duration};
#[cfg(feature = "ssr")]
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
#[cfg(feature = "ssr")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use uuid::Uuid;

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,        // User ID
    pub username: String,
    pub role: UserRole,
    pub exp: i64,        // Expiration time
    pub iat: i64,        // Issued at
    pub jti: String,     // JWT ID
}

#[cfg(feature = "ssr")]
pub struct AuthService {
    jwt_secret: String,
    jwt_expiry_hours: i64,
}

#[cfg(feature = "ssr")]
impl AuthService {
    pub fn new() -> Self {
        Self {
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "your-super-secret-jwt-key-change-in-production".to_string()),
            jwt_expiry_hours: std::env::var("JWT_EXPIRY_HOURS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(24),
        }
    }

    /// Hash a password using bcrypt
    pub fn hash_password(&self, password: &str) -> AppResult<String> {
        hash(password, DEFAULT_COST).map_err(|e| {
            tracing::error!("Failed to hash password: {}", e);
            AppError::Internal("Password hashing failed".to_string())
        })
    }

    /// Verify a password against a hash
    pub fn verify_password(&self, password: &str, hash: &str) -> AppResult<bool> {
        verify(password, hash).map_err(|e| {
            tracing::error!("Failed to verify password: {}", e);
            AppError::Internal("Password verification failed".to_string())
        })
    }

    /// Generate a JWT token for a user
    pub fn generate_token(&self, user: &User) -> AppResult<String> {
        let now = Utc::now();
        let exp = now + Duration::hours(self.jwt_expiry_hours);
        
        let claims = Claims {
            sub: user.id,
            username: user.username.clone(),
            role: user.role.clone(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
            jti: Uuid::new_v4().to_string(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )
        .map_err(|e| {
            tracing::error!("Failed to generate JWT: {}", e);
            AppError::Internal("Token generation failed".to_string())
        })
    }

    /// Validate and decode a JWT token
    pub fn validate_token(&self, token: &str) -> AppResult<Claims> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )
        .map(|data| data.claims)
        .map_err(|e| {
            tracing::warn!("Invalid JWT token: {}", e);
            AppError::Unauthorized
        })
    }

    /// Register a new user
    pub async fn register_user(&self, db: &Db, input: RegisterInput) -> AppResult<AuthResponse> {
        // Check if username or email already exists
        let existing_user = sqlx::query!(
            "SELECT COUNT(*) as count FROM users WHERE username = $1 OR email = $2",
            input.username,
            input.email
        )
        .fetch_one(&**db)
        .await
        .map_err(AppError::Database)?;

        if existing_user.count.unwrap_or(0) > 0 {
            return Err(AppError::Validation("Username or email already exists".to_string()));
        }

        // Hash the password
        let password_hash = self.hash_password(&input.password)?;

        // Create the user
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (username, email, password_hash, display_name, role)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, username, email, password_hash, display_name, bio, avatar_url, 
                      role as "role: UserRole", email_verified, is_active, created_at, updated_at
            "#,
            input.username,
            input.email,
            password_hash,
            input.display_name,
            UserRole::User as UserRole
        )
        .fetch_one(&**db)
        .await
        .map_err(AppError::Database)?;

        // Generate token
        let token = self.generate_token(&user)?;
        let expires_at = Utc::now() + Duration::hours(self.jwt_expiry_hours);

        tracing::info!("User registered: {}", user.username);

        Ok(AuthResponse {
            user: user.into(),
            token,
            expires_at,
        })
    }

    /// Authenticate a user with email/username and password
    pub async fn login_user(&self, db: &Db, input: LoginInput) -> AppResult<AuthResponse> {
        // Find user by email or username
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, display_name, bio, avatar_url, 
                   role as "role: UserRole", email_verified, is_active, created_at, updated_at
            FROM users 
            WHERE (email = $1 OR username = $1) AND is_active = true
            "#,
            input.email_or_username
        )
        .fetch_optional(&**db)
        .await
        .map_err(AppError::Database)?;

        let user = user.ok_or_else(|| AppError::Validation("Invalid credentials".to_string()))?;

        // Verify password
        if !self.verify_password(&input.password, &user.password_hash)? {
            return Err(AppError::Validation("Invalid credentials".to_string()));
        }

        // Generate token
        let token = self.generate_token(&user)?;
        let expires_at = Utc::now() + Duration::hours(self.jwt_expiry_hours);

        tracing::info!("User logged in: {}", user.username);

        Ok(AuthResponse {
            user: user.into(),
            token,
            expires_at,
        })
    }

    /// Get user by ID
    pub async fn get_user_by_id(&self, db: &Db, user_id: i64) -> AppResult<User> {
        sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, display_name, bio, avatar_url, 
                   role as "role: UserRole", email_verified, is_active, created_at, updated_at
            FROM users 
            WHERE id = $1 AND is_active = true
            "#,
            user_id
        )
        .fetch_optional(&**db)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))
    }

    /// Check if user has permission for a given role
    pub fn check_permission(&self, user_role: &UserRole, required_role: &UserRole) -> bool {
        match required_role {
            UserRole::User => true, // Everyone can access user-level content
            UserRole::Author => matches!(user_role, UserRole::Author | UserRole::Admin),
            UserRole::Admin => matches!(user_role, UserRole::Admin),
        }
    }
}

#[cfg(feature = "ssr")]
impl Default for AuthService {
    fn default() -> Self {
        Self::new()
    }
}