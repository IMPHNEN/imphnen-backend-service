use super::dto::*;
use crate::middleware::hackathon_auth::HackathonAuthUser;
use crate::teams::domain::service::TeamService;
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

#[utoipa::path(
    post,
    path = "/v1/hackathon/teams",
    request_body = CreateTeamRequest,
    responses(
        (status = 200, description = "Create a new team",
         body = inline(TeamResponse),
         example = json!({
             "data": {
                 "id": "7c3a1d2e-8f4b-4c5a-9d6e-1f2a3b4c5d6e",
                 "name": "Rust Enjoyers",
                 "description": "A team that loves Rust",
                 "city": "Jakarta",
                 "visibility": "public",
                 "logo": null,
                 "banner": null,
                 "leader_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
                 "leader": {
                     "id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
                     "email": "leader@example.com",
                     "fullname": "Budi Santoso",
                     "avatar": null,
                     "phone_number": null,
                     "location": "Jakarta",
                     "bio": null,
                     "skills": ["Rust"],
                     "is_active": true
                 },
                 "members": [],
                 "member_count": 1,
                 "has_submission": false,
                 "created_at": "2025-01-01T00:00:00Z",
                 "updated_at": "2025-01-01T00:00:00Z"
             },
             "version": "0.3.0"
         })),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Hackathon - Teams",
    security(("bearer_auth" = []))
)]
pub async fn create_team_handler(
	Extension(service): Extension<Arc<dyn TeamService>>,
	Extension(auth): Extension<HackathonAuthUser>,
	Json(body): Json<CreateTeamRequest>,
) -> Result<axum::response::Response, AppError> {
	let team = service.create_team(auth.user_id, body.into()).await?;
	Ok(ApiSuccess(TeamResponse::from(team)).into_response())
}

#[utoipa::path(
    get,
    path = "/v1/hackathon/teams/{team_id}",
    params(("team_id" = Uuid, Path, description = "Team ID")),
    responses(
        (status = 200, description = "Get team by ID",
         body = inline(TeamResponse),
         example = json!({
             "data": {
                 "id": "7c3a1d2e-8f4b-4c5a-9d6e-1f2a3b4c5d6e",
                 "name": "Rust Enjoyers",
                 "description": "A team that loves Rust",
                 "city": "Jakarta",
                 "visibility": "public",
                 "logo": null,
                 "banner": null,
                 "leader_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
                 "leader": {
                     "id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
                     "email": "leader@example.com",
                     "fullname": "Budi Santoso",
                     "avatar": null,
                     "phone_number": null,
                     "location": "Jakarta",
                     "bio": null,
                     "skills": ["Rust"],
                     "is_active": true
                 },
                 "members": [
                     {
                         "id": "1a2b3c4d-5e6f-7a8b-9c0d-1e2f3a4b5c6d",
                         "team_id": "7c3a1d2e-8f4b-4c5a-9d6e-1f2a3b4c5d6e",
                         "user_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
                         "user": {
                             "id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
                             "email": "leader@example.com",
                             "fullname": "Budi Santoso",
                             "avatar": null,
                             "phone_number": null,
                             "location": "Jakarta",
                             "bio": null,
                             "skills": ["Rust"],
                             "is_active": true
                         },
                         "role": "leader",
                         "status": "active",
                         "joined_at": "2025-01-01T00:00:00Z"
                     }
                 ],
                 "member_count": 1,
                 "has_submission": false,
                 "created_at": "2025-01-01T00:00:00Z",
                 "updated_at": "2025-01-01T00:00:00Z"
             },
             "version": "0.3.0"
         })),
        (status = 404, description = "Team not found")
    ),
    tag = "Hackathon - Teams"
)]
pub async fn get_team_handler(
	Extension(service): Extension<Arc<dyn TeamService>>,
	Path(team_id): Path<Uuid>,
) -> Result<axum::response::Response, AppError> {
	let team = service.get_team_by_id(team_id).await?;
	Ok(ApiSuccess(TeamResponse::from(team)).into_response())
}

#[utoipa::path(
    get,
    path = "/v1/hackathon/teams/browse",
    params(BrowseTeamsQuery),
    responses(
        (status = 200, description = "Browse teams with filters",
         body = inline(TeamListResponse),
         example = json!({
             "data": {
                 "data": [
                     {
                         "id": "7c3a1d2e-8f4b-4c5a-9d6e-1f2a3b4c5d6e",
                         "name": "Rust Enjoyers",
                         "description": "A team that loves Rust",
                         "city": "Jakarta",
                         "visibility": "public",
                         "logo": null,
                         "banner": null,
                         "leader_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
                         "leader": null,
                         "members": null,
                         "member_count": 3,
                         "has_submission": false,
                         "created_at": "2025-01-01T00:00:00Z",
                         "updated_at": "2025-01-01T00:00:00Z"
                     }
                 ],
                 "total": 1,
                 "page": 1,
                 "per_page": 10
             },
             "version": "0.3.0"
         }))
    ),
    tag = "Hackathon - Teams"
)]
pub async fn browse_teams_handler(
	Extension(service): Extension<Arc<dyn TeamService>>,
	Query(query): Query<BrowseTeamsQuery>,
) -> Result<axum::response::Response, AppError> {
	let result = service.browse_teams(query.into()).await?;
	Ok(
		ApiSuccess(TeamListResponse {
			data: result.teams.into_iter().map(TeamResponse::from).collect(),
			total: result.total,
			page: result.page,
			per_page: result.per_page,
		})
		.into_response(),
	)
}

