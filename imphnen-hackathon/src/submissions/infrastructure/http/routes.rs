use super::handlers::*;
use crate::middleware::hackathon_auth::hackathon_auth_middleware;
use crate::submissions::application::submission_service::SubmissionServiceImpl;
use crate::submissions::domain::service::SubmissionService;
use crate::submissions::infrastructure::persistence::PostgresSubmissionRepository;
use axum::{
	Extension, Router,
	middleware::from_fn,
	routing::{get, post, put},
};
use sqlx::PgPool;
use std::sync::Arc;

pub fn hackathon_submissions_routes(pool: Arc<PgPool>) -> Router {
	let service: Arc<dyn SubmissionService> = Arc::new(SubmissionServiceImpl::new(
		Arc::new(PostgresSubmissionRepository::new(pool.clone())),
	));
	Router::new()
		.route(
			"/submissions/teams/{team_id}",
			get(get_team_submission_handler).post(create_submission_handler),
		)
		.route(
			"/submissions/{submission_id}",
			put(update_submission_handler),
		)
		.route(
			"/submissions/{submission_id}/submit",
			post(submit_project_handler),
		)
		.route(
			"/submissions/{submission_id}/confirm",
			post(confirm_submission_handler),
		)
		.route(
			"/submissions/{submission_id}/cancel",
			post(cancel_submission_handler),
		)
		.layer(Extension(service))
		.layer(Extension(pool))
		.layer(from_fn(hackathon_auth_middleware))
}
