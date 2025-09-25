use crate::{AppState, MetaRequestDto};
use crate::{
	MessageResponseDto, ResponseListSuccessDto, ResponseSuccessDto,
	TeamsCreateRequestDto, TeamsDetailItemDto, TeamsListItemDto, permissions_guard,
	TeamsUpdateRequestDto, TeamInviteRequestDto, TeamAcceptInvitationRequestDto,
	TeamMemberDto, TeamsSearchQueryDto, PublicTeamsListItemDto, PublicTeamsDetailItemDto,
	AdminTeamsListItemDto, AdminTeamsDetailItemDto, PermissionsEnum
};
use axum::response::Response;
use axum::extract::Path;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use super::teams_service::{TeamsServiceTrait, TeamsService};
use axum::Router;

// Helper function for endpoints requiring authentication without specific permissions
async fn authenticated<F, Fut>(
	headers: HeaderMap,
	state: Extension<AppState>,
	f: F,
) -> Response
where
	F: FnOnce(crate::Claims, AppState) -> Fut,
	Fut: std::future::Future<Output = Response> + Send,
{
	match permissions_guard(headers, state, vec![]).await {
		Ok((claims, state)) => f(claims, state).await,
		Err(response) => response,
	}
}

// Helper function for endpoints requiring specific permissions
async fn with_perms<F, Fut>(
	headers: HeaderMap,
	state: Extension<AppState>,
	perms: Vec<PermissionsEnum>,
	f: F,
) -> Response
where
	F: FnOnce(crate::Claims, AppState) -> Fut,
	Fut: std::future::Future<Output = Response> + Send,
{
	match permissions_guard(headers, state, perms).await {
		Ok((claims, state)) => f(claims, state).await,
		Err(response) => response,
	}
}

#[utoipa::path(
	get,
	security(
		("Bearer" = [])
	),
	path = "/v1/teams",
	params(
		("page" = Option<i64>, Query, description = "Page number"),
		("per_page" = Option<i64>, Query, description = "Items per page"),
		("search" = Option<String>, Query, description = "Search keyword"),
		("sort_by" = Option<String>, Query, description = "Sort by field"),
		("order" = Option<String>, Query, description = "Order ASC or DESC"),
		("filter" = Option<String>, Query, description = "Filter value"),
		("filter_by" = Option<String>, Query, description = "Field to filter by"),
	),
	responses(
		(status = 200, description = "Get team list", body = ResponseListSuccessDto<Vec<TeamsListItemDto>>),
		(status = 200, description = "Get public team list", body = ResponseListSuccessDto<Vec<PublicTeamsListItemDto>>)
	),
	tag = "Teams"
)]
pub async fn get_team_list(
	headers: Option<HeaderMap>,
	Extension(state): Extension<AppState>,
	axum::extract::Query(meta): axum::extract::Query<MetaRequestDto>,
) -> Response {
	let state = state;
	match headers {
		Some(headers) => {
			match permissions_guard(headers, axum::Extension(state.clone()), vec![]).await {
				Ok((_claims, state)) => TeamsService::get_team_list(&state, meta).await,
				Err(_) => TeamsService::get_public_team_list(&state, meta).await,
			}
		},
		None => TeamsService::get_public_team_list(&state, meta).await,
	}
}

#[utoipa::path(
	get,
	path = "/v1/teams/{id}",
	params(
		("id" = String, Path, description = "Team ID")
	),
	responses(
		(status = 200, description = "Get team by ID", body = ResponseSuccessDto<TeamsDetailItemDto>),
		(status = 200, description = "Get public team by ID", body = ResponseSuccessDto<PublicTeamsDetailItemDto>)
	),
	tag = "Teams"
)]
pub async fn get_team_by_id(
	headers: Option<HeaderMap>,
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
) -> Response {
	let state = state;
	match headers {
		Some(headers) => {
			match permissions_guard(headers, axum::Extension(state.clone()), vec![]).await {
				Ok((_claims, state)) => TeamsService::get_team_by_id(&state, id).await,
				Err(_) => TeamsService::get_public_team_by_id(&state, id).await,
			}
		},
		None => TeamsService::get_public_team_by_id(&state, id).await,
	}
}

