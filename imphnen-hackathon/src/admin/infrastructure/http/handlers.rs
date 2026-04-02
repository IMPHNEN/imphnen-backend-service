use super::dto::*;
use crate::admin::domain::service::AdminService;
use axum::{
	Extension, Json,
	extract::{Path, Query},
	response::IntoResponse,
};
use imphnen_utils::{
	errors::AppError,
	response_format::{ApiMessage, ApiSuccess},
};
use std::sync::Arc;
use uuid::Uuid;

pub async fn admin_list_users(
	Extension(service): Extension<Arc<dyn AdminService>>,
	Query(q): Query<PageQuery>,
) -> Result<axum::response::Response, AppError> {
	let (users, total) = service.list_users(q.page, q.limit, q.search).await?;
	Ok(
		ApiSuccess(PagedResponse {
			data: users,
			total,
			page: q.page,
			limit: q.limit,
		})
		.into_response(),
	)
}

pub async fn admin_get_user(
	Extension(service): Extension<Arc<dyn AdminService>>,
	Path(user_id): Path<Uuid>,
) -> Result<axum::response::Response, AppError> {
	let user = service.get_user(user_id).await?;
	Ok(ApiSuccess(user).into_response())
}

pub async fn admin_set_admin(
	Extension(service): Extension<Arc<dyn AdminService>>,
	Path(user_id): Path<Uuid>,
	Json(body): Json<SetAdminRequest>,
) -> Result<ApiMessage, AppError> {
	service.set_admin(user_id, body.is_admin).await?;
	Ok(ApiMessage::ok("User admin status updated"))
}

pub async fn admin_delete_user(
	Extension(service): Extension<Arc<dyn AdminService>>,
	Path(user_id): Path<Uuid>,
) -> Result<ApiMessage, AppError> {
	service.delete_user(user_id).await?;
	Ok(ApiMessage::ok("User deleted"))
}

pub async fn admin_list_teams(
	Extension(service): Extension<Arc<dyn AdminService>>,
	Query(q): Query<PageQuery>,
) -> Result<axum::response::Response, AppError> {
	let (teams, total) = service.list_teams(q.page, q.limit, q.search).await?;
	Ok(
		ApiSuccess(PagedResponse {
			data: teams,
			total,
			page: q.page,
			limit: q.limit,
		})
		.into_response(),
	)
}

pub async fn admin_delete_team(
	Extension(service): Extension<Arc<dyn AdminService>>,
	Path(team_id): Path<Uuid>,
) -> Result<ApiMessage, AppError> {
	service.delete_team(team_id).await?;
	Ok(ApiMessage::ok("Team deleted"))
}

pub async fn admin_list_submissions(
	Extension(service): Extension<Arc<dyn AdminService>>,
	Query(q): Query<PageQuery>,
) -> Result<axum::response::Response, AppError> {
	let (subs, total) = service.list_submissions(q.page, q.limit, q.status).await?;
	Ok(
		ApiSuccess(PagedResponse {
			data: subs,
			total,
			page: q.page,
			limit: q.limit,
		})
		.into_response(),
	)
}

pub async fn admin_set_winner(
	Extension(service): Extension<Arc<dyn AdminService>>,
	Json(body): Json<SetWinnerRequest>,
) -> Result<ApiMessage, AppError> {
	service
		.set_winner(body.team_id, body.rank, body.prize)
		.await?;
	Ok(ApiMessage::ok("Winner set"))
}

pub async fn admin_remove_winner(
	Extension(service): Extension<Arc<dyn AdminService>>,
	Path(team_id): Path<Uuid>,
) -> Result<ApiMessage, AppError> {
	service.remove_winner(team_id).await?;
	Ok(ApiMessage::ok("Winner removed"))
}

pub async fn admin_list_winners(
	Extension(service): Extension<Arc<dyn AdminService>>,
) -> Result<axum::response::Response, AppError> {
	let rows = service.list_winners().await?;
	Ok(ApiSuccess(rows).into_response())
}
