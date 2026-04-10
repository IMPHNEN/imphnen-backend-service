use super::super::dto::{
	HackathonProfileDto, MentorProfileDto, QrProfileDto, SessionProfileDto,
	UsersDetailItemDto, UsersListItemDto, UsersMeResponseDto,
};
use crate::require_permissions;
use crate::users::domain::UserService;
use axum::{
	Extension,
	extract::{Path, Query},
	http::HeaderMap,
	response::IntoResponse,
};
use imphnen_entities::{
	PermissionsEnum, ResponseListSuccessDto, ResponseSuccessDto,
};
use imphnen_libs::AppState;
use imphnen_utils::{ApiPaginated, ApiSuccess, AppError};
use paginator_axum::PaginationQuery;
use paginator_utils::PaginatorResponse;
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use std::collections::HashSet;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct MeQueryParams {
	/// Comma-separated list of modules to include: hackathon,qr,mentor,sessions
	pub include: Option<String>,
}

#[utoipa::path(
    get,
    path = "/v1/iam/users",
    security(("Bearer" = [])),
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
        (status = 200, description = "[ADMIN] Get user list", body = ResponseListSuccessDto<Vec<UsersListItemDto>>)
    ),
    tag = "Users"
)]
pub async fn get_user_list(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Extension(service): Extension<Arc<dyn UserService>>,
	PaginationQuery(params): PaginationQuery,
) -> Result<impl IntoResponse, AppError> {
	require_permissions!(headers, state, [PermissionsEnum::ReadListUsers], {
		let result = service.list(params).await?;
		let mapped = PaginatorResponse {
			data: result
				.data
				.into_iter()
				.map(UsersListItemDto::from)
				.collect::<Vec<_>>(),
			meta: result.meta,
		};
		Ok(ApiPaginated(mapped))
	})
}

#[utoipa::path(
    get,
    path = "/v1/iam/users/detail/{id}",
    security(("Bearer" = [])),
    params(("id" = String, Path, description = "User ID")),
    responses(
        (status = 200, description = "[ADMIN] Get user by ID", body = ResponseSuccessDto<UsersDetailItemDto>)
    ),
    tag = "Users"
)]
pub async fn get_user_by_id(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Extension(service): Extension<Arc<dyn UserService>>,
	Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
	require_permissions!(headers, state, [PermissionsEnum::ReadDetailUsers], {
		Uuid::parse_str(&id).map_err(|_| {
			AppError::BadRequestError("Invalid User ID format".to_string())
		})?;
		let user = service.get(id).await?;
		if user.is_deleted {
			return Err(AppError::NotFoundError("User not found".to_string()));
		}
		Ok(ApiSuccess(UsersDetailItemDto::from(user)))
	})
}

#[utoipa::path(
    get,
    path = "/v1/iam/users/me",
    security(("Bearer" = [])),
    params(
        ("include" = Option<String>, Query, description = "Comma-separated modules to include: hackathon,qr,mentor,sessions. Omit to include all."),
    ),
    responses(
        (status = 200, description = "[USER] Get current user profile (unified across all modules)", body = ResponseSuccessDto<UsersMeResponseDto>)
    ),
    tag = "Users"
)]
pub async fn get_user_me(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Extension(service): Extension<Arc<dyn UserService>>,
	Query(params): Query<MeQueryParams>,
) -> Result<impl IntoResponse, AppError> {
	let (claims, _) = crate::permissions_guard(
		headers,
		axum::extract::Extension(state.clone()),
		vec![],
	)
	.await?;
	let user = service.get_me(claims.user_id.clone()).await?;
	if user.is_deleted {
		return Err(AppError::NotFoundError("User not found".to_string()));
	}

	let user_dto = UsersDetailItemDto::from(user);
	let user_uuid = Uuid::parse_str(&claims.user_id).ok();
	let db = &state.postgres_connection.conn;

	let includes: Option<HashSet<String>> = params.include.map(|s| {
		s.split(',').map(|v| v.trim().to_lowercase()).collect()
	});
	let should_include = |module: &str| -> bool {
		match &includes {
			None => true,
			Some(set) => set.contains(module),
		}
	};

	let hackathon = if should_include("hackathon") {
		fetch_hackathon_profile(db, user_uuid).await
	} else {
		None
	};
	let qr = if should_include("qr") {
		fetch_qr_profile(db, user_uuid).await
	} else {
		None
	};
	let mentor = if should_include("mentor") {
		fetch_mentor_profile(db, user_uuid).await
	} else {
		None
	};
	let sessions = if should_include("sessions") {
		fetch_sessions(db, user_uuid).await
	} else {
		None
	};

	Ok(ApiSuccess(UsersMeResponseDto {
		user: user_dto,
		hackathon,
		qr,
		mentor,
		sessions,
	}))
}

