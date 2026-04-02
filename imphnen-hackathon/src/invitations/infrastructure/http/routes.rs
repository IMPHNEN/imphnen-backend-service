use axum::{middleware::from_fn, routing::{get, post}, Extension, Router};
use sqlx::PgPool;
use std::sync::Arc;
use crate::invitations::application::invitation_service::InvitationServiceImpl;
use crate::invitations::domain::service::InvitationService;
use crate::invitations::infrastructure::persistence::PostgresInvitationRepository;
use crate::middleware::hackathon_auth::hackathon_auth_middleware;
use super::handlers::*;

pub fn build_invitation_routes(pool: Arc<PgPool>) -> Router {
    let service: Arc<dyn InvitationService> = Arc::new(InvitationServiceImpl::new(
        Arc::new(PostgresInvitationRepository::new(pool.clone())),
    ));
    Router::new()
        .route("/invitations/my", get(get_my_invitations_handler))
        .route("/invitations/:invitation_id/respond", post(respond_to_invitation_handler))
        .route("/invitations/teams/:team_id/invite", post(invite_team_member_handler))
        .layer(Extension(service))
        .layer(Extension(pool))
        .layer(from_fn(hackathon_auth_middleware))
}
