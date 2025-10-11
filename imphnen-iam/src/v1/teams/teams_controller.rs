use crate::{AppState, MetaRequestDto};
use crate::{
	MessageResponseDto, ResponseListSuccessDto, ResponseSuccessDto,
	TeamsCreateRequestDto, TeamsDetailItemDto, TeamsListItemDto, permissions_guard,
	TeamsUpdateRequestDto, TeamInviteRequestDto, TeamAcceptInvitationRequestDto,
	TeamMemberDto, TeamsSearchQueryDto, PublicTeamsListItemDto, PublicTeamsDetailItemDto,
	AdminTeamsListItemDto, AdminTeamsDetailItemDto, PermissionsEnum
};
use super::super::teams::{TeamsRepository, TeamMembersSchema};
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
  Extension(state): Extension<AppState>,
  axum::extract::Query(meta): axum::extract::Query<MetaRequestDto>,
) -> impl IntoResponse {
  TeamsService::get_public_team_list(&state, meta).await
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
  Extension(state): Extension<AppState>,
  Path(id): Path<String>,
) -> impl IntoResponse {
  TeamsService::get_public_team_by_id(&state, id).await
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

	// Try to treat this request as an admin first; if the caller has ManageAllTeams
	// permission, route to the admin update. Otherwise fall back to normal authenticated
	// update which enforces leader-only rules.
	let state_clone = state.clone();
	match crate::permissions_guard(headers.clone(), axum::Extension(state_clone.clone()), vec![PermissionsEnum::ManageAllTeams]).await {
		Ok((claims, state)) => {
			// Caller is admin
			TeamsService::update_team_admin(&state, claims, id, payload).await
		}
		Err(_) => {
			// Not admin - proceed with normal authenticated flow
			authenticated(headers, Extension(state), move |claims, state| TeamsService::update_team(&state, claims, id, payload)).await
		}
	}
}

#[derive(serde::Deserialize)]
pub struct AddTeamMemberRequestDto {
	pub user_id: String,
	pub role: Option<String>,
}

pub async fn post_add_team_member(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Path(team_id): Path<String>,
	Json(payload): Json<AddTeamMemberRequestDto>,
) -> impl IntoResponse {
	// Determine caller and whether they have admin permissions
	let state_clone = state.clone();
	let is_admin = crate::permissions_guard(headers.clone(), axum::Extension(state_clone.clone()), vec![PermissionsEnum::ManageAllTeams]).await.is_ok();

	// Authenticate the caller (will return 401 if no token)
	let auth = permissions_guard(headers, axum::Extension(state.clone()), vec![/* no specific perms */]).await;
	let (claims, state) = match auth {
		Ok((c, s)) => (c, s),
		Err(response) => return response,
	};

	// Permission: admins can add anyone; otherwise only team leader or existing member can add
	let repo = TeamsRepository::new(&state);
	let thing_id = imphnen_utils::make_thing_from_enum(imphnen_libs::ResourceEnum::Teams, &team_id);
	let team = match repo.query_team_by_id(&thing_id).await {
		Ok(t) => t,
		Err(_) => return crate::common_response(axum::http::StatusCode::NOT_FOUND, "Team not found"),
	};

	if !is_admin {
		let user_thing = imphnen_utils::make_thing_from_enum(imphnen_libs::ResourceEnum::Users, &claims.user_id);
		let is_member = repo.query_is_team_member(&thing_id, &user_thing).await.unwrap_or(false);
		let is_leader = team.leader_id.id.to_raw() == claims.user_id;
		if !is_member && !is_leader {
			return crate::common_response(axum::http::StatusCode::FORBIDDEN, "Only team leader or members can add a member");
		}
	}

	// Build member schema and add via repository
	let member_schema = TeamMembersSchema::create(team_id.clone(), payload.user_id.clone(), payload.role.clone());
	match repo.query_add_team_member(member_schema).await {
		Ok(msg) => crate::success_response(crate::ResponseSuccessDto { data: msg }),
		Err(e) => crate::common_response(axum::http::StatusCode::BAD_REQUEST, &e.to_string()),
	}
}

pub async fn delete_remove_team_member(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Path((team_id, user_id)): Path<(String, String)>,
) -> impl IntoResponse {
	let state_clone = state.clone();
	let is_admin = crate::permissions_guard(headers.clone(), axum::Extension(state_clone.clone()), vec![PermissionsEnum::ManageAllTeams]).await.is_ok();

	let auth = permissions_guard(headers, axum::Extension(state.clone()), vec![]).await;
	let (claims, state) = match auth {
		Ok((c, s)) => (c, s),
		Err(response) => return response,
	};

	let repo = TeamsRepository::new(&state);
	let thing_id = imphnen_utils::make_thing_from_enum(imphnen_libs::ResourceEnum::Teams, &team_id);
	let team = match repo.query_team_by_id(&thing_id).await {
		Ok(t) => t,
		Err(_) => return crate::common_response(axum::http::StatusCode::NOT_FOUND, "Team not found"),
	};

	if !is_admin {
		// Only leader can remove members
		if team.leader_id.id.to_raw() != claims.user_id {
			return crate::common_response(axum::http::StatusCode::FORBIDDEN, "Only team leader can remove members");
		}
	}

	let user_thing = imphnen_utils::make_thing_from_enum(imphnen_libs::ResourceEnum::Users, &user_id);
	match repo.query_remove_team_member(&thing_id, &user_thing).await {
		Ok(msg) => crate::success_response(crate::ResponseSuccessDto { data: msg }),
		Err(e) => crate::common_response(axum::http::StatusCode::BAD_REQUEST, &e.to_string()),
	}
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
		TeamsService::get_admin_team_list(&state, meta)
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
		TeamsService::get_admin_team_by_id(&state, id)
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
		TeamsService::get_admin_team_members(&state, id)
	}).await
}

pub fn teams_router() -> Router {
	Router::new()
		.route("/", axum::routing::get(get_team_list))
		.route("/{id}", axum::routing::get(get_team_by_id))
		.route("/create", axum::routing::post(post_create_team))
		.route("/update/{id}", axum::routing::put(put_update_team))
		.route("/delete/{id}", axum::routing::delete(delete_team))
		.route("/{id}/invite", axum::routing::post(post_invite_team_members))
		.route("/accept/{token}", axum::routing::post(post_accept_invitation))
		.route("/search", axum::routing::get(get_public_team_search))
		.route("/{id}/members", axum::routing::get(get_team_members))
			.route("/{id}/members", axum::routing::post(post_add_team_member))
			.route("/{id}/members/{user_id}", axum::routing::delete(delete_remove_team_member))
		.route("/{id}/leave", axum::routing::post(post_leave_team))
		.route("/leave-me", axum::routing::post(post_leave_current_team))
}
