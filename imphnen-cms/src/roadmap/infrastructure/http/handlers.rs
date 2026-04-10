use super::dto::{
	RoadmapCreateRequestDto, RoadmapDetailItemDto, RoadmapListItemDto,
	RoadmapUpdateRequestDto,
};
use crate::roadmap::domain::RoadmapService;
use axum::{
	Extension,
	extract::Path,
	http::HeaderMap,
	response::{IntoResponse, Response},
};
use imphnen_entities::ResponseSuccessDto;
use imphnen_iam::{PermissionsEnum, require_permissions};
use imphnen_libs::{AppState, ValidatedJson};
use imphnen_utils::AppError;
use imphnen_utils::{ApiMessage, ApiPaginated, ApiSuccess};
use paginator_axum::PaginationQuery;
use paginator_utils::PaginatorResponse;
use std::sync::Arc;
use uuid::Uuid;

#[utoipa::path(
    get,
    path = "/v1/landing/cms/roadmap",
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("per_page" = Option<i64>, Query, description = "Items per page"),
        ("search" = Option<String>, Query, description = "Search keyword"),
        ("sort_by" = Option<String>, Query, description = "Sort by field"),
        ("order" = Option<String>, Query, description = "Order ASC or DESC"),
    ),
    responses(
        (status = 200, description = "[PUBLIC] Get roadmap list")
    ),
    tag = "Roadmap"
)]
pub async fn get_roadmap_list(
	Extension(service): Extension<Arc<dyn RoadmapService>>,
	PaginationQuery(params): PaginationQuery,
) -> Response {
	match service.list(params).await {
		Ok(result) => {
			let mapped = PaginatorResponse {
				data: result
					.data
					.into_iter()
					.map(RoadmapListItemDto::from)
					.collect::<Vec<_>>(),
				meta: result.meta,
			};
			ApiPaginated(mapped).into_response()
		}
		Err(e) => ApiMessage::new(axum::http::StatusCode::BAD_REQUEST, e.to_string())
			.into_response(),
	}
}

#[utoipa::path(
    get,
    path = "/v1/landing/cms/roadmap/detail/{id}",
    params(
        ("id" = String, Path, description = "Roadmap item ID")
    ),
    responses(
        (status = 200, description = "[PUBLIC] Get roadmap item by ID", body = ResponseSuccessDto<RoadmapDetailItemDto>)
    ),
    tag = "Roadmap"
)]
pub async fn get_roadmap_by_id(
	Extension(service): Extension<Arc<dyn RoadmapService>>,
	Path(id): Path<String>,
) -> Response {
	let uuid = match Uuid::parse_str(&id) {
		Ok(u) => u,
		Err(e) => {
			return ApiMessage::new(
				axum::http::StatusCode::BAD_REQUEST,
				format!("Invalid UUID: {e}"),
			)
			.into_response();
		}
	};
	match service.get(uuid).await {
		Ok(item) => ApiSuccess(RoadmapDetailItemDto::from(item)).into_response(),
		Err(e) => ApiMessage::new(axum::http::StatusCode::NOT_FOUND, e.to_string())
			.into_response(),
	}
}

#[utoipa::path(
    post,
    security(("Bearer" = [])),
    path = "/v1/landing/cms/roadmap/create",
    request_body = RoadmapCreateRequestDto,
    responses(
        (status = 201, description = "[ADMIN] Create new roadmap item")
    ),
    tag = "Roadmap"
)]
pub async fn post_create_roadmap(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Extension(service): Extension<Arc<dyn RoadmapService>>,
	ValidatedJson(payload): ValidatedJson<RoadmapCreateRequestDto>,
) -> Result<impl IntoResponse, AppError> {
	require_permissions!(headers, state, [PermissionsEnum::Administrator], {
		let entity = payload.into();
		service.create(entity).await?;
		Ok(ApiMessage::created("Roadmap item created"))
	})
}

#[utoipa::path(
    patch,
    security(("Bearer" = [])),
    path = "/v1/landing/cms/roadmap/update/{id}",
    params(
        ("id" = String, Path, description = "Roadmap item ID")
    ),
    request_body = RoadmapUpdateRequestDto,
    responses(
        (status = 200, description = "[ADMIN] Update roadmap item")
    ),
    tag = "Roadmap"
)]
pub async fn patch_update_roadmap(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Extension(service): Extension<Arc<dyn RoadmapService>>,
	Path(id): Path<String>,
	ValidatedJson(payload): ValidatedJson<RoadmapUpdateRequestDto>,
) -> Result<impl IntoResponse, AppError> {
	require_permissions!(headers, state, [PermissionsEnum::Administrator], {
		let uuid = Uuid::parse_str(&id)
			.map_err(|e| AppError::BadRequestError(format!("Invalid UUID: {e}")))?;
		let existing = service.get(uuid).await?;
		let entity = crate::roadmap::domain::RoadmapEntity {
			id: existing.id,
			title: payload.title,
			description: payload.description,
			status: payload.status,
			votes: existing.votes,
			is_deleted: existing.is_deleted,
			created_at: existing.created_at,
			updated_at: chrono::Utc::now(),
		};
		service.update(entity).await?;
		Ok(ApiMessage::ok("Roadmap item updated"))
	})
}

#[utoipa::path(
    delete,
    security(("Bearer" = [])),
    path = "/v1/landing/cms/roadmap/delete/{id}",
    params(
        ("id" = String, Path, description = "Roadmap item ID")
    ),
    responses(
        (status = 200, description = "[ADMIN] Soft delete roadmap item")
    ),
    tag = "Roadmap"
)]
pub async fn delete_roadmap(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Extension(service): Extension<Arc<dyn RoadmapService>>,
	Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
	require_permissions!(headers, state, [PermissionsEnum::Administrator], {
		let uuid = Uuid::parse_str(&id)
			.map_err(|e| AppError::BadRequestError(format!("Invalid UUID: {e}")))?;
		service.delete(uuid).await?;
		Ok(ApiMessage::ok("Roadmap item deleted"))
	})
}

#[utoipa::path(
    post,
    path = "/v1/landing/cms/roadmap/vote/{id}",
    params(
        ("id" = String, Path, description = "Roadmap item ID")
    ),
    responses(
        (status = 200, description = "[PUBLIC] Vote for a roadmap item")
    ),
    tag = "Roadmap"
)]
pub async fn post_vote_roadmap(
	Extension(service): Extension<Arc<dyn RoadmapService>>,
	Path(id): Path<String>,
) -> Response {
	let uuid = match Uuid::parse_str(&id) {
		Ok(u) => u,
		Err(e) => {
			return ApiMessage::new(
				axum::http::StatusCode::BAD_REQUEST,
				format!("Invalid UUID: {e}"),
			)
			.into_response();
		}
	};
	match service.vote(uuid).await {
		Ok(()) => ApiMessage::ok("Vote recorded").into_response(),
		Err(e) => ApiMessage::new(axum::http::StatusCode::BAD_REQUEST, e.to_string())
			.into_response(),
	}
}
