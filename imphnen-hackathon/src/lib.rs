pub mod admin;
pub mod certificates;
pub mod chat;
pub mod common;
pub mod invitations;
pub mod join_requests;
pub mod middleware;
pub mod storage;
pub mod submissions;
pub mod teams;
pub mod users;
pub mod winners;

pub use admin::hackathon_admin_routes;
pub use certificates::hackathon_certificates_routes;
pub use chat::build_chat_routes;
pub use invitations::build_invitation_routes;
pub use join_requests::build_join_request_routes;
pub use storage::hackathon_storage_routes;
pub use submissions::hackathon_submissions_routes;
pub use teams::build_team_routes;
pub use users::hackathon_users_routes;
pub use winners::hackathon_winners_routes;

use axum::Router;
use imphnen_storage::MinioService;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub fn hackathon_router(
	db: DatabaseConnection,
	minio: Arc<MinioService>,
) -> Router {
	let pool = Arc::new(db.get_postgres_connection_pool().clone());

	Router::new()
		.merge(hackathon_users_routes(pool.clone()))
		.merge(build_team_routes(pool.clone()))
		.merge(build_invitation_routes(pool.clone()))
		.merge(build_join_request_routes(pool.clone()))
		.merge(build_chat_routes(pool.clone()))
		.merge(hackathon_submissions_routes(pool.clone()))
		.merge(hackathon_storage_routes(pool.clone(), minio))
		.merge(hackathon_certificates_routes(pool.clone()))
		.merge(hackathon_winners_routes(pool.clone()))
		.merge(hackathon_admin_routes(pool))
}
