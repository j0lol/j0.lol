mod handlers;
mod post;

use axum::{routing::get, Router};
use include_dir::include_dir;
use include_dir::Dir;
use minijinja::Environment;
use rand::distributions::Alphanumeric;
use rand::thread_rng;
use rand::Rng as _;
use thiserror::Error;
use tower_http::services::fs::ServeDir;
use tower_http::services::ServeFile;
const URL: &str = "0.0.0.0:8080";

#[derive(Clone)]
pub struct AppState {
    env: minijinja::Environment<'static>,
}

static TEMPLATES_DIR: Dir = include_dir!("./templates");

#[derive(Error, Debug)]
pub enum CustomLoaderError {
    #[error("Template not found.")]
    NotFound,
    #[error("Template not UTF-8.")]
    NotUtf8,
}
#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let mut env = Environment::new();
    env.set_loader(|name| {
        Ok(TEMPLATES_DIR
            .get_file(name)
            .and_then(|f| f.contents_utf8().map(|f| f.to_owned())))
    });
    let css_affix: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    env.add_global("CSS_AFFIX", css_affix.clone());

    let app = Router::new()
        .route("/", get(handlers::index))
        .route("/contact", get(handlers::contact))
        .route("/posts", get(handlers::posts))
        .route("/post/:path", get(handlers::post))
        .route_service(&format!("/cachebusting/style-{css_affix}.css"), ServeFile::new("static/style.css"))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(AppState { env });

    println!("Binding to {}", URL);
    let listener = tokio::net::TcpListener::bind(URL).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
