use super::{
	MentorDetailResponseDto, MentorListResponseDto, MentorUpdateRequestDto,
	MentorUserRegisterRequestDto, MentorVerifyRequestDto, MentorsService,
};
use crate::v1::mentors::mentors_dto::MentorRegisterResponseDto;
use ::axum::{
	extract::{Extension, Json, Path, Query},
	http::HeaderMap,
	response::{IntoResponse, Response},
};
use imphnen_entities::*;
use imphnen_iam::{PermissionsEnum, permissions_guard};
use imphnen_utils::extract_email;
use serde_json::json;

#[utoipa::path(
    post,
    path = "/v1/mentors/register",
    request_body = MentorUserRegisterRequestDto,
    responses(
        (status = 200, description = "Mentor registered successfully", body = MentorRegisterResponseDto),
        (status = 400, description = "Bad request - validation error"),
        (status = 409, description = "Conflict - user already has mentor profile"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Mentors"
)]
pub async fn post_register_mentor(
	Extension(app_state): Extension<AppState>,
	Json(dto): Json<MentorUserRegisterRequestDto>,
) -> Response {
	MentorsService::register_mentor(&app_state, dto).await
}

#[utoipa::path(
    get,
    path = "/v1/mentors",
    params(
        ("page" = Option<u64>, Query, description = "Page number"),
        ("per_page" = Option<u64>, Query, description = "Items per page"),
        ("search" = Option<String>, Query, description = "Search query"),
        ("sort_by" = Option<String>, Query, description = "Sort by field"),
        ("order" = Option<String>, Query, description = "Sort order (ASC/DESC)"),
    ),
    responses(
        (status = 200, description = "Get list of mentors", body = Vec<MentorListResponseDto>),
        (status = 500, description = "Internal server error")
    ),
    tag = "Mentors",
    security(
        ("Bearer" = [])
    )
)]
pub async fn get_mentor_list(
	headers: HeaderMap,
	Extension(app_state): Extension<AppState>,
	Query(meta): Query<MetaRequestDto>,
) -> Response {
	match permissions_guard(
		headers,
		Extension(app_state),
		vec![PermissionsEnum::ReadListMentors],
	)
	.await
	{
		Ok((_user, app_state)) => MentorsService::get_mentor_list(&app_state, meta).await,
		Err(response) => response,
	}
}

#[utoipa::path(
    get,
    path = "/v1/mentors/detail/{id}",
    params(
        ("id" = String, Path, description = "Mentor ID")
    ),
    responses(
        (status = 200, description = "Get mentor by ID", body = MentorDetailResponseDto),
        (status = 404, description = "Mentor not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Mentors",
    security(
        ("Bearer" = [])
    )
)]
pub async fn get_mentor_by_id(
	headers: HeaderMap,
	Extension(app_state): Extension<AppState>,
	Path(id): Path<String>,
) -> Response {
	match permissions_guard(
		headers,
		Extension(app_state),
		vec![PermissionsEnum::ReadDetailMentors],
	)
	.await
	{
		Ok((_user, app_state)) => MentorsService::get_mentor_by_id(&app_state, &id).await,
		Err(response) => response,
	}
}

#[utoipa::path(
    put,
    path = "/v1/mentors/update/{id}",
    params(
        ("id" = String, Path, description = "Mentor ID")
    ),
    request_body = MentorUpdateRequestDto,
    responses(
        (status = 200, description = "Mentor updated successfully", body = MentorDetailResponseDto),
        (status = 400, description = "Bad request - validation error"),
        (status = 404, description = "Mentor not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Mentors - Admin",
    security(
        ("Bearer" = [])
    )
)]
pub async fn put_update_mentor(
	headers: HeaderMap,
	Extension(app_state): Extension<AppState>,
	Path(id): Path<String>,
	Json(dto): Json<MentorUpdateRequestDto>,
) -> Response {
	match permissions_guard(
		headers,
		Extension(app_state),
		vec![PermissionsEnum::UpdateMentors],
	)
	.await
	{
		Ok((_user, app_state)) => MentorsService::update_mentor(&app_state, &id, dto).await,
		Err(response) => response,
	}
}

#[utoipa::path(
    delete,
    path = "/v1/mentors/delete/{id}",
    params(
        ("id" = String, Path, description = "Mentor ID")
    ),
    responses(
        (status = 200, description = "Mentor deleted successfully"),
        (status = 404, description = "Mentor not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Mentors - Admin",
    security(
        ("Bearer" = [])
    )
)]
pub async fn delete_mentor(
	headers: HeaderMap,
	Extension(app_state): Extension<AppState>,
	Path(id): Path<String>,
) -> Response {
	match permissions_guard(
		headers,
		Extension(app_state),
		vec![PermissionsEnum::DeleteMentors],
	)
	.await
	{
		Ok((_user, app_state)) => MentorsService::delete_mentor(&app_state, &id).await,
		Err(response) => response,
	}
}

