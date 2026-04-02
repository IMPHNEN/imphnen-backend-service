use axum::{Extension, Json, extract::{Path, Query}, response::IntoResponse};
use std::sync::Arc;
use uuid::Uuid;
use imphnen_utils::{errors::AppError, response_format::{ApiSuccess, ApiMessage}};
use crate::middleware::hackathon_auth::HackathonAuthUser;
use crate::teams::domain::service::TeamService;
use super::dto::*;

pub async fn create_team_handler(
    Extension(service): Extension<Arc<dyn TeamService>>,
    Extension(auth): Extension<HackathonAuthUser>,
    Json(body): Json<CreateTeamRequest>,
) -> Result<axum::response::Response, AppError> {
    let team = service.create_team(auth.user_id, body.into()).await?;
    Ok(ApiSuccess(TeamResponse::from(team)).into_response())
}

pub async fn get_team_handler(
    Extension(service): Extension<Arc<dyn TeamService>>,
    Path(team_id): Path<Uuid>,
) -> Result<axum::response::Response, AppError> {
    let team = service.get_team_by_id(team_id).await?;
    Ok(ApiSuccess(TeamResponse::from(team)).into_response())
}

pub async fn browse_teams_handler(
    Extension(service): Extension<Arc<dyn TeamService>>,
    Query(query): Query<BrowseTeamsQuery>,
) -> Result<axum::response::Response, AppError> {
    let result = service.browse_teams(query.into()).await?;
    Ok(ApiSuccess(TeamListResponse {
        data: result.teams.into_iter().map(TeamResponse::from).collect(),
        total: result.total,
        page: result.page,
        per_page: result.per_page,
    }).into_response())
}

pub async fn get_my_teams_handler(
    Extension(service): Extension<Arc<dyn TeamService>>,
    Extension(auth): Extension<HackathonAuthUser>,
) -> Result<axum::response::Response, AppError> {
    let teams = service.get_user_teams(auth.user_id).await?;
    Ok(ApiSuccess(teams.into_iter().map(TeamResponse::from).collect::<Vec<_>>()).into_response())
}

pub async fn update_team_handler(
    Extension(service): Extension<Arc<dyn TeamService>>,
    Extension(auth): Extension<HackathonAuthUser>,
    Path(team_id): Path<Uuid>,
    Json(body): Json<UpdateTeamRequest>,
) -> Result<axum::response::Response, AppError> {
    let team = service.update_team(team_id, auth.user_id, body.into()).await?;
    Ok(ApiSuccess(TeamResponse::from(team)).into_response())
}

pub async fn delete_team_handler(
    Extension(service): Extension<Arc<dyn TeamService>>,
    Extension(auth): Extension<HackathonAuthUser>,
    Path(team_id): Path<Uuid>,
) -> Result<axum::response::Response, AppError> {
    service.delete_team(team_id, auth.user_id).await?;
    Ok(ApiMessage::ok("Team deleted successfully").into_response())
}

pub async fn leave_team_handler(
    Extension(service): Extension<Arc<dyn TeamService>>,
    Extension(auth): Extension<HackathonAuthUser>,
    Path(team_id): Path<Uuid>,
) -> Result<axum::response::Response, AppError> {
    service.leave_team(team_id, auth.user_id).await?;
    Ok(ApiMessage::ok("Left team successfully").into_response())
}

pub async fn remove_member_handler(
    Extension(service): Extension<Arc<dyn TeamService>>,
    Extension(auth): Extension<HackathonAuthUser>,
    Path((team_id, member_id)): Path<(Uuid, Uuid)>,
) -> Result<axum::response::Response, AppError> {
    service.remove_team_member(team_id, auth.user_id, member_id).await?;
    Ok(ApiMessage::ok("Member removed successfully").into_response())
}
