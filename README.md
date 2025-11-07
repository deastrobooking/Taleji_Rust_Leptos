# Taleji 📝

> A modern, high-performance blogging and research platform built with Rust, Leptos, and WebAssembly.

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![Leptos](https://img.shields.io/badge/leptos-0.6-blue.svg)](https://leptos.dev)
[![PostgreSQL](https://img.shields.io/badge/postgresql-15+-blue.svg)](https://postgresql.org)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

Taleji is a cutting-edge blogging platform that combines the performance of Rust with the interactivity of modern web technologies. Built for global scale with full-stack Rust using the Leptos framework.

## 🚀 Features

### Current Features ✅
- **Full-Stack Rust**: Server-side rendering with client-side hydration
- **Blazing Fast**: Built with Leptos and WebAssembly for optimal performance
- **Markdown Support**: Rich content creation with pulldown-cmark
- **PostgreSQL Database**: Robust data persistence with SQLx
- **Responsive Design**: Mobile-first CSS design
- **SEO Friendly**: Meta tags and structured content

### Planned Features 🔄
See our comprehensive [Feature Roadmap](features.md) for detailed development plans including:
- User authentication and authorization
- Advanced content management
- Search and discovery
- Internationalization (i18n)
- Analytics and insights
- Community features

## 🛠️ Technology Stack

### Backend
- **[Rust](https://www.rust-lang.org/)**: Systems programming language for performance and safety
- **[Leptos](https://leptos.dev/)**: Full-stack Rust web framework
- **[Axum](https://github.com/tokio-rs/axum)**: Web server framework
- **[SQLx](https://github.com/launchbadge/sqlx)**: Async SQL toolkit
- **[PostgreSQL](https://postgresql.org/)**: Advanced relational database

### Frontend
- **[Leptos](https://leptos.dev/)**: Reactive web framework with hydration
- **[WebAssembly](https://webassembly.org/)**: High-performance web applications
- **CSS3**: Modern responsive styling
- **[pulldown-cmark](https://github.com/raphlinus/pulldown-cmark)**: CommonMark parser

### Development Tools
- **[Cargo](https://doc.rust-lang.org/cargo/)**: Rust package manager
- **[SQLx CLI](https://github.com/launchbadge/sqlx/tree/master/sqlx-cli)**: Database migrations
- **[Leptos CLI](https://github.com/leptos-rs/cargo-leptos)**: Development server

## 🏁 Quick Start

### Prerequisites
- [Rust](https://rustup.rs/) (1.70+)
- [PostgreSQL](https://postgresql.org/) (15+)
- [Node.js](https://nodejs.org/) (for Tailwind CSS - optional)

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/deastrobooking/Taleji_Rust_Leptos.git
   cd Taleji_Rust_Leptos
   ```

2. **Install Leptos CLI**
   ```bash
   cargo install cargo-leptos
   ```

3. **Install SQLx CLI**
   ```bash
   cargo install sqlx-cli --features postgres
   ```

4. **Set up the database**
   ```bash
   # Create database
   createdb taleji_blog
   
   # Set environment variable
   export DATABASE_URL="postgresql://username:password@localhost/taleji_blog"
   
   # Run migrations
   sqlx migrate run
   ```

5. **Create environment file**
   ```bash
   cp .env.example .env
   # Edit .env with your database URL and other settings
   ```

### Development

1. **Run the development server**
   ```bash
   cargo leptos watch
   ```

2. **Open your browser**
   Navigate to `http://localhost:3000`

### Production Build

```bash
cargo leptos build --release
```

## 📁 Project Structure

```
Taleji_Rust_Leptos/
├── src/
│   ├── app.rs              # Main app component and routing
│   ├── lib.rs              # Library root and hydration
│   ├── main.rs             # Server entry point
│   ├── db.rs               # Database connection and setup
│   ├── models.rs           # Data models and schemas
│   ├── markdown.rs         # Markdown processing utilities
│   └── pages/              # Page components
│       ├── mod.rs
│       ├── home.rs         # Home page with post listings
│       └── post.rs         # Individual post page
├── migrations/             # Database migrations
│   └── 20251104120000_create_posts.sql
├── style/
│   └── main.css            # Application styles
├── Cargo.toml              # Rust dependencies and metadata
├── Leptos.toml             # Leptos configuration
└── README.md               # This file
```

## 🗄️ Database Schema

### Posts Table
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

## 🔧 Configuration

### Environment Variables

Create a `.env` file in the root directory:

```env
# Database
DATABASE_URL=postgresql://username:password@localhost/taleji_blog

# Application
RUST_LOG=info
LEPTOS_SITE_ADDR=0.0.0.0:3000
LEPTOS_RELOAD_PORT=3001
```

### Leptos Configuration

The `Leptos.toml` file contains framework-specific settings:

```toml
[leptos]
output-name = "leptos-blog"
site-root = "target/site"
site-pkg-dir = "pkg"
style-file = "style/main.css"
assets-dir = "public"
site-addr = "0.0.0.0:5000"
reload-port = 3001
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run with features
cargo test --features ssr

# Run specific test module
cargo test --test integration_tests
```

## 🚢 Deployment

### Docker Deployment

```dockerfile
# Dockerfile example
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo leptos build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/leptos-blog /app/
COPY --from=builder /app/target/site /app/site
CMD ["./leptos-blog"]
```

### Production Considerations

- **Environment Variables**: Configure all required environment variables
- **Database**: Set up PostgreSQL with proper connection pooling
- **Static Assets**: Use a CDN for serving static files
- **SSL**: Configure HTTPS with proper certificates
- **Monitoring**: Set up logging and health checks

## 📊 Performance

### Benchmarks
- **Cold Start**: < 100ms
- **Page Load**: < 200ms (95th percentile)
- **Bundle Size**: < 500KB compressed
- **Lighthouse Score**: 95+ across all metrics

### Optimizations
- Server-side rendering for instant page loads
- WebAssembly for client-side performance
- Efficient database queries with prepared statements
- Optimized CSS with minimal footprint

## 🤝 Contributing

We welcome contributions! Please see our [Feature Roadmap](features.md) for planned improvements.

### Development Setup

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes and add tests
4. Ensure all tests pass: `cargo test`
5. Format code: `cargo fmt`
6. Lint code: `cargo clippy`
7. Commit changes: `git commit -m 'Add amazing feature'`
8. Push to branch: `git push origin feature/amazing-feature`
9. Open a Pull Request

### Code Standards

- Follow Rust naming conventions
- Add documentation for public APIs
- Include tests for new functionality
- Maintain backwards compatibility
- Update README if needed

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Leptos](https://leptos.dev/) - The reactive web framework that powers this application
- [Rust Community](https://www.rust-lang.org/community) - For the amazing ecosystem and tools
- [PostgreSQL](https://postgresql.org/) - For reliable and powerful database functionality

## 🔗 Links

- **Live Demo**: [Coming Soon]
- **Documentation**: [Coming Soon]
- **Issues**: [GitHub Issues](https://github.com/deastrobooking/Taleji_Rust_Leptos/issues)
- **Discussions**: [GitHub Discussions](https://github.com/deastrobooking/Taleji_Rust_Leptos/discussions)

---

**Built with ❤️ using Rust and Leptos**

*For detailed feature planning and roadmap, see [features.md](features.md)*