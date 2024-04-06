use std::path::PathBuf;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
};
use minijinja::context;
use tokio::task::block_in_place;

use crate::{
    post::{list_posts, read_post, PostRenderError},
    AppState,
};

pub async fn index(State(st): State<AppState>) -> impl IntoResponse {
    let content = block_in_place(|| st.env.get_template("index.html")?.render(context! {}));

    match content {
        Ok(content) => Html(content).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn contact(State(st): State<AppState>) -> impl IntoResponse {
    let content = block_in_place(|| st.env.get_template("contact.html")?.render(context! {}));

    match content {
        Ok(content) => Html(content).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn posts(State(st): State<AppState>) -> impl IntoResponse {
    let posts = match list_posts() {
        Ok(e) => e,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };
    let content = block_in_place(|| {
        st.env
            .get_template("posts.html")?
            .render(context! { posts })
    });

    match content {
        Ok(html) => Html(html).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn post(State(st): State<AppState>, Path(post_path): Path<String>) -> impl IntoResponse {
    let mut path = PathBuf::from("./posts/");
    path.push(format!("{post_path}.md"));

    if let Ok(content) = std::fs::read_to_string(path) {
        match block_in_place(|| read_post(content, &st.env)) {
            Ok(x) => Html(x).into_response(),
            Err(PostRenderError::PostNotPublished) => (StatusCode::NOT_FOUND).into_response(),
            err => {err.unwrap(); "".into_response()},
        }
    } else {
        (StatusCode::NOT_FOUND).into_response()
    }
}
