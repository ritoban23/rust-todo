use axum::http::Response;
use axum::body::Body;

pub async fn hello_world() -> Response<Body> {
    Response::new(Body::from("Hello World from my own file"))
}
