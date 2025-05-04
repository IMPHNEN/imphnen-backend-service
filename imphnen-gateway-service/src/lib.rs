use axum::{
	Extension, Router, middleware::from_fn, response::Redirect, routing::get,
};
use imphnen_entities::{AppState, SurrealMemClient, SurrealWsClient};
use imphnen_iam_service::{iam_protected_routes, iam_public_routes};

pub mod docs;
pub use docs::*;
use imphnen_middleware_service::{auth_middleware, cors_middleware};
use utoipa_swagger_ui::SwaggerUi;

pub async fn gateway_service(
	surrealdb_ws: SurrealWsClient,
	surrealdb_mem: SurrealMemClient,
) -> Router {
	let state = AppState {
		surrealdb_ws,
		surrealdb_mem,
	};

	let public_routes = iam_public_routes();
	let protected_routes = iam_protected_routes();//.layer(from_fn(auth_middleware));

	Router::new()
		.route("/", get(Redirect::to("/docs")))
		.nest("/v1", public_routes.merge(protected_routes))
		.merge(SwaggerUi::new("/docs").url("/openapi.json", docs_router()))
		.layer(cors_middleware())
		.layer(Extension(state.clone()))
}
