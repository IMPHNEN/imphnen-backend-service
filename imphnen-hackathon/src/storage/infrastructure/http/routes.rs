use super::handlers::*;
use crate::middleware::hackathon_auth::hackathon_auth_middleware;
use crate::storage::application::storage_service::StorageServiceImpl;
use crate::storage::domain::service::StorageService;
use axum::{Extension, Router, middleware::from_fn, routing::post};
use imphnen_storage::MinioService;
use sqlx::PgPool;
use std::sync::Arc;

pub fn hackathon_storage_routes(
	pool: Arc<PgPool>,
	minio: Arc<MinioService>,
) -> Router {
	let service: Arc<dyn StorageService> = Arc::new(StorageServiceImpl::new(minio));
	Router::new()
		.route("/upload", post(upload_file_handler))
		.route("/upload/avatar", post(upload_avatar_handler))
		.route("/upload/team", post(upload_team_handler))
		.route("/upload/submission", post(upload_submission_handler))
		.layer(Extension(service))
		.layer(from_fn(hackathon_auth_middleware))
		.layer(Extension(pool))
}
