use leptos::*;
use leptos_router::*;
use leptos_meta::*;
use crate::models::Post;

#[cfg(feature = "ssr")]
use crate::db::Db;
#[cfg(feature = "ssr")]
use crate::error::{AppError, log_error};

#[server(GetPostBySlug, "/api")]
pub async fn get_post_by_slug(slug: String) -> Result<Post, ServerFnError> {
    use sqlx::Row;
    
    let db = expect_context::<Db>();

    // Validate slug format
    if slug.is_empty() || slug.len() > 100 {
        let error = AppError::Validation("Invalid slug format".to_string());
        log_error(&error, &format!("Invalid slug: {}", slug));
        return Err(ServerFnError::from(error));
    }

    let post = sqlx::query_as::<_, Post>(
        r#"
        SELECT * FROM posts
        WHERE slug = $1 AND published_at IS NOT NULL
        "#
    )
    .bind(&slug)
    .fetch_one(&*db)
    .await
    .map_err(|e| {
        let app_error = match e {
            sqlx::Error::RowNotFound => AppError::NotFound(format!("Post with slug '{}' not found", slug)),
            _ => AppError::Database(e),
        };
        log_error(&app_error, &format!("Failed to fetch post with slug: {}", slug));
        ServerFnError::from(app_error)
    })?;

    tracing::info!("Retrieved post: {} (slug: {})", post.title, post.slug);
    Ok(post)
}

#[component]
pub fn PostPage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.with(|m| m.get("slug").cloned().unwrap_or_default());

    let post_res = create_resource(slug, |slug| async move {
        get_post_by_slug(slug).await
    });

    view! {
        <article class="post-page">
            <Suspense fallback=move || view! { <p class="loading">"Loading post..."</p> }>
                {move || post_res.get().map(|res| match res {
                    Ok(post) => {
                        let date = post.published_at
                            .map(|d| d.format("%B %d, %Y").to_string())
                            .unwrap_or_default();
                        view! {
                            <Title text=post.title.clone() />
                            <Meta name="description" content=post.summary.clone() />
                            <div class="post-header">
                                <h1>{post.title}</h1>
                                <p class="post-meta">{date}</p>
                                <p class="post-summary">{post.summary}</p>
                            </div>
                            <div class="post-body" inner_html=post.body_html></div>
                            <div class="post-footer">
                                <a href="/" class="back-link">"← Back to all posts"</a>
                            </div>
                        }.into_view()
                    },
                    Err(e) => view! {
                        <div class="error-page">
                            <h1>"Post not found"</h1>
                            <p>{format!("Error: {e}")}</p>
                            <a href="/" class="back-link">"← Back to all posts"</a>
                        </div>
                    }.into_view()
                })}
            </Suspense>
        </article>
    }
}
