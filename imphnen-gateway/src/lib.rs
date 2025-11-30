use axum::{
    Extension,
    Router,
    middleware::from_fn,
    response::Redirect,
    routing::get,
};
use imphnen_cms::{
    events_protected_routes,
    events_public_routes,
    testimonials_protected_routes,
    testimonials_public_routes,
};
use imphnen_dimentorin::dimentorin_router;
use imphnen_gacha::gacha_router;
use imphnen_iam::{
    iam_protected_routes,
    iam_public_routes,
    v1::auth::auth_repository::AuthRepoImpl,
};
use imphnen_libs::PostgresUserLookupService;
use imphnen_libs::{AppState, axum::PostgresClients};
use imphnen_middleware::{auth_middleware, cors_middleware, rate_limiting_middleware, security_headers_middleware};
use std::sync::Arc;
use utoipa_swagger_ui::SwaggerUi;

pub mod docs;
pub use docs::{ApiDoc, SecurityAddon, docs_router};

pub async fn gateway_service(
    postgres_clients: PostgresClients,
) -> Router {
    let state = AppState {
        postgres_connection: postgres_clients.main.clone(),
        user_lookup_service: Arc::new(PostgresUserLookupService::new()),
        auth_repository: Arc::new(AuthRepoImpl::new()),
    };

    let public_routes = Router::new()
        .merge(iam_public_routes().layer(from_fn(rate_limiting_middleware)))
        .merge(testimonials_public_routes())
        .merge(events_public_routes());

    let protected_routes = Router::new()
        .merge(iam_protected_routes())
        .merge(events_protected_routes())
        .merge(testimonials_protected_routes())
        .merge(dimentorin_router())
        .nest("/gacha", gacha_router())
        .layer(from_fn(auth_middleware));

    Router::new()
            .route("/", get(Redirect::to("/docs")))
            .nest("/v1", public_routes.merge(protected_routes))
            .merge(SwaggerUi::new("/docs").url("/openapi.json", docs_router()))
            .layer(cors_middleware())
            .layer(from_fn(security_headers_middleware))
            .layer(Extension(state))
}