#[utoipa::path(
    put,
    path = "/v1/mentors/verify/{id}",
    params(
        ("id" = String, Path, description = "Mentor ID")
    ),
    request_body = MentorVerifyRequestDto,
    responses(
        (status = 200, description = "Mentor verified successfully", body = MentorDetailResponseDto),
        (status = 400, description = "Bad request - validation error"),
        (status = 404, description = "Mentor not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Mentors - Admin",
    security(
        ("Bearer" = [])
    )
)]
pub async fn put_verify_mentor(
	headers: HeaderMap,
	Extension(app_state): Extension<AppState>,
	Path(id): Path<String>,
	Json(dto): Json<MentorVerifyRequestDto>,
) -> Response {
	match permissions_guard(
		headers,
		Extension(app_state),
		vec![PermissionsEnum::VerifyMentors],
	)
	.await
	{
		Ok((_user, app_state)) => MentorsService::verify_mentor(&app_state, &id, dto).await,
		Err(response) => response,
	}
}

#[utoipa::path(
    get,
    path = "/v1/mentors/me",
    responses(
        (status = 200, description = "Current user's mentor profile", body = MentorDetailResponseDto),
        (status = 401, description = "Unauthorized - invalid token"),
        (status = 403, description = "Mentor profile not found for current user"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Mentors",
    security(
        ("Bearer" = [])
    )
)]
pub async fn get_mentor_me(
	headers: HeaderMap,
	Extension(app_state): Extension<AppState>,
) -> Response {
	match permissions_guard(
		headers.clone(),
		Extension(app_state),
		vec![PermissionsEnum::ReadOwnMentorProfile],
	)
	.await
	{
		Ok((_user, app_state)) => {
			let email = match extract_email(&headers) {
				Some(email) => email,
				None => {
					return (
						axum::http::StatusCode::UNAUTHORIZED,
						Json(json!({
								"error": "Unauthorized",
								"message": "Token tidak valid"
						})),
					)
						.into_response();
				}
			};
			MentorsService::get_mentor_me(&app_state, &email).await
		}
		Err(response) => response,
	}
}

#[utoipa::path(
    put,
    path = "/v1/mentors/update/me",
    request_body = MentorUpdateRequestDto,
    responses(
        (status = 200, description = "Mentor profile updated successfully", body = MentorDetailResponseDto),
        (status = 400, description = "Bad request - validation error"),
        (status = 401, description = "Unauthorized - invalid token"),
        (status = 404, description = "Mentor profile not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Mentors",
    security(
        ("Bearer" = [])
    )
)]
pub async fn put_update_mentor_me(
	headers: HeaderMap,
	Extension(app_state): Extension<AppState>,
	Json(dto): Json<MentorUpdateRequestDto>,
) -> Response {
	match permissions_guard(
		headers.clone(),
		Extension(app_state),
		vec![PermissionsEnum::UpdateOwnMentorProfile],
	)
	.await
	{
		Ok((_user, app_state)) => {
			let email = match extract_email(&headers) {
				Some(email) => email,
				None => {
					return imphnen_utils::common_response(
						axum::http::StatusCode::UNAUTHORIZED,
						"Token tidak valid",
					);
				}
			};
			MentorsService::update_mentor_me(&app_state, &email, dto).await
		}
		Err(response) => response,
	}
}
#[utoipa::path(
    put,
    path = "/v1/mentors/update",
    request_body = MentorUpdateRequestDto,
    responses(
        (status = 400, description = "Bad request - Mentor ID is required for update"),
    ),
    tag = "Mentors - Admin"
)]
pub async fn put_update_mentor_no_id() -> Response {
    imphnen_utils::common_response(
        axum::http::StatusCode::BAD_REQUEST,
        "Mentor ID is required for update",
    )
}

#[utoipa::path(
    get,
    path = "/v1/mentors/status",
    responses(
        (status = 200, description = "Mentor application status", body = String),
        (status = 401, description = "Unauthorized - invalid token"),
        (status = 403, description = "No mentor application found for current user"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Mentors",
    security(
        ("Bearer" = [])
    )
)]
pub async fn get_mentor_status(
	headers: HeaderMap,
	Extension(app_state): Extension<AppState>,
) -> Response {
	match permissions_guard(
		headers.clone(),
		Extension(app_state),
		vec![PermissionsEnum::ReadOwnMentorStatus],
	)
	.await
	{
		Ok((_user, app_state)) => {
			let email = match extract_email(&headers) {
				Some(email) => email,
				None => {
					return (
						axum::http::StatusCode::UNAUTHORIZED,
						Json(json!({
								"error": "Unauthorized",
								"message": "Token tidak valid"
						})),
					)
						.into_response();
				}
			};
			MentorsService::get_mentor_status(&app_state, &email).await
		}
		Err(response) => response,
	}
}
