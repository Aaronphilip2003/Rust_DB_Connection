mod api_docs;
mod handlers;
mod models;
mod routes;
mod services;

use api_docs::ApiDoc;
use axum::Router;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{EnvFilter, fmt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    fmt()
        .with_env_filter(EnvFilter::new("hello_api=info,tower_http=debug"))
        .init();

    let app = Router::new()
        .merge(routes::hello_routes::routes())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    println!("Server running on http://localhost:8000");
    println!("Swagger UI at http://localhost:8000/swagger-ui");

    axum::serve(listener, app).await.unwrap();
}
