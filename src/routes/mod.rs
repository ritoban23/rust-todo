use axum::{body::Body, Router, routing::get};

pub fn create_routes() -> Router<Body> {
    Router::new().route("/", get(|| async { "Hello, World!" }))
}