#[utoipa::path(
	post,
	security(
		("Bearer" = [])
	),
	path = "/v1/teams/create",
	request_body = TeamsCreateRequestDto,
	responses(
		(status = 200, description = "Create new team", body = ResponseSuccessDto<serde_json::Value>)
	),
	tag = "Teams"
)]
pub async fn post_create_team(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Json(payload): Json<TeamsCreateRequestDto>,
) -> impl IntoResponse {
	authenticated(headers, Extension(state), move |claims, state| TeamsService::create_team(&state, claims, payload)).await
}

#[utoipa::path(
	put,
	security(
		("Bearer" = [])
	),
	path = "/v1/teams/update/{id}",
	params(
		("id" = String, Path, description = "Team ID")
	),
	request_body = TeamsUpdateRequestDto,
	responses(
		(status = 200, description = "Update team", body = MessageResponseDto)
	),
	tag = "Teams"
)]
pub async fn put_update_team(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
	Json(payload): Json<TeamsUpdateRequestDto>,
) -> impl IntoResponse {
	authenticated(headers, Extension(state), move |claims, state| TeamsService::update_team(&state, claims, id, payload)).await
}

#[utoipa::path(
	delete,
	security(
		("Bearer" = [])
	),
	path = "/v1/teams/delete/{id}",
	params(
		("id" = String, Path, description = "Team ID")
	),
	responses(
		(status = 200, description = "Delete team", body = MessageResponseDto)
	),
	tag = "Teams"
)]
pub async fn delete_team(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
) -> impl IntoResponse {
	authenticated(headers, Extension(state), move |claims, state| TeamsService::delete_team(&state, claims, id)).await
}

#[utoipa::path(
	post,
	security(
		("Bearer" = [])
	),
	path = "/v1/teams/{id}/invite",
	params(
		("id" = String, Path, description = "Team ID")
	),
	request_body = TeamInviteRequestDto,
	responses(
		(status = 200, description = "Invite team members", body = ResponseSuccessDto<serde_json::Value>)
	),
	tag = "Teams"
)]
pub async fn post_invite_team_members(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Path(team_id): Path<String>,
	Json(payload): Json<TeamInviteRequestDto>,
) -> impl IntoResponse {
	authenticated(headers, Extension(state), move |claims, state| TeamsService::invite_team_members(&state, claims, team_id, payload)).await
}

#[utoipa::path(
	post,
	security(
		("Bearer" = [])
	),
	path = "/v1/teams/accept/{token}",
	params(
		("token" = String, Path, description = "Invitation token")
	),
	responses(
		(status = 200, description = "Accept team invitation", body = MessageResponseDto)
	),
	tag = "Teams"
)]
pub async fn post_accept_invitation(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Path(token): Path<String>,
) -> impl IntoResponse {
	let accept_dto = TeamAcceptInvitationRequestDto { token };
	authenticated(headers, Extension(state), move |claims, state| TeamsService::accept_invitation(&state, claims, accept_dto)).await
}

#[utoipa::path(
	get,
	path = "/v1/teams/search",
	params(
		("query" = Option<String>, Query, description = "Search query"),
		("open" = Option<bool>, Query, description = "Filter by open teams"),
		("skills" = Option<Vec<String>>, Query, description = "Filter by required skills"),
		("location" = Option<String>, Query, description = "Filter by location"),
		("page" = Option<i64>, Query, description = "Page number"),
		("per_page" = Option<i64>, Query, description = "Items per page"),
	),
	responses(
		(status = 200, description = "Search teams", body = ResponseListSuccessDto<Vec<PublicTeamsListItemDto>>)
	),
	tag = "Teams"
)]
pub async fn get_public_team_search(
	Extension(state): Extension<AppState>,
	axum::extract::Query(search_params): axum::extract::Query<TeamsSearchQueryDto>,
) -> impl IntoResponse {
	TeamsService::search_teams(&state, search_params).await
}

#[utoipa::path(
	get,
	security(
		("Bearer" = [])
	),
	path = "/v1/teams/{id}/members",
	params(
		("id" = String, Path, description = "Team ID")
	),
	responses(
		(status = 200, description = "Get team members", body = ResponseSuccessDto<Vec<TeamMemberDto>>)
	),
	tag = "Teams"
)]
pub async fn get_team_members(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
) -> impl IntoResponse {
	authenticated(headers, Extension(state), move |claims, state| TeamsService::get_team_members(&state, claims, id)).await
}

