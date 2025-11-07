use leptos::*;
use leptos_router::*;
use leptos_meta::*;
use crate::models::{PostWithMetadata, Post, UserProfile, Category, Tag};

#[cfg(feature = "ssr")]
use crate::{
    db::Db,
    error::{AppError, log_error},
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchFilters {
    pub query: Option<String>,
    pub category_id: Option<i64>,
    pub tag_ids: Vec<i64>,
    pub author_id: Option<i64>,
    pub featured_only: bool,
    pub published_only: bool,
    pub sort_by: SearchSortBy,
    pub page: i32,
    pub per_page: i32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SearchSortBy {
    Newest,
    Oldest,
    MostViewed,
    MostLiked,
    Relevance,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchResults {
    pub posts: Vec<PostWithMetadata>,
    pub total_count: i64,
    pub page: i32,
    pub per_page: i32,
    pub has_next_page: bool,
    pub has_prev_page: bool,
}

impl Default for SearchFilters {
    fn default() -> Self {
        Self {
            query: None,
            category_id: None,
            tag_ids: Vec::new(),
            author_id: None,
            featured_only: false,
            published_only: true,
            sort_by: SearchSortBy::Newest,
            page: 1,
            per_page: 20,
        }
    }
}

#[server(SearchPosts, "/api")]
pub async fn search_posts(filters: SearchFilters) -> Result<SearchResults, ServerFnError> {
    let db = expect_context::<Db>();
    
    let offset = ((filters.page - 1) * filters.per_page) as i64;
    let limit = filters.per_page as i64;

    // Build the query based on filters
    let mut query_parts = vec!["SELECT DISTINCT p.*"];
    let mut from_parts = vec!["FROM posts p"];
    let mut where_parts = vec![];
    let mut params: Vec<Box<dyn sqlx::Encode<'_, sqlx::Postgres> + Send + Sync>> = Vec::new();
    let mut param_count = 0;

    // Join with authors if needed
    if filters.author_id.is_some() || matches!(filters.sort_by, SearchSortBy::Relevance) {
        from_parts.push("LEFT JOIN users u ON p.author_id = u.id");
    }

    // Join with categories if needed
    if filters.category_id.is_some() {
        from_parts.push("LEFT JOIN categories c ON p.category_id = c.id");
    }

    // Join with tags if needed
    if !filters.tag_ids.is_empty() {
        from_parts.push("LEFT JOIN post_tags pt ON p.id = pt.post_id");
        from_parts.push("LEFT JOIN tags t ON pt.tag_id = t.id");
    }

    // Published filter
    if filters.published_only {
        where_parts.push("p.published_at IS NOT NULL");
    }

    // Featured filter
    if filters.featured_only {
        where_parts.push("p.featured = true");
    }

    // Text search filter
    if let Some(ref query) = filters.query {
        if !query.trim().is_empty() {
            param_count += 1;
            where_parts.push(&format!("to_tsvector('english', p.title || ' ' || p.summary || ' ' || p.body_markdown) @@ plainto_tsquery('english', ${})", param_count));
        }
    }

    // Category filter
    if let Some(category_id) = filters.category_id {
        param_count += 1;
        where_parts.push(&format!("p.category_id = ${}", param_count));
    }

    // Author filter
    if let Some(author_id) = filters.author_id {
        param_count += 1;
        where_parts.push(&format!("p.author_id = ${}", param_count));
    }

    // Tag filter
    if !filters.tag_ids.is_empty() {
        param_count += 1;
        where_parts.push(&format!("t.id = ANY(${})", param_count));
    }

    // Build complete query
    let mut query_sql = query_parts.join(" ");
    query_sql.push(' ');
    query_sql.push_str(&from_parts.join(" "));
    
    if !where_parts.is_empty() {
        query_sql.push_str(" WHERE ");
        query_sql.push_str(&where_parts.join(" AND "));
    }

    // Add ordering
    match filters.sort_by {
        SearchSortBy::Newest => query_sql.push_str(" ORDER BY p.published_at DESC NULLS LAST, p.created_at DESC"),
        SearchSortBy::Oldest => query_sql.push_str(" ORDER BY p.published_at ASC NULLS LAST, p.created_at ASC"),
        SearchSortBy::MostViewed => query_sql.push_str(" ORDER BY p.views_count DESC, p.published_at DESC"),
        SearchSortBy::MostLiked => query_sql.push_str(" ORDER BY p.likes_count DESC, p.published_at DESC"),
        SearchSortBy::Relevance => {
            if filters.query.is_some() {
                query_sql.push_str(" ORDER BY ts_rank(to_tsvector('english', p.title || ' ' || p.summary || ' ' || p.body_markdown), plainto_tsquery('english', $1)) DESC, p.published_at DESC");
            } else {
                query_sql.push_str(" ORDER BY p.published_at DESC");
            }
        }
    }

    query_sql.push_str(&format!(" LIMIT {} OFFSET {}", limit, offset));

    // Execute the search query with simplified approach for now
    let posts = sqlx::query_as::<_, Post>(&format!(
        "SELECT * FROM posts WHERE {} ORDER BY {} LIMIT {} OFFSET {}",
        if filters.published_only { "published_at IS NOT NULL" } else { "1=1" },
        match filters.sort_by {
            SearchSortBy::Newest => "published_at DESC NULLS LAST, created_at DESC",
            SearchSortBy::Oldest => "published_at ASC NULLS LAST, created_at ASC", 
            SearchSortBy::MostViewed => "views_count DESC, published_at DESC",
            SearchSortBy::MostLiked => "likes_count DESC, published_at DESC",
            SearchSortBy::Relevance => "published_at DESC",
        },
        limit,
        offset
    ))
    .fetch_all(&**db)
    .await
    .map_err(|e| {
        log_error(&AppError::Database(e), "Failed to search posts");
        ServerFnError::new("Search failed".to_string())
    })?;

    // Get total count
    let total_count = sqlx::query_scalar::<_, i64>(&format!(
        "SELECT COUNT(*) FROM posts WHERE {}",
        if filters.published_only { "published_at IS NOT NULL" } else { "1=1" }
    ))
    .fetch_one(&**db)
    .await
    .map_err(|e| {
        log_error(&AppError::Database(e), "Failed to count posts");
        ServerFnError::new("Count failed".to_string())
    })?;

    // Convert posts to PostWithMetadata (simplified for now)
    let posts_with_metadata: Vec<PostWithMetadata> = posts
        .into_iter()
        .map(|post| PostWithMetadata {
            post,
            author: None,     // TODO: Load authors
            category: None,   // TODO: Load categories  
            tags: Vec::new(), // TODO: Load tags
        })
        .collect();

    let has_next_page = (offset + limit) < total_count;
    let has_prev_page = filters.page > 1;

    tracing::info!("Search completed: {} results", posts_with_metadata.len());

    Ok(SearchResults {
        posts: posts_with_metadata,
        total_count,
        page: filters.page,
        per_page: filters.per_page,
        has_next_page,
        has_prev_page,
    })
}

#[server(GetCategories, "/api")]
pub async fn get_categories() -> Result<Vec<Category>, ServerFnError> {
    let db = expect_context::<Db>();

    let categories = sqlx::query_as::<_, Category>(
        "SELECT * FROM categories WHERE is_active = true ORDER BY name"
    )
    .fetch_all(&**db)
    .await
    .map_err(|e| {
        log_error(&AppError::Database(e), "Failed to fetch categories");
        ServerFnError::new("Failed to load categories".to_string())
    })?;

    Ok(categories)
}

#[server(GetPopularTags, "/api")]
pub async fn get_popular_tags(limit: Option<i32>) -> Result<Vec<Tag>, ServerFnError> {
    let db = expect_context::<Db>();
    let limit = limit.unwrap_or(20);

    let tags = sqlx::query_as::<_, Tag>(
        "SELECT * FROM tags ORDER BY usage_count DESC, name ASC LIMIT $1"
    )
    .bind(limit)
    .fetch_all(&**db)
    .await
    .map_err(|e| {
        log_error(&AppError::Database(e), "Failed to fetch tags");
        ServerFnError::new("Failed to load tags".to_string())
    })?;

    Ok(tags)
}

#[component]
pub fn SearchPage() -> impl IntoView {
    let query_params = use_query_map();
    
    let (search_filters, set_search_filters) = create_signal(SearchFilters::default());
    let (search_query, set_search_query) = create_signal(String::new());
    
    // Initialize filters from URL params
    create_effect(move |_| {
        query_params.with(|params| {
            if let Some(q) = params.get("q") {
                set_search_query.set(q.clone());
                set_search_filters.update(|filters| filters.query = Some(q.clone()));
            }
        });
    });

    let search_results = create_resource(
        move || search_filters.get(),
        |filters| async move { search_posts(filters).await }
    );

    let categories = create_resource(|| (), |_| async { get_categories().await });
    let popular_tags = create_resource(|| (), |_| async { get_popular_tags(Some(20)).await });

    view! {
        <Title text="Search - Taleji" />
        <Meta name="description" content="Search blog posts and articles on Taleji" />
        
        <div class="search-page">
            <div class="search-header">
                <h1>"Search Posts"</h1>
                
                <div class="search-form">
                    <input
                        type="text"
                        placeholder="Search posts..."
                        class="search-input"
                        prop:value=move || search_query.get()
                        on:input=move |ev| {
                            let value = event_target_value(&ev);
                            set_search_query.set(value.clone());
                        }
                        on:keydown=move |ev| {
                            if ev.key() == "Enter" {
                                set_search_filters.update(|filters| {
                                    filters.query = Some(search_query.get());
                                    filters.page = 1;
                                });
                            }
                        }
                    />
                    <button 
                        class="search-button"
                        on:click=move |_| {
                            set_search_filters.update(|filters| {
                                filters.query = Some(search_query.get());
                                filters.page = 1;
                            });
                        }
                    >
                        "Search"
                    </button>
                </div>
            </div>

            <div class="search-content">
                <aside class="search-filters">
                    <h3>"Filters"</h3>
                    
                    <div class="filter-section">
                        <h4>"Categories"</h4>
                        <Suspense fallback=|| view! { <p>"Loading categories..."</p> }>
                            {move || categories.get().map(|res| match res {
                                Ok(cats) => view! {
                                    <div class="filter-options">
                                        {cats.into_iter().map(|cat| view! {
                                            <label class="filter-option">
                                                <input 
                                                    type="radio" 
                                                    name="category"
                                                    on:change=move |_| {
                                                        set_search_filters.update(|filters| {
                                                            filters.category_id = Some(cat.id);
                                                            filters.page = 1;
                                                        });
                                                    }
                                                />
                                                {cat.name}
                                            </label>
                                        }).collect_view()}
                                    </div>
                                }.into_view(),
                                Err(e) => view! {
                                    <p class="error">{format!("Error: {}", e)}</p>
                                }.into_view()
                            })}
                        </Suspense>
                    </div>

                    <div class="filter-section">
                        <h4>"Popular Tags"</h4>
                        <Suspense fallback=|| view! { <p>"Loading tags..."</p> }>
                            {move || popular_tags.get().map(|res| match res {
                                Ok(tags) => view! {
                                    <div class="tag-cloud">
                                        {tags.into_iter().map(|tag| view! {
                                            <button 
                                                class="tag-button"
                                                on:click=move |_| {
                                                    set_search_filters.update(|filters| {
                                                        if !filters.tag_ids.contains(&tag.id) {
                                                            filters.tag_ids.push(tag.id);
                                                        }
                                                        filters.page = 1;
                                                    });
                                                }
                                            >
                                                {tag.name}
                                            </button>
                                        }).collect_view()}
                                    </div>
                                }.into_view(),
                                Err(e) => view! {
                                    <p class="error">{format!("Error: {}", e)}</p>
                                }.into_view()
                            })}
                        </Suspense>
                    </div>
                </aside>

                <main class="search-results">
                    <Suspense fallback=|| view! { <p class="loading">"Searching..."</p> }>
                        {move || search_results.get().map(|res| match res {
                            Ok(results) => view! {
                                <div class="results-header">
                                    <p class="results-count">
                                        {format!("Found {} results", results.total_count)}
                                    </p>
                                </div>
                                
                                <div class="posts-grid">
                                    {results.posts.into_iter().map(|post_data| view! {
                                        <article class="post-card">
                                            <h3>
                                                <a href=format!("/post/{}", post_data.post.slug)>
                                                    {post_data.post.title}
                                                </a>
                                            </h3>
                                            <p class="post-summary">{post_data.post.summary}</p>
                                            <div class="post-meta">
                                                <span class="views">{format!("{} views", post_data.post.views_count)}</span>
                                                <span class="likes">{format!("{} likes", post_data.post.likes_count)}</span>
                                            </div>
                                        </article>
                                    }).collect_view()}
                                </div>

                                <Show when=move || results.total_count == 0>
                                    <div class="no-results">
                                        <h3>"No posts found"</h3>
                                        <p>"Try adjusting your search criteria."</p>
                                    </div>
                                </Show>
                            }.into_view(),
                            Err(e) => view! {
                                <p class="error">{format!("Search error: {}", e)}</p>
                            }.into_view()
                        })}
                    </Suspense>
                </main>
            </div>
        </div>
    }
}