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

#[utoipa::path(
    get,
    path = "/v1/hackathon/admin/users",
    params(PageQuery),
    responses(
        (status = 200, description = "Admin: list all users",
         example = json!({
             "data": {
                 "data": [
                     {
                         "id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
                         "email": "user@example.com",
                         "fullname": "Budi Santoso",
                         "avatar": null,
                         "is_active": true,
                         "is_admin": false,
                         "created_at": "2025-01-01T00:00:00Z"
                     }
                 ],
                 "total": 42,
                 "page": 1,
                 "limit": 20
             },
             "version": "0.3.0"
         })),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - admin only")
    ),
    tag = "Hackathon - Admin",
    security(("bearer_auth" = []))
)]
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

#[utoipa::path(
    get,
    path = "/v1/hackathon/admin/users/{user_id}",
    params(("user_id" = Uuid, Path, description = "User ID")),
    responses(
        (status = 200, description = "Admin: get user by ID",
         example = json!({
             "data": {
                 "id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
                 "email": "user@example.com",
                 "fullname": "Budi Santoso",
                 "avatar": null,
                 "is_active": true,
                 "is_admin": false,
                 "created_at": "2025-01-01T00:00:00Z"
             },
             "version": "0.3.0"
         })),
        (status = 403, description = "Forbidden - admin only"),
        (status = 404, description = "User not found")
    ),
    tag = "Hackathon - Admin",
    security(("bearer_auth" = []))
)]
pub async fn admin_get_user(
	Extension(service): Extension<Arc<dyn AdminService>>,
	Path(user_id): Path<Uuid>,
) -> Result<axum::response::Response, AppError> {
	let user = service.get_user(user_id).await?;
	Ok(ApiSuccess(user).into_response())
}

#[utoipa::path(
    post,
    path = "/v1/hackathon/admin/users/{user_id}/set-admin",
    params(("user_id" = Uuid, Path, description = "User ID")),
    request_body = SetAdminRequest,
    responses(
        (status = 200, description = "Admin: set user admin status",
         example = json!({"message": "User admin status updated", "version": "0.3.0"})),
        (status = 403, description = "Forbidden - admin only")
    ),
    tag = "Hackathon - Admin",
    security(("bearer_auth" = []))
)]
pub async fn admin_set_admin(
	Extension(service): Extension<Arc<dyn AdminService>>,
	Path(user_id): Path<Uuid>,
	Json(body): Json<SetAdminRequest>,
) -> Result<ApiMessage, AppError> {
	service.set_admin(user_id, body.is_admin).await?;
	Ok(ApiMessage::ok("User admin status updated"))
}

#[utoipa::path(
    delete,
    path = "/v1/hackathon/admin/users/{user_id}",
    params(("user_id" = Uuid, Path, description = "User ID")),
    responses(
        (status = 200, description = "Admin: delete user",
         example = json!({"message": "User deleted", "version": "0.3.0"})),
        (status = 403, description = "Forbidden - admin only")
    ),
    tag = "Hackathon - Admin",
    security(("bearer_auth" = []))
)]
pub async fn admin_delete_user(
	Extension(service): Extension<Arc<dyn AdminService>>,
	Path(user_id): Path<Uuid>,
) -> Result<ApiMessage, AppError> {
	service.delete_user(user_id).await?;
	Ok(ApiMessage::ok("User deleted"))
}

#[utoipa::path(
    get,
    path = "/v1/hackathon/admin/teams",
    params(PageQuery),
    responses(
        (status = 200, description = "Admin: list all teams",
         example = json!({
             "data": {
                 "data": [
                     {
                         "id": "7c3a1d2e-8f4b-4c5a-9d6e-1f2a3b4c5d6e",
                         "name": "Rust Enjoyers",
                         "city": "Jakarta",
                         "visibility": "public",
                         "leader_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
                         "created_at": "2025-01-01T00:00:00Z"
                     }
                 ],
                 "total": 15,
                 "page": 1,
                 "limit": 20
             },
             "version": "0.3.0"
         })),
        (status = 403, description = "Forbidden - admin only")
    ),
    tag = "Hackathon - Admin",
    security(("bearer_auth" = []))
)]
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

