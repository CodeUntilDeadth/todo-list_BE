mod docs;
mod controller;

use axum::{routing::get,Router};
use controller::ping;
pub use docs::*;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
pub fn build() -> Router{
    let router = Router::new()
        .route("/",get(ping))
        .merge( SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", APIdocs::openapi()));
    router
}

