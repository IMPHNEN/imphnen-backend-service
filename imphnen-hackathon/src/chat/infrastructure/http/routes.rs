use axum::{middleware::from_fn, routing::{delete, get, post}, Extension, Router};
use sqlx::PgPool;
use std::sync::Arc;
use crate::chat::application::chat_service::ChatServiceImpl;
use crate::chat::domain::service::ChatService;
use crate::chat::infrastructure::persistence::PostgresChatRepository;
use crate::common::hackathon_jwt::HackathonJwtService;
use crate::middleware::hackathon_auth::hackathon_auth_middleware;
use super::handlers::*;

pub fn build_chat_routes(pool: Arc<PgPool>, jwt: Arc<HackathonJwtService>) -> Router {
    let service: Arc<dyn ChatService> = Arc::new(ChatServiceImpl::new(
        Arc::new(PostgresChatRepository::new(pool.clone())),
    ));
    Router::new()
        .route("/chat/teams/:team_id", get(get_team_messages_handler).post(send_message_handler))
        .route("/chat/messages/:message_id", delete(delete_message_handler))
        .layer(Extension(service))
        .layer(Extension(jwt.clone()))
        .layer(Extension(pool))
        .layer(from_fn(hackathon_auth_middleware))
}
