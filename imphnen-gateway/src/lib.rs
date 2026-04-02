use axum::{
	Extension, Router, middleware::from_fn, response::Redirect, routing::get,
};
use imphnen_cms::{
	events_protected_routes, events_public_routes, qr_router, testimonials_protected_routes,
	testimonials_public_routes,
};
use imphnen_dimentorin::{
	mentors_protected_routes, mentors_public_routes, sessions_protected_routes,
	sessions_public_routes,
};
use imphnen_gacha::gacha_router;
use imphnen_hackathon::hackathon_router;
use imphnen_iam::{
	auth_public_routes, permissions_protected_routes, roles_protected_routes,
	users_protected_routes,
};
use imphnen_libs::PostgresUserLookupService;
use imphnen_libs::services::PostgresAuthRepository as AuthRepoImpl;
use imphnen_libs::{AppState, axum::PostgresClients};
use imphnen_middleware::{
	auth_middleware, cors_middleware, rate_limiting_middleware,
	security_headers_middleware,
};
use imphnen_storage::{MinioConfig, create_minio_service_from_config};
use std::sync::Arc;
use utoipa_swagger_ui::SwaggerUi;

pub mod docs;
pub use docs::{ApiDoc, SecurityAddon, docs_router};

pub async fn gateway_service(postgres_clients: PostgresClients) -> Router {
	let state = AppState {
		postgres_connection: postgres_clients.main.clone(),
		user_lookup_service: Arc::new(PostgresUserLookupService::new()),
		auth_repository: Arc::new(AuthRepoImpl::new()),
	};

	let db = state.postgres_connection.conn.clone();
	let state_arc = Arc::new(state.clone());
	let minio = Arc::new(
		create_minio_service_from_config(
			MinioConfig::from_env().expect("MinIO config required"),
		)
		.await
		.expect("Failed to create MinIO service"),
	);

	let iam_routes = Router::new()
		.merge(
			auth_public_routes(db.clone(), Arc::clone(&state_arc))
				.layer(from_fn(rate_limiting_middleware)),
		)
		.merge(
			Router::new()
				.merge(users_protected_routes(db.clone(), Arc::clone(&state_arc)))
				.merge(roles_protected_routes(db.clone(), Arc::clone(&state_arc)))
				.merge(permissions_protected_routes(db.clone(), Arc::clone(&state_arc)))
				.layer(from_fn(auth_middleware)),
		);

	let cms_routes = Router::new()
		.merge(testimonials_public_routes(db.clone()))
		.merge(events_public_routes(db.clone()))
		.merge(
			Router::new()
				.merge(events_protected_routes(db.clone()))
				.merge(testimonials_protected_routes(db.clone()))
				.layer(from_fn(auth_middleware)),
		);

	let dimentorin_routes = Router::new()
		.merge(mentors_public_routes(db.clone(), Arc::clone(&state_arc)))
		.merge(sessions_public_routes(db.clone()))
		.merge(
			Router::new()
				.merge(mentors_protected_routes(db.clone(), Arc::clone(&state_arc)))
				.merge(sessions_protected_routes(db.clone(), Arc::clone(&state_arc)))
				.layer(from_fn(auth_middleware)),
		);

	let gacha_routes = gacha_router(db.clone(), Arc::clone(&state_arc))
		.layer(from_fn(auth_middleware));

	Router::new()
		.route("/", get(Redirect::to("/docs")))
		.nest("/v1/iam", iam_routes)
		.nest("/v1/landing/cms", cms_routes)
		.nest("/v1/dimentorin", dimentorin_routes)
		.nest("/v1/gacha", gacha_routes)
		.nest("/v1/hackathon", hackathon_router(db.clone(), minio))
		.nest("/v1/qr", qr_router(db.clone()))
		.merge(SwaggerUi::new("/docs").url("/openapi.json", docs_router()))
		.layer(cors_middleware())
		.layer(from_fn(security_headers_middleware))
		.layer(Extension(state))
}
