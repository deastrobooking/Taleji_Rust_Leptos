#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use axum::Router;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use leptos_blog::app::*;
    use leptos_blog::db;

    dotenvy::dotenv().ok();

    let conf = get_configuration(Some("Cargo.toml")).await?;
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options.clone();

    let db_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let pool = db::create_pool(&db_url).await?;

    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            move || provide_context(pool.clone()),
            App,
        )
        .with_state(leptos_options);

    println!("Listening on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
}
