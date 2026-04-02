use axum::{middleware::from_fn, routing::{get, post}, Extension, Router};
use sqlx::PgPool;
use std::sync::Arc;
use crate::join_requests::application::join_request_service::JoinRequestServiceImpl;
use crate::join_requests::domain::service::JoinRequestService;
use crate::join_requests::infrastructure::persistence::PostgresJoinRequestRepository;
use crate::common::hackathon_jwt::HackathonJwtService;
use crate::middleware::hackathon_auth::hackathon_auth_middleware;
use super::handlers::*;

pub fn build_join_request_routes(pool: Arc<PgPool>, jwt: Arc<HackathonJwtService>) -> Router {
    let service: Arc<dyn JoinRequestService> = Arc::new(JoinRequestServiceImpl::new(
        Arc::new(PostgresJoinRequestRepository::new(pool.clone())),
    ));
    Router::new()
        .route("/join-requests/teams/:team_id", post(create_join_request_handler))
        .route("/join-requests/my", get(get_my_join_requests_handler))
        .route("/join-requests/teams/:team_id/pending", get(get_team_join_requests_handler))
        .route("/join-requests/:request_id/respond", post(respond_to_join_request_handler))
        .layer(Extension(service))
        .layer(Extension(jwt.clone()))
        .layer(Extension(pool))
        .layer(from_fn(hackathon_auth_middleware))
}
