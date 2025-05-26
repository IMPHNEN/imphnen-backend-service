use axum::{
	Extension, Router, middleware::from_fn, response::Redirect, routing::get,
};
use imphnen_cms::{events_protected_routes, events_public_routes};
use imphnen_entities::{AppState, SurrealMemClient, SurrealWsClient};
use imphnen_gacha::gacha_router;
use imphnen_iam::{iam_protected_routes, iam_public_routes};
use imphnen_middleware::{auth_middleware, cors_middleware};
use utoipa_swagger_ui::SwaggerUi;

pub mod docs;
pub use docs::*;

pub async fn gateway_service(
	surrealdb_ws: SurrealWsClient,
	surrealdb_mem: SurrealMemClient,
) -> Router {
	let state = AppState {
		surrealdb_ws,
		surrealdb_mem,
	};

	let public_routes = Router::new()
		.merge(iam_public_routes())
		.merge(events_public_routes());

	let protected_routes = Router::new()
		.merge(iam_protected_routes())
		.merge(events_protected_routes())
		.merge(gacha_router())
		.layer(from_fn(auth_middleware));

	let routes = public_routes.merge(protected_routes);

	Router::new()
		.route("/", get(Redirect::to("/docs")))
		.nest("/v1", routes)
		.merge(SwaggerUi::new("/docs").url("/openapi.json", docs_router()))
		.layer(cors_middleware())
		.layer(Extension(state))
}
