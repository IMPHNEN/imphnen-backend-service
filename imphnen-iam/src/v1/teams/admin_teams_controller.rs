use crate::{AppState, MetaRequestDto};
use crate::{
    MessageResponseDto, ResponseListSuccessDto, ResponseSuccessDto,
    TeamsCreateRequestDto, TeamsUpdateRequestDto, TeamInviteRequestDto,
    TeamMemberDto, AdminTeamsListItemDto, AdminTeamsDetailItemDto, PermissionsEnum
};
use axum::response::Response;
use axum::extract::Path;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use super::teams_service::{TeamsServiceTrait, TeamsService};
use axum::Router;

/// Helper function for admin endpoints requiring specific permissions
async fn with_admin_perms<F, Fut>(
    headers: HeaderMap,
    state: Extension<AppState>,
    f: F,
) -> Response
where
    F: FnOnce(crate::Claims, AppState) -> Fut,
    Fut: std::future::Future<Output = Response> + Send,
{
    match crate::permissions_guard(headers, state, vec![PermissionsEnum::ManageAllTeams]).await {
        Ok((claims, state)) => f(claims, state).await,
        Err(response) => response,
    }
}

#[utoipa::path(
    get,
    security(
        ("Bearer" = [])
    ),
    path = "/v1/admin/teams",
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
        (status = 200, description = "Get all teams (admin)", body = ResponseListSuccessDto<Vec<AdminTeamsListItemDto>>)
    ),
    tag = "Admin - Teams"
)]
pub async fn get_all_teams(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
    axum::extract::Query(meta): axum::extract::Query<MetaRequestDto>,
) -> Response {
    with_admin_perms(headers, Extension(state), move |_claims, state| {
        TeamsService::get_admin_team_list(&state, meta)
    }).await
}

#[utoipa::path(
    get,
    security(
        ("Bearer" = [])
    ),
    path = "/v1/admin/teams/{id}",
    params(
        ("id" = String, Path, description = "Team ID")
    ),
    responses(
        (status = 200, description = "Get team by ID (admin)", body = ResponseSuccessDto<AdminTeamsDetailItemDto>)
    ),
    tag = "Admin - Teams"
)]
pub async fn get_team_by_id(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
    Path(id): Path<String>,
) -> Response {
    with_admin_perms(headers, Extension(state), move |_claims, state| {
        TeamsService::get_admin_team_by_id(&state, id)
    }).await
}

#[utoipa::path(
    get,
    security(
        ("Bearer" = [])
    ),
    path = "/v1/admin/teams/{id}/members",
    params(
        ("id" = String, Path, description = "Team ID")
    ),
    responses(
        (status = 200, description = "Get team members (admin)", body = ResponseSuccessDto<Vec<TeamMemberDto>>)
    ),
    tag = "Admin - Teams"
)]
pub async fn get_team_members(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
    Path(id): Path<String>,
) -> Response {
    with_admin_perms(headers, Extension(state), move |_claims, state| {
        TeamsService::get_admin_team_members(&state, id)
    }).await
}

#[utoipa::path(
    post,
    security(
        ("Bearer" = [])
    ),
    path = "/v1/admin/teams",
    request_body = TeamsCreateRequestDto,
    responses(
        (status = 200, description = "Create team (admin)", body = ResponseSuccessDto<serde_json::Value>)
    ),
    tag = "Admin - Teams"
)]
pub async fn create_team(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
    Json(payload): Json<TeamsCreateRequestDto>,
) -> impl IntoResponse {
    with_admin_perms(headers, Extension(state), move |claims, state| {
        TeamsService::create_team(&state, claims, payload)
    }).await
}

#[utoipa::path(
    put,
    security(
        ("Bearer" = [])
    ),
    path = "/v1/admin/teams/{id}",
    params(
        ("id" = String, Path, description = "Team ID")
    ),
    request_body = TeamsUpdateRequestDto,
    responses(
        (status = 200, description = "Update team (admin)", body = MessageResponseDto)
    ),
    tag = "Admin - Teams"
)]
pub async fn update_team(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<TeamsUpdateRequestDto>,
) -> impl IntoResponse {
    with_admin_perms(headers, Extension(state), move |claims, state| {
        TeamsService::update_team(&state, claims, id, payload)
    }).await
}

#[utoipa::path(
    delete,
    security(
        ("Bearer" = [])
    ),
    path = "/v1/admin/teams/{id}",
    params(
        ("id" = String, Path, description = "Team ID")
    ),
    responses(
        (status = 200, description = "Delete team (admin)", body = MessageResponseDto)
    ),
    tag = "Admin - Teams"
)]
pub async fn delete_team(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    with_admin_perms(headers, Extension(state), move |claims, state| {
        TeamsService::delete_team(&state, claims, id)
    }).await
}

#[utoipa::path(
    post,
    security(
        ("Bearer" = [])
    ),
    path = "/v1/admin/teams/{id}/invite",
    params(
        ("id" = String, Path, description = "Team ID")
    ),
    request_body = TeamInviteRequestDto,
    responses(
        (status = 200, description = "Invite team members (admin)", body = ResponseSuccessDto<serde_json::Value>)
    ),
    tag = "Admin - Teams"
)]
pub async fn invite_team_members(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
    Path(team_id): Path<String>,
    Json(payload): Json<TeamInviteRequestDto>,
) -> impl IntoResponse {
    with_admin_perms(headers, Extension(state), move |claims, state| {
        TeamsService::invite_team_members(&state, claims, team_id, payload)
    }).await
}

pub fn admin_teams_router() -> Router {
    Router::new()
        .route("/", axum::routing::get(get_all_teams))
        .route("/:id", axum::routing::get(get_team_by_id))
        .route("/:id/members", axum::routing::get(get_team_members))
        .route("/", axum::routing::post(create_team))
        .route("/:id", axum::routing::put(update_team))
        .route("/:id", axum::routing::delete(delete_team))
        .route("/:id/invite", axum::routing::post(invite_team_members))
}