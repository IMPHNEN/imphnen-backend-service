use super::{
	MentorDetailResponseDto, MentorListResponseDto, MentorUpdateRequestDto,
	MentorUserRegisterRequestDto, MentorVerifyRequestDto, MentorsService,
};
use crate::v1::mentors::mentors_dto::MentorRegisterResponseDto;
use ::axum::{
	extract::{Extension, Path, Query},
	http::HeaderMap,
	response::Response,
};
use imphnen_entities::MetaRequestDto;
use imphnen_libs::{AppState, ValidatedJson};
use imphnen_iam::{PermissionsEnum, require_permissions};
use imphnen_utils::extract_email;

#[utoipa::path(
    post,
    path = "/v1/mentors/register",
    request_body = MentorUserRegisterRequestDto,
    responses(
        (status = 200, description = "[PUBLIC] Mentor registered successfully", body = MentorRegisterResponseDto),
        (status = 400, description = "[PUBLIC] Bad request - validation error"),
        (status = 409, description = "[PUBLIC] Conflict - user already has mentor profile"),
        (status = 500, description = "[PUBLIC] Internal server error")
    ),
    tag = "Mentors"
)]
pub async fn post_register_mentor(
	Extension(app_state): Extension<AppState>,
	ValidatedJson(dto): ValidatedJson<MentorUserRegisterRequestDto>,
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
        (status = 200, description = "[ADMIN] Get list of mentors", body = Vec<MentorListResponseDto>),
        (status = 500, description = "[ADMIN] Internal server error")
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
	require_permissions!(headers, app_state, [PermissionsEnum::ReadListMentors], {
		MentorsService::get_mentor_list(&app_state, meta).await
	})
}

#[utoipa::path(
    get,
    path = "/v1/mentors/detail/{id}",
    params(
        ("id" = String, Path, description = "Mentor ID")
    ),
    responses(
        (status = 200, description = "[ADMIN] Get mentor by ID", body = MentorDetailResponseDto),
        (status = 404, description = "[ADMIN] Mentor not found"),
        (status = 500, description = "[ADMIN] Internal server error")
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
	require_permissions!(headers, app_state, [PermissionsEnum::ReadDetailMentors], {
		MentorsService::get_mentor_by_id(&app_state, &id).await
	})
}

#[utoipa::path(
    put,
    path = "/v1/mentors/update/{id}",
    params(
        ("id" = String, Path, description = "Mentor ID")
    ),
    request_body = MentorUpdateRequestDto,
    responses(
        (status = 200, description = "[ADMIN] Mentor updated successfully", body = MentorDetailResponseDto),
        (status = 400, description = "[ADMIN] Bad request - validation error"),
        (status = 404, description = "[ADMIN] Mentor not found"),
        (status = 500, description = "[ADMIN] Internal server error")
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
	ValidatedJson(dto): ValidatedJson<MentorUpdateRequestDto>,
) -> Response {
	require_permissions!(headers, app_state, [PermissionsEnum::UpdateMentors], {
		MentorsService::update_mentor(&app_state, &id, dto).await
	})
}

#[utoipa::path(
    delete,
    path = "/v1/mentors/delete/{id}",
    params(
        ("id" = String, Path, description = "Mentor ID")
    ),
    responses(
        (status = 200, description = "[ADMIN] Mentor deleted successfully"),
        (status = 404, description = "[ADMIN] Mentor not found"),
        (status = 500, description = "[ADMIN] Internal server error")
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
	require_permissions!(headers, app_state, [PermissionsEnum::DeleteMentors], {
		MentorsService::delete_mentor(&app_state, &id).await
	})
}

#[utoipa::path(
    put,
    path = "/v1/mentors/verify/{id}",
    params(
        ("id" = String, Path, description = "Mentor ID")
    ),
    request_body = MentorVerifyRequestDto,
    responses(
        (status = 200, description = "[ADMIN] Mentor verified successfully", body = MentorDetailResponseDto),
        (status = 400, description = "[ADMIN] Bad request - validation error"),
        (status = 404, description = "[ADMIN] Mentor not found"),
        (status = 500, description = "[ADMIN] Internal server error")
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
	ValidatedJson(dto): ValidatedJson<MentorVerifyRequestDto>,
) -> Response {
	require_permissions!(headers, app_state, [PermissionsEnum::VerifyMentors], {
		MentorsService::verify_mentor(&app_state, &id, dto).await
	})
}

#[utoipa::path(
    get,
    path = "/v1/mentors/me",
    responses(
        (status = 200, description = "[MENTOR] Current user's mentor profile", body = MentorDetailResponseDto),
        (status = 401, description = "[MENTOR] Unauthorized - invalid token"),
        (status = 403, description = "[MENTOR] Mentor profile not found for current user"),
        (status = 500, description = "[MENTOR] Internal server error")
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
	require_permissions!(headers.clone(), app_state, [PermissionsEnum::ReadOwnMentorProfile], {
		let email = match extract_email(&headers) {
			Some(email) => email,
			None => {
				return imphnen_utils::common_response(
					axum::http::StatusCode::UNAUTHORIZED,
					"Token tidak valid",
				);
			}
		};
		MentorsService::get_mentor_me(&app_state, &email).await
	})
}

#[utoipa::path(
    put,
    path = "/v1/mentors/update/me",
    request_body = MentorUpdateRequestDto,
    responses(
        (status = 200, description = "[MENTOR] Mentor profile updated successfully", body = MentorDetailResponseDto),
        (status = 400, description = "[MENTOR] Bad request - validation error"),
        (status = 401, description = "[MENTOR] Unauthorized - invalid token"),
        (status = 404, description = "[MENTOR] Mentor profile not found"),
        (status = 500, description = "[MENTOR] Internal server error")
    ),
    tag = "Mentors",
    security(
        ("Bearer" = [])
    )
)]
pub async fn put_update_mentor_me(
	headers: HeaderMap,
	Extension(app_state): Extension<AppState>,
	ValidatedJson(dto): ValidatedJson<MentorUpdateRequestDto>,
) -> Response {
	require_permissions!(headers.clone(), app_state, [PermissionsEnum::UpdateOwnMentorProfile], {
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
	})
}
#[utoipa::path(
    put,
    path = "/v1/mentors/update",
    request_body = MentorUpdateRequestDto,
    responses(
        (status = 400, description = "[PUBLIC] Bad request - Mentor ID is required for update"),
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
        (status = 200, description = "[MENTOR] Mentor application status", body = String),
        (status = 401, description = "[MENTOR] Unauthorized - invalid token"),
        (status = 403, description = "[MENTOR] No mentor application found for current user"),
        (status = 500, description = "[MENTOR] Internal server error")
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
	require_permissions!(headers.clone(), app_state, [PermissionsEnum::ReadOwnMentorStatus], {
		let email = match extract_email(&headers) {
			Some(email) => email,
			None => {
				return imphnen_utils::common_response(
					axum::http::StatusCode::UNAUTHORIZED,
					"Token tidak valid",
				);
			}
		};
		MentorsService::get_mentor_status(&app_state, &email).await
	})
}
