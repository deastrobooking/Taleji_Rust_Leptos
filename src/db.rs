#[cfg(feature = "ssr")]
use sqlx::{Pool, Postgres, PgPoolOptions};
#[cfg(feature = "ssr")]
use std::sync::Arc;
#[cfg(feature = "ssr")]
use std::time::Duration;
#[cfg(feature = "ssr")]
use crate::error::{AppError, AppResult};

#[cfg(feature = "ssr")]
pub type Db = Arc<Pool<Postgres>>;

/// Database configuration
#[cfg(feature = "ssr")]
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: Duration,
    pub idle_timeout: Option<Duration>,
    pub max_lifetime: Option<Duration>,
}

#[cfg(feature = "ssr")]
impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://localhost/taleji_blog".to_string()),
            max_connections: 20,
            min_connections: 5,
            connect_timeout: Duration::from_secs(10),
            idle_timeout: Some(Duration::from_secs(600)), // 10 minutes
            max_lifetime: Some(Duration::from_secs(1800)), // 30 minutes
        }
    }
}

#[cfg(feature = "ssr")]
pub async fn create_pool(database_url: &str) -> anyhow::Result<Db> {
    let config = DatabaseConfig {
        url: database_url.to_string(),
        ..Default::default()
    };
    create_pool_with_config(config).await
}

#[cfg(feature = "ssr")]
pub async fn create_pool_with_config(config: DatabaseConfig) -> anyhow::Result<Db> {
    tracing::info!("Connecting to database with optimized pool configuration");

    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(config.connect_timeout)
        .idle_timeout(config.idle_timeout)
        .max_lifetime(config.max_lifetime)
        .test_before_acquire(true)
        .connect(&config.url)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create database pool: {}", e);
            anyhow::anyhow!("Database connection failed: {}", e)
        })?;

    // Run migrations
    tracing::info!("Running database migrations");
    sqlx::migrate!().run(&pool).await.map_err(|e| {
        tracing::error!("Failed to run migrations: {}", e);
        anyhow::anyhow!("Migration failed: {}", e)
    })?;

    tracing::info!("Database pool created successfully");
    Ok(Arc::new(pool))
}

/// Health check for database connection
#[cfg(feature = "ssr")]
pub async fn health_check(db: &Db) -> AppResult<()> {
    sqlx::query("SELECT 1")
        .execute(&**db)
        .await
        .map_err(AppError::Database)?;
    
    Ok(())
}

/// Get database connection pool statistics
#[cfg(feature = "ssr")]
pub fn pool_status(db: &Db) -> String {
    format!(
        "Pool status - Size: {}, Idle: {}, Active: {}",
        db.size(),
        db.num_idle(),
        db.size() - db.num_idle()
    )
}
