pub mod config;
pub mod common;
pub mod middleware;
pub mod admin;
pub mod auth;
pub mod certificates;
pub mod chat;
pub mod storage;
pub mod submissions;
pub mod teams;
pub mod users;
pub mod invitations;
pub mod join_requests;
pub mod winners;

pub use admin::hackathon_admin_routes;
pub use auth::hackathon_auth_routes;
pub use certificates::hackathon_certificates_routes;
pub use chat::build_chat_routes;
pub use storage::hackathon_storage_routes;
pub use submissions::hackathon_submissions_routes;
pub use teams::build_team_routes;
pub use users::hackathon_users_routes;
pub use invitations::build_invitation_routes;
pub use join_requests::build_join_request_routes;
pub use winners::hackathon_winners_routes;
pub use config::HackathonConfig;

use axum::Router;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use common::{hackathon_jwt::HackathonJwtService, supabase_client::SupabaseClient};

pub fn hackathon_router(db: DatabaseConnection, config: Arc<HackathonConfig>) -> Router {
    let pool = Arc::new(db.get_postgres_connection_pool().clone());
    let jwt = Arc::new(HackathonJwtService::new(&config.jwt_secret, config.jwt_expiry_hours));
    let supabase = Arc::new(SupabaseClient::new(
        config.supabase_url.clone(),
        config.supabase_anon_key.clone(),
        config.supabase_service_role_key.clone(),
        config.storage_bucket.clone(),
    ));

    Router::new()
        .merge(hackathon_auth_routes(pool.clone(), jwt.clone(), supabase.clone(), config.clone()))
        .merge(hackathon_users_routes(pool.clone(), jwt.clone()))
        .merge(build_team_routes(pool.clone(), jwt.clone()))
        .merge(build_invitation_routes(pool.clone(), jwt.clone()))
        .merge(build_join_request_routes(pool.clone(), jwt.clone()))
        .merge(build_chat_routes(pool.clone(), jwt.clone()))
        .merge(hackathon_submissions_routes(pool.clone(), jwt.clone()))
        .merge(hackathon_storage_routes(pool.clone(), jwt.clone(), supabase))
        .merge(hackathon_certificates_routes(pool.clone()))
        .merge(hackathon_winners_routes(pool.clone()))
        .merge(hackathon_admin_routes(pool, jwt))
}
