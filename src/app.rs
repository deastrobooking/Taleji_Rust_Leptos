use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::{
    home::HomePage, 
    post::PostPage, 
    auth::{LoginPage, RegisterPage},
    search::SearchPage
};

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
                    <div class="nav-content">
                        <a href="/" class="logo">"Taleji"</a>
                        <div class="nav-links">
                            <a href="/search" class="nav-link">"Search"</a>
                            <a href="/login" class="nav-link">"Login"</a>
                            <a href="/register" class="nav-link btn btn-primary">"Sign Up"</a>
                        </div>
                    </div>
                </div>
            </nav>
            <main class="container">
                <Routes>
                    <Route path="/" view=HomePage />
                    <Route path="/post/:slug" view=PostPage />
                    <Route path="/search" view=SearchPage />
                    <Route path="/login" view=LoginPage />
                    <Route path="/register" view=RegisterPage />
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