async fn fetch_hackathon_profile(
	db: &DatabaseConnection,
	user_id: Option<Uuid>,
) -> Option<HackathonProfileDto> {
	let uid = user_id?;
	let pool = db.get_postgres_connection_pool();
	sqlx::query_as::<_, HackathonRow>(
		"SELECT COALESCE(is_admin, false) as is_admin, phone_number, location, bio, skills FROM hackathon_users WHERE id = $1",
	)
	.bind(uid)
	.fetch_optional(pool)
	.await
	.ok()?
	.map(|h| HackathonProfileDto {
		is_admin: h.is_admin,
		phone_number: h.phone_number,
		location: h.location,
		bio: h.bio,
		skills: h.skills,
	})
}

async fn fetch_qr_profile(
	db: &DatabaseConnection,
	user_id: Option<Uuid>,
) -> Option<QrProfileDto> {
	let uid = user_id?;
	let pool = db.get_postgres_connection_pool();
	sqlx::query_as::<_, QrRow>("SELECT role, provider FROM qr_users WHERE id = $1")
		.bind(uid)
		.fetch_optional(pool)
		.await
		.ok()?
		.map(|q| QrProfileDto {
			role: q.role,
			provider: q.provider,
		})
}

async fn fetch_mentor_profile(
	db: &DatabaseConnection,
	user_id: Option<Uuid>,
) -> Option<MentorProfileDto> {
	let uid = user_id?;
	let pool = db.get_postgres_connection_pool();
	sqlx::query_as::<_, MentorRow>(
		r#"SELECT id, status, current_company, "current_role", years_of_experience FROM app_mentors WHERE user_id = $1 AND is_deleted = false"#,
	)
	.bind(uid)
	.fetch_optional(pool)
	.await
	.ok()?
	.map(|m| MentorProfileDto {
		mentor_id: m.id.to_string(),
		status: m.status,
		current_company: m.current_company,
		current_role: m.current_role,
		years_of_experience: m.years_of_experience,
	})
}

async fn fetch_sessions(
	db: &DatabaseConnection,
	user_id: Option<Uuid>,
) -> Option<Vec<SessionProfileDto>> {
	let uid = user_id?;
	let pool = db.get_postgres_connection_pool();
	let rows = sqlx::query_as::<_, SessionRow>(
		r#"SELECT id, topic, description, scheduled_at, duration_minutes,
			session_type, status,
			CASE WHEN mentor_id = $1 THEN 'mentor' ELSE 'mentee' END as role
		FROM sessions
		WHERE mentor_id = $1 OR mentee_id = $1
		ORDER BY scheduled_at DESC
		LIMIT 20"#,
	)
	.bind(uid)
	.fetch_all(pool)
	.await
	.ok()?;
	if rows.is_empty() {
		return None;
	}
	Some(
		rows.into_iter()
			.map(|s| SessionProfileDto {
				id: s.id.to_string(),
				topic: s.topic,
				description: s.description,
				scheduled_at: s.scheduled_at.to_rfc3339(),
				duration_minutes: s.duration_minutes,
				session_type: s.session_type,
				status: s.status,
				role: s.role,
			})
			.collect(),
	)
}

#[derive(sqlx::FromRow)]
struct SessionRow {
	id: Uuid,
	topic: String,
	description: Option<String>,
	scheduled_at: chrono::DateTime<chrono::Utc>,
	duration_minutes: i32,
	session_type: String,
	status: String,
	role: String,
}

#[derive(sqlx::FromRow)]
struct HackathonRow {
	is_admin: bool,
	phone_number: Option<String>,
	location: Option<String>,
	bio: Option<String>,
	skills: Option<serde_json::Value>,
}

#[derive(sqlx::FromRow)]
struct QrRow {
	role: String,
	provider: String,
}

#[derive(sqlx::FromRow)]
struct MentorRow {
	id: Uuid,
	status: Option<String>,
	current_company: Option<String>,
	current_role: Option<String>,
	years_of_experience: Option<i32>,
}