#[utoipa::path(
	post,
	security(
		("Bearer" = [])
	),
	path = "/v1/teams/{id}/leave",
	params(
		("id" = String, Path, description = "Team ID")
	),
	responses(
		(status = 200, description = "Leave specific team", body = MessageResponseDto)
	),
	tag = "Teams"
)]
pub async fn post_leave_team(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
) -> impl IntoResponse {
	authenticated(headers, Extension(state), move |claims, state| TeamsService::leave_team(&state, claims, id)).await
}

#[utoipa::path(
	post,
	security(
		("Bearer" = [])
	),
	path = "/v1/teams/leave-me",
	responses(
		(status = 200, description = "Leave current team", body = MessageResponseDto)
	),
	tag = "Teams"
)]
pub async fn post_leave_current_team(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
) -> impl IntoResponse {
	authenticated(headers, Extension(state), |claims, state| TeamsService::leave_current_team(&state, claims)).await
}

#[utoipa::path(
	get,
	security(
		("Bearer" = [])
	),
	path = "/v1/teams/admin",
	params(
		("page" = Option<i64>, Query, description = "Page number"),
		("per_page" = Option<i64>, Query, description = "Items per page"),
		("search" = Option<String>, Query, description = "Search keyword"),
		("sort_by" = Option<String>, Query, description = "Sort by field"),
		("order" = Option<String>, Query, description = "Order ASC or DESC"),
		("filter" = Option<String>, Query, description = "Filter value"),
		("filter_by" = Option<String>, Query, description = "Field to filter by"),
	),
	responses(
		(status = 200, description = "Get admin team list", body = ResponseListSuccessDto<Vec<AdminTeamsListItemDto>>)
	),
	tag = "Teams - Admin"
)]
pub async fn get_admin_team_list(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	axum::extract::Query(meta): axum::extract::Query<MetaRequestDto>,
) -> Response {
	let state = state;
	with_perms(headers, axum::Extension(state), vec![PermissionsEnum::ReadListTeams], move |_claims, state| {
	    let response = TeamsService::get_admin_team_list(&state, meta);
	    response
	}).await
}

#[utoipa::path(
	get,
	security(
		("Bearer" = [])
	),
	path = "/v1/teams/admin/{id}",
	params(
		("id" = String, Path, description = "Team ID")
	),
	responses(
		(status = 200, description = "Get admin team by ID", body = ResponseSuccessDto<AdminTeamsDetailItemDto>)
	),
	tag = "Teams - Admin"
)]
pub async fn get_admin_team_by_id(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
) -> Response {
	let state = state;
	with_perms(headers, axum::Extension(state), vec![PermissionsEnum::ReadDetailTeams], move |_claims, state| {
	    let response = TeamsService::get_admin_team_by_id(&state, id);
	    response
	}).await
}

#[utoipa::path(
	get,
	security(
		("Bearer" = [])
	),
	path = "/v1/teams/admin/{id}/members",
	params(
		("id" = String, Path, description = "Team ID")
	),
	responses(
		(status = 200, description = "Get admin team members", body = ResponseSuccessDto<Vec<TeamMemberDto>>)
	),
	tag = "Teams - Admin"
)]
pub async fn get_admin_team_members(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
) -> Response {
	let state = state;
	with_perms(headers, axum::Extension(state), vec![PermissionsEnum::ReadDetailTeams], move |_claims, state| {
	    let response = TeamsService::get_admin_team_members(&state, id);
	    response
	}).await
}

pub fn teams_router() -> Router {
	Router::new()
		.route("/", axum::routing::get(get_team_list))
		.route("/:id", axum::routing::get(get_team_by_id))
		.route("/create", axum::routing::post(post_create_team))
		.route("/update/:id", axum::routing::put(put_update_team))
		.route("/delete/:id", axum::routing::delete(delete_team))
		.route("/:id/invite", axum::routing::post(post_invite_team_members))
		.route("/accept/:token", axum::routing::post(post_accept_invitation))
		.route("/search", axum::routing::get(get_public_team_search))
		.route("/:id/members", axum::routing::get(get_team_members))
		.route("/:id/leave", axum::routing::post(post_leave_team))
		.route("/leave-me", axum::routing::post(post_leave_current_team))
		.route("/admin", axum::routing::get(get_admin_team_list))
		.route("/admin/:id", axum::routing::get(get_admin_team_by_id))
		.route("/admin/:id/members", axum::routing::get(get_admin_team_members))
}
