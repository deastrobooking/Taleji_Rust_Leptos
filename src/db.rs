#[cfg(feature = "ssr")]
use sqlx::{Pool, Postgres};
#[cfg(feature = "ssr")]
use std::sync::Arc;

#[cfg(feature = "ssr")]
pub type Db = Arc<Pool<Postgres>>;

#[cfg(feature = "ssr")]
pub async fn create_pool(database_url: &str) -> anyhow::Result<Db> {
    let pool = Pool::<Postgres>::connect(database_url).await?;
    sqlx::migrate!().run(&pool).await?;
    Ok(Arc::new(pool))
}
