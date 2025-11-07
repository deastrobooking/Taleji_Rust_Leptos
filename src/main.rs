#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use axum::{Router, middleware};
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use leptos_blog::app::*;
    use leptos_blog::{db, security};
    use tower_http::{compression::CompressionLayer, trace::TraceLayer};
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "leptos_blog=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenvy::dotenv().ok();
    
    tracing::info!("Starting Taleji blog server");

    let conf = get_configuration(Some("Cargo.toml")).await?;
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options.clone();

    let db_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let pool = db::create_pool(&db_url).await?;
    
    // Test database connection
    db::health_check(&pool).await?;
    tracing::info!("Database connection established: {}", db::pool_status(&pool));

    let routes = generate_route_list(App);

    // Create rate limiter (100 requests per minute)
    let rate_limiter = security::RateLimiter::new(100, 60);

    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            move || {
                provide_context(pool.clone());
                provide_context(leptos_blog::auth::AuthService::new());
            },
            App,
        )
        // Add security middleware
        .layer(middleware::from_fn(security::request_id))
        .layer(middleware::from_fn(security::security_headers))
        .layer(middleware::from_fn(security::csrf_protection))
        .layer(middleware::from_fn(move |req, next| {
            let limiter = rate_limiter.clone();
            async move { limiter.check_rate_limit(req, next).await }
        }))
        // Add compression and tracing
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .with_state(leptos_options);

    tracing::info!("Server starting on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
}
