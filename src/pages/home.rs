use leptos::*;
use crate::models::Post;

#[cfg(feature = "ssr")]
use crate::db::Db;
#[cfg(feature = "ssr")]
use crate::error::{AppError, log_error};

#[server(GetPublishedPosts, "/api")]
pub async fn get_published_posts() -> Result<Vec<Post>, ServerFnError> {
    let db = expect_context::<Db>();

    let posts = sqlx::query_as::<_, Post>(
        r#"
        SELECT * FROM posts
        WHERE published_at IS NOT NULL
        ORDER BY published_at DESC
        LIMIT 50
        "#
    )
    .fetch_all(&*db)
    .await
    .map_err(|e| {
        let app_error = AppError::Database(e);
        log_error(&app_error, "Failed to fetch published posts");
        ServerFnError::from(app_error)
    })?;

    tracing::info!("Retrieved {} published posts", posts.len());
    Ok(posts)
}

#[component]
pub fn HomePage() -> impl IntoView {
    let posts = create_resource(|| (), |_| async { get_published_posts().await });

    view! {
        <section class="home">
            <h1>"Latest Posts"</h1>
            <Suspense fallback=move || view! { <p class="loading">"Loading posts..."</p> }>
                {move || posts.get().map(|res| match res {
                    Ok(posts) if posts.is_empty() => view! {
                        <p class="empty">"No posts yet. Check back soon!"</p>
                    }.into_view(),
                    Ok(posts) => view! {
                        <ul class="post-list">
                            {posts.into_iter().map(|p| {
                                let date = p.published_at
                                    .map(|d| d.format("%B %d, %Y").to_string())
                                    .unwrap_or_default();
                                view! {
                                    <li class="post-item">
                                        <a href=format!("/post/{}", p.slug) class="post-link">
                                            <h2 class="post-title">{p.title}</h2>
                                            <p class="post-date">{date}</p>
                                            <p class="post-summary">{p.summary}</p>
                                        </a>
                                    </li>
                                }
                            }).collect_view()}
                        </ul>
                    }.into_view(),
                    Err(e) => view! {
                        <p class="error">{format!("Error loading posts: {e}")}</p>
                    }.into_view()
                })}
            </Suspense>
        </section>
    }
}
