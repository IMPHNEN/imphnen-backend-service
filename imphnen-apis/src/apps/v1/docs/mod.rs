use utoipa::OpenApi;

pub mod docs_controller;
pub use docs_controller::*;

pub fn docs_router() -> utoipa::openapi::OpenApi {
	ApiDoc::openapi()
}