#[utoipa::path(
    get,
    path = "/v1/hackathon/teams/my",
    responses(
        (status = 200, description = "Get my teams",
         example = json!({
             "data": [
                 {
                     "id": "7c3a1d2e-8f4b-4c5a-9d6e-1f2a3b4c5d6e",
                     "name": "Rust Enjoyers",
                     "description": "A team that loves Rust",
                     "city": "Jakarta",
                     "visibility": "public",
                     "logo": null,
                     "banner": null,
                     "leader_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
                     "leader": null,
                     "members": null,
                     "member_count": 3,
                     "has_submission": false,
                     "created_at": "2025-01-01T00:00:00Z",
                     "updated_at": "2025-01-01T00:00:00Z"
                 }
             ],
             "version": "0.3.0"
         })),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Hackathon - Teams",
    security(("bearer_auth" = []))
)]
pub async fn get_my_teams_handler(
	Extension(service): Extension<Arc<dyn TeamService>>,
	Extension(auth): Extension<HackathonAuthUser>,
) -> Result<axum::response::Response, AppError> {
	let teams = service.get_user_teams(auth.user_id).await?;
	Ok(
		ApiSuccess(
			teams
				.into_iter()
				.map(TeamResponse::from)
				.collect::<Vec<_>>(),
		)
		.into_response(),
	)
}

#[utoipa::path(
    put,
    path = "/v1/hackathon/teams/{team_id}",
    params(("team_id" = Uuid, Path, description = "Team ID")),
    request_body = UpdateTeamRequest,
    responses(
        (status = 200, description = "Update team",
         body = inline(TeamResponse),
         example = json!({
             "data": {
                 "id": "7c3a1d2e-8f4b-4c5a-9d6e-1f2a3b4c5d6e",
                 "name": "Rust Enjoyers Updated",
                 "description": "We love Rust and systems programming",
                 "city": "Bandung",
                 "visibility": "public",
                 "logo": "https://cdn.example.com/team-logo.png",
                 "banner": null,
                 "leader_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
                 "leader": null,
                 "members": null,
                 "member_count": 3,
                 "has_submission": false,
                 "created_at": "2025-01-01T00:00:00Z",
                 "updated_at": "2025-01-15T00:00:00Z"
             },
             "version": "0.3.0"
         })),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - not team leader")
    ),
    tag = "Hackathon - Teams",
    security(("bearer_auth" = []))
)]
pub async fn update_team_handler(
	Extension(service): Extension<Arc<dyn TeamService>>,
	Extension(auth): Extension<HackathonAuthUser>,
	Path(team_id): Path<Uuid>,
	Json(body): Json<UpdateTeamRequest>,
) -> Result<axum::response::Response, AppError> {
	let team = service
		.update_team(team_id, auth.user_id, body.into())
		.await?;
	Ok(ApiSuccess(TeamResponse::from(team)).into_response())
}

#[utoipa::path(
    delete,
    path = "/v1/hackathon/teams/{team_id}",
    params(("team_id" = Uuid, Path, description = "Team ID")),
    responses(
        (status = 200, description = "Delete team",
         example = json!({"message": "Team deleted successfully", "version": "0.3.0"})),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - not team leader")
    ),
    tag = "Hackathon - Teams",
    security(("bearer_auth" = []))
)]
pub async fn delete_team_handler(
	Extension(service): Extension<Arc<dyn TeamService>>,
	Extension(auth): Extension<HackathonAuthUser>,
	Path(team_id): Path<Uuid>,
) -> Result<axum::response::Response, AppError> {
	service.delete_team(team_id, auth.user_id).await?;
	Ok(ApiMessage::ok("Team deleted successfully").into_response())
}

#[utoipa::path(
    post,
    path = "/v1/hackathon/teams/{team_id}/leave",
    params(("team_id" = Uuid, Path, description = "Team ID")),
    responses(
        (status = 200, description = "Leave team",
         example = json!({"message": "Left team successfully", "version": "0.3.0"})),
        (status = 401, description = "Unauthorized")
    ),
    tag = "Hackathon - Teams",
    security(("bearer_auth" = []))
)]
pub async fn leave_team_handler(
	Extension(service): Extension<Arc<dyn TeamService>>,
	Extension(auth): Extension<HackathonAuthUser>,
	Path(team_id): Path<Uuid>,
) -> Result<axum::response::Response, AppError> {
	service.leave_team(team_id, auth.user_id).await?;
	Ok(ApiMessage::ok("Left team successfully").into_response())
}

#[utoipa::path(
    delete,
    path = "/v1/hackathon/teams/{team_id}/members/{member_id}",
    params(
        ("team_id" = Uuid, Path, description = "Team ID"),
        ("member_id" = Uuid, Path, description = "Member user ID")
    ),
    responses(
        (status = 200, description = "Remove team member",
         example = json!({"message": "Member removed successfully", "version": "0.3.0"})),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - not team leader")
    ),
    tag = "Hackathon - Teams",
    security(("bearer_auth" = []))
)]
pub async fn remove_member_handler(
	Extension(service): Extension<Arc<dyn TeamService>>,
	Extension(auth): Extension<HackathonAuthUser>,
	Path((team_id, member_id)): Path<(Uuid, Uuid)>,
) -> Result<axum::response::Response, AppError> {
	service
		.remove_team_member(team_id, auth.user_id, member_id)
		.await?;
	Ok(ApiMessage::ok("Member removed successfully").into_response())
}
