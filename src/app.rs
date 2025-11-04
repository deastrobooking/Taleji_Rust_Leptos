use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::{home::HomePage, post::PostPage};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html lang="en" />
        <Title text="Rust/Leptos Blog" />
        <Meta name="viewport" content="width=device-width, initial-scale=1" />
        <Link rel="stylesheet" href="/style.css" />
        <Body class="blog-app"/>
        
        <Router>
            <nav class="nav">
                <div class="container">
                    <a href="/" class="logo">"Rust Blog"</a>
                </div>
            </nav>
            <main class="container">
                <Routes>
                    <Route path="/" view=HomePage />
                    <Route path="/post/:slug" view=PostPage />
                </Routes>
            </main>
            <footer class="footer">
                <div class="container">
                    <p>"Built with Leptos + Rust + WebAssembly"</p>
                </div>
            </footer>
        </Router>
    }
}
