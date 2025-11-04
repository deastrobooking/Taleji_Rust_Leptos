# Rust/Leptos Blog

A modern, high-performance blog built with Rust and Leptos full-stack SSR framework.

## Overview

This project is a "Next.js-in-Rust" blog kit featuring:
- **Leptos**: Full-stack Rust framework with server-side rendering
- **Axum**: Fast, ergonomic web server
- **PostgreSQL**: Reliable database with sqlx
- **Markdown**: Blog posts stored as Markdown, rendered to HTML
- **WebAssembly**: Client-side hydration support (not yet built)

## Current Status

✅ **Working Features:**
- Server-side rendering (SSR) on port 5000
- PostgreSQL database with posts table
- Markdown blog posts with HTML rendering
- Blog listing page showing all published posts
- Individual post pages with full content
- Responsive styling with modern CSS
- Sample blog posts loaded

⏳ **Next Phase:**
- Build client-side WASM bundle for hydration
- Add admin UI for creating/editing posts
- Implement search functionality
- Add RSS feed generation
- Filesystem-based Markdown import

## Project Structure

```
.
├── src/
│   ├── main.rs          # Axum + Leptos server entry
│   ├── lib.rs           # Library exports (SSR + hydrate)
│   ├── app.rs           # Root Leptos component with routing
│   ├── models.rs        # Post data model
│   ├── db.rs            # Database pool and helpers
│   ├── markdown.rs      # Markdown to HTML rendering
│   └── pages/
│       ├── home.rs      # Blog listing page
│       └── post.rs      # Individual post page
├── migrations/          # Database SQL migrations
├── style/
│   └── main.css         # Responsive blog styling
├── Cargo.toml           # Rust dependencies and features
└── Leptos.toml          # Leptos configuration

```

## Tech Stack Details

### Backend
- **Leptos 0.6**: Full-stack Rust framework
- **Axum 0.7**: Web server
- **sqlx 0.7**: PostgreSQL driver with compile-time query checking
- **pulldown-cmark**: Markdown parser
- **tokio**: Async runtime

### Frontend
- **Leptos SSR**: Server-side rendering for SEO
- **Responsive CSS**: Mobile-friendly design
- **WebAssembly**: (Next phase - client hydration)

## Database Schema

```sql
CREATE TABLE posts (
    id              BIGSERIAL PRIMARY KEY,
    slug            TEXT UNIQUE NOT NULL,
    title           TEXT NOT NULL,
    summary         TEXT NOT NULL,
    body_markdown   TEXT NOT NULL,
    body_html       TEXT NOT NULL,
    published_at    TIMESTAMPTZ,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

## Running the Application

The blog server is configured to run automatically via the `blog-server` workflow.

**Manual execution:**
```bash
cargo run --no-default-features --features ssr --bin leptos-blog
```

The server listens on `0.0.0.0:5000`.

## Feature Flags

The project uses Cargo features to separate server and client builds:

- `ssr`: Server-side rendering (includes Axum, sqlx, database)
- `hydrate`: Client-side WASM bundle (includes wasm-bindgen)

**Important:** Never build with both features at once. Build server with `--features ssr` only.

## Key Implementation Notes

1. **Server Functions**: Leptos `#[server]` macros provide clean RPC-style API
2. **Context Injection**: Database pool is provided via Leptos context
3. **SEO-Friendly**: All pages render server-side for optimal SEO
4. **Type Safety**: sqlx provides compile-time SQL query verification

## Adding Blog Posts

Currently, blog posts are in the database. Sample posts are included.

**Via SQL:**
```sql
INSERT INTO posts (slug, title, summary, body_markdown, body_html, published_at)
VALUES (
    'my-post-slug',
    'My Post Title',
    'Brief summary',
    '# Markdown content',
    '<h1>HTML content</h1>',
    NOW()
);
```

## Architecture

This follows the Leptos full-stack SSR pattern:
1. **Server** renders pages to HTML on request
2. **Client** (future) hydrates for interactivity
3. **Server functions** handle data fetching seamlessly
4. **Routing** is isomorphic - same routes work client and server-side

## Recent Changes

- **2025-11-04**: Initial project setup
  - Created full Leptos SSR application
  - Set up PostgreSQL database with migrations
  - Implemented blog listing and post pages
  - Fixed Cargo.toml feature configuration to separate SSR/WASM builds
  - Added sample blog posts and styling

## Troubleshooting

**"cannot access imported statics on non-wasm targets"**
- This means WASM dependencies leaked into SSR build
- Solution: Ensure Cargo.toml has `default-features = false` for leptos deps
- Build with: `cargo build --no-default-features --features ssr`

**Build takes too long**
- First build compiles many dependencies (~400 crates)
- Subsequent builds use cached artifacts and are much faster
- Use `cargo build --features ssr` for incremental builds
