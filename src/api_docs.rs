use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(crate::handlers::hello_handler::hello),
    components(schemas(crate::models::responses::hello_response::HelloResponse))
)]
pub struct ApiDoc;