#[utoipa::path(
    delete,
    path = "/v1/hackathon/admin/teams/{team_id}",
    params(("team_id" = Uuid, Path, description = "Team ID")),
    responses(
        (status = 200, description = "Admin: delete team",
         example = json!({"message": "Team deleted", "version": "0.3.0"})),
        (status = 403, description = "Forbidden - admin only")
    ),
    tag = "Hackathon - Admin",
    security(("bearer_auth" = []))
)]
pub async fn admin_delete_team(
	Extension(service): Extension<Arc<dyn AdminService>>,
	Path(team_id): Path<Uuid>,
) -> Result<ApiMessage, AppError> {
	service.delete_team(team_id).await?;
	Ok(ApiMessage::ok("Team deleted"))
}

#[utoipa::path(
    get,
    path = "/v1/hackathon/admin/submissions",
    params(PageQuery),
    responses(
        (status = 200, description = "Admin: list all submissions",
         example = json!({
             "data": {
                 "data": [
                     {
                         "id": "c3d4e5f6-a7b8-9012-cdef-123456789012",
                         "team_id": "7c3a1d2e-8f4b-4c5a-9d6e-1f2a3b4c5d6e",
                         "project_name": "EcoTrack - Sustainability Monitor",
                         "status": "submitted",
                         "submitted_at": "2025-01-20T12:00:00Z",
                         "created_at": "2025-01-05T00:00:00Z"
                     }
                 ],
                 "total": 8,
                 "page": 1,
                 "limit": 20
             },
             "version": "0.3.0"
         })),
        (status = 403, description = "Forbidden - admin only")
    ),
    tag = "Hackathon - Admin",
    security(("bearer_auth" = []))
)]
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

#[utoipa::path(
    post,
    path = "/v1/hackathon/admin/winners",
    request_body = SetWinnerRequest,
    responses(
        (status = 200, description = "Admin: set winner",
         example = json!({"message": "Winner set", "version": "0.3.0"})),
        (status = 403, description = "Forbidden - admin only")
    ),
    tag = "Hackathon - Admin",
    security(("bearer_auth" = []))
)]
pub async fn admin_set_winner(
	Extension(service): Extension<Arc<dyn AdminService>>,
	Json(body): Json<SetWinnerRequest>,
) -> Result<ApiMessage, AppError> {
	service
		.set_winner(body.team_id, body.rank, body.prize)
		.await?;
	Ok(ApiMessage::ok("Winner set"))
}

#[utoipa::path(
    delete,
    path = "/v1/hackathon/admin/winners/{team_id}",
    params(("team_id" = Uuid, Path, description = "Team ID")),
    responses(
        (status = 200, description = "Admin: remove winner",
         example = json!({"message": "Winner removed", "version": "0.3.0"})),
        (status = 403, description = "Forbidden - admin only")
    ),
    tag = "Hackathon - Admin",
    security(("bearer_auth" = []))
)]
pub async fn admin_remove_winner(
	Extension(service): Extension<Arc<dyn AdminService>>,
	Path(team_id): Path<Uuid>,
) -> Result<ApiMessage, AppError> {
	service.remove_winner(team_id).await?;
	Ok(ApiMessage::ok("Winner removed"))
}

#[utoipa::path(
    get,
    path = "/v1/hackathon/admin/winners",
    responses(
        (status = 200, description = "Admin: list winners",
         example = json!({
             "data": [
                 {
                     "id": "d4e5f6a7-b8c9-0123-defa-234567890123",
                     "team_id": "7c3a1d2e-8f4b-4c5a-9d6e-1f2a3b4c5d6e",
                     "rank": 1,
                     "prize": "Rp 10.000.000",
                     "created_at": "2025-02-01T10:00:00Z"
                 }
             ],
             "version": "0.3.0"
         })),
        (status = 403, description = "Forbidden - admin only")
    ),
    tag = "Hackathon - Admin",
    security(("bearer_auth" = []))
)]
pub async fn admin_list_winners(
	Extension(service): Extension<Arc<dyn AdminService>>,
) -> Result<axum::response::Response, AppError> {
	let rows = service.list_winners().await?;
	Ok(ApiSuccess(rows).into_response())
}
