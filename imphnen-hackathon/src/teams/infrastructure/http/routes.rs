use super::handlers::*;
use crate::middleware::hackathon_auth::hackathon_auth_middleware;
use crate::teams::application::team_service::TeamServiceImpl;
use crate::teams::domain::service::TeamService;
use crate::teams::infrastructure::persistence::PostgresTeamRepository;
use axum::{
	Extension, Router,
	middleware::from_fn,
	routing::{delete, get, post, put},
};
use sqlx::PgPool;
use std::sync::Arc;

pub fn build_team_routes(pool: Arc<PgPool>) -> Router {
	let repo = Arc::new(PostgresTeamRepository::new(pool.clone()));
	let service: Arc<dyn TeamService> = Arc::new(TeamServiceImpl::new(repo));

	let public = Router::new()
		.route("/teams/browse", get(browse_teams_handler))
		.route("/teams/:team_id", get(get_team_handler))
		.layer(Extension(service.clone()));

	let protected = Router::new()
		.route("/teams", post(create_team_handler))
		.route("/teams/my", get(get_my_teams_handler))
		.route(
			"/teams/:team_id",
			put(update_team_handler).delete(delete_team_handler),
		)
		.route("/teams/:team_id/leave", post(leave_team_handler))
		.route(
			"/teams/:team_id/members/:member_id",
			delete(remove_member_handler),
		)
		.layer(Extension(service))
		.layer(Extension(pool.clone()))
		.layer(from_fn(hackathon_auth_middleware));

	Router::new().merge(public).merge(protected)
}
