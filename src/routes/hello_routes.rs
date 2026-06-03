use axum::{Router, routing::get};

use crate::handlers::hello_handler::hello;

pub fn routes() -> Router {
    Router::new().route("/hello", get(hello))
}
