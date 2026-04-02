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
use imphnen_dimentorin::{
    mentors_public_routes, mentors_protected_routes,
    sessions_public_routes, sessions_protected_routes,
};
use imphnen_gacha::gacha_router;
use imphnen_iam::{
    auth_public_routes,
    permissions_protected_routes,
    roles_protected_routes,
    users_protected_routes,
};
use imphnen_libs::services::PostgresAuthRepository as AuthRepoImpl;
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

    let db = state.postgres_connection.conn.clone();
    let state_arc = Arc::new(state.clone());

    let public_routes = Router::new()
        .merge(auth_public_routes(db.clone(), Arc::clone(&state_arc)).layer(from_fn(rate_limiting_middleware)))
        .merge(testimonials_public_routes(db.clone()))
        .merge(events_public_routes(db.clone()))
        .merge(mentors_public_routes(db.clone(), Arc::clone(&state_arc)))
        .merge(sessions_public_routes(db.clone()));

    let protected_routes = Router::new()
        .merge(users_protected_routes(db.clone(), Arc::clone(&state_arc)))
        .merge(roles_protected_routes(db.clone(), Arc::clone(&state_arc)))
        .merge(permissions_protected_routes(db.clone(), Arc::clone(&state_arc)))
        .merge(events_protected_routes(db.clone()))
        .merge(testimonials_protected_routes(db.clone()))
        .merge(mentors_protected_routes(db.clone(), Arc::clone(&state_arc)))
        .merge(sessions_protected_routes(db.clone(), Arc::clone(&state_arc)))
        .nest("/gacha", gacha_router(db.clone(), Arc::clone(&state_arc)))
        .layer(from_fn(auth_middleware));

    Router::new()
            .route("/", get(Redirect::to("/docs")))
            .nest("/v1", public_routes.merge(protected_routes))
            .merge(SwaggerUi::new("/docs").url("/openapi.json", docs_router()))
            .layer(cors_middleware())
            .layer(from_fn(security_headers_middleware))
            .layer(Extension(state))
}
