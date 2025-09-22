use crate::{AppState, MetaRequestDto};
use crate::{
	MessageResponseDto, ResponseListSuccessDto, ResponseSuccessDto,
	TeamsCreateRequestDto, TeamsDetailItemDto, TeamsListItemDto, permissions_guard,
	TeamsUpdateRequestDto, TeamInviteRequestDto, TeamAcceptInvitationRequestDto,
	TeamMemberDto, TeamsSearchQueryDto, PublicTeamsListItemDto, PublicTeamsDetailItemDto
};
use axum::extract::Path;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::{Extension, Json};

use super::teams_service::{TeamsServiceTrait, TeamsService};

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
		(status = 200, description = "Get team list", body = ResponseListSuccessDto<Vec<TeamsListItemDto>>)
	),
	tag = "Teams"
)]
pub async fn get_team_list(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	axum::extract::Query(meta): axum::extract::Query<MetaRequestDto>,
) -> impl IntoResponse {
	match permissions_guard(
		headers,
		Extension(state),
		vec![],
	)
	.await
	{
		Ok((_claims, state)) => TeamsService::get_team_list(&state, meta).await,
		Err(response) => response,
	}
}

#[utoipa::path(
get,
security(
       ("Bearer" = [])
   ),
path = "/v1/teams/detail/{id}",
params(
	("id" = String, Path, description = "Team ID")
),
responses(
	(status = 200, description = "Get team by ID", body = ResponseSuccessDto<TeamsDetailItemDto>)
),
tag = "Teams"
)]
pub async fn get_team_by_id(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
) -> impl IntoResponse {
	match permissions_guard(
		headers,
		Extension(state),
		vec![],
	)
	.await
	{
		Ok((_claims, state)) => TeamsService::get_team_by_id(&state, id).await,
		Err(response) => response,
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
	match permissions_guard(
		headers,
		Extension(state),
		vec![],
	)
	.await
	{
		Ok((claims, state)) => TeamsService::create_team(&state, claims, payload).await,
		Err(response) => response,
	}
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
	match permissions_guard(
		headers,
		Extension(state),
		vec![],
	)
	.await
	{
		Ok((claims, state)) => TeamsService::update_team(&state, claims, id, payload).await,
		Err(response) => response,
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
	match permissions_guard(
		headers,
		Extension(state),
		vec![],
	)
	.await
	{
		Ok((claims, state)) => TeamsService::delete_team(&state, claims, id).await,
		Err(response) => response,
	}
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
	match permissions_guard(
		headers,
		Extension(state),
		vec![],
	)
	.await
	{
		Ok((claims, state)) => TeamsService::invite_team_members(&state, claims, team_id, payload).await,
		Err(response) => response,
	}
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
	match permissions_guard(
		headers,
		Extension(state),
		vec![],
	)
	.await
	{
		Ok((claims, state)) => TeamsService::accept_invitation(&state, claims, accept_dto).await,
		Err(response) => response,
	}
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
	match permissions_guard(
		headers,
		Extension(state),
		vec![],
	)
	.await
	{
		Ok((claims, state)) => TeamsService::get_team_members(&state, claims, id).await,
		Err(response) => response,
	}
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
		(status = 200, description = "Leave team", body = MessageResponseDto)
	),
	tag = "Teams"
)]
pub async fn post_leave_team(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
) -> impl IntoResponse {
	match permissions_guard(
		headers,
		Extension(state),
		vec![],
	)
	.await
	{
		Ok((claims, state)) => TeamsService::leave_team(&state, claims, id).await,
		Err(response) => response,
	}
}

#[utoipa::path(
	get,
	path = "/v1/teams/public",
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
		(status = 200, description = "Get public team list", body = ResponseListSuccessDto<Vec<PublicTeamsListItemDto>>)
	),
	tag = "Teams"
)]
pub async fn get_public_team_list(
	Extension(state): Extension<AppState>,
	axum::extract::Query(meta): axum::extract::Query<MetaRequestDto>,
) -> impl IntoResponse {
	TeamsService::get_team_list(&state, meta).await
}

#[utoipa::path(
	get,
	path = "/v1/teams/public/{id}",
	params(
		("id" = String, Path, description = "Team ID")
	),
	responses(
		(status = 200, description = "Get public team by ID", body = ResponseSuccessDto<PublicTeamsDetailItemDto>)
	),
	tag = "Teams"
)]
pub async fn get_public_team_by_id(
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
) -> impl IntoResponse {
	TeamsService::get_team_by_id(&state, id).await
}