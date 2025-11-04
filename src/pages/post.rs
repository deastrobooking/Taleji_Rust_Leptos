use leptos::*;
use leptos_router::*;
use leptos_meta::*;
use crate::models::Post;

#[cfg(feature = "ssr")]
use crate::db::Db;

#[server(GetPostBySlug, "/api")]
pub async fn get_post_by_slug(slug: String) -> Result<Post, ServerFnError> {
    let db = expect_context::<Db>();

    let post = sqlx::query_as::<_, Post>(
        r#"
        SELECT * FROM posts
        WHERE slug = $1 AND published_at IS NOT NULL
        "#
    )
    .bind(&slug)
    .fetch_one(&*db)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

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
