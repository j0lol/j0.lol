mod post;

use askama::Template;
use axum::{routing::get, Router};
use tower_http::services::fs::ServeDir;

const URL: &str = "0.0.0.0:8080";

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(index))
        .route("/posts", get(post::posts))
        .route("/post/:path", get(post::post))
        .nest_service("/static", ServeDir::new("static"));

    println!("Binding to {}", URL);
    let listener = tokio::net::TcpListener::bind(URL).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

async fn index() -> IndexTemplate {
    IndexTemplate {}
}
