use std::sync::Arc;
use axum::{Extension, extract::Path, http::HeaderMap, response::{IntoResponse, Response}};
use paginator_axum::PaginationQuery;
use paginator_utils::PaginatorResponse;
use uuid::Uuid;
use imphnen_libs::{AppState, ValidatedJson};
use imphnen_utils::{ApiSuccess, ApiPaginated, ApiMessage};
use imphnen_entities::ResponseSuccessDto;
use imphnen_iam::{PermissionsEnum, require_permissions};
use imphnen_utils::AppError;
use super::dto::{EventsCreateRequestDto, EventsDetailItemDto, EventsListItemDto, EventsUpdateRequestDto};
use crate::events::domain::EventService;

#[utoipa::path(
    get,
    path = "/v1/cms/landing/events",
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("per_page" = Option<i64>, Query, description = "Items per page"),
        ("search" = Option<String>, Query, description = "Search keyword"),
        ("sort_by" = Option<String>, Query, description = "Sort by field"),
        ("order" = Option<String>, Query, description = "Order ASC or DESC"),
    ),
    responses(
        (status = 200, description = "[PUBLIC] Get event list")
    ),
    tag = "Events"
)]
pub async fn get_event_list(
    Extension(service): Extension<Arc<dyn EventService>>,
    PaginationQuery(params): PaginationQuery,
) -> Response {
    match service.list(params).await {
        Ok(result) => {
            let mapped = PaginatorResponse {
                data: result.data.into_iter().map(EventsListItemDto::from).collect::<Vec<_>>(),
                meta: result.meta,
            };
            ApiPaginated(mapped).into_response()
        }
        Err(e) => ApiMessage::new(axum::http::StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/v1/cms/landing/events/detail/{id}",
    params(
        ("id" = String, Path, description = "Event ID")
    ),
    responses(
        (status = 200, description = "[PUBLIC] Get event by ID", body = ResponseSuccessDto<EventsDetailItemDto>)
    ),
    tag = "Events"
)]
pub async fn get_event_by_id(
    Extension(service): Extension<Arc<dyn EventService>>,
    Path(id): Path<String>,
) -> Response {
    let uuid = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(e) => return ApiMessage::new(axum::http::StatusCode::BAD_REQUEST, format!("Invalid UUID: {e}")).into_response(),
    };
    match service.get(uuid).await {
        Ok(event) => ApiSuccess(EventsDetailItemDto::from(event)).into_response(),
        Err(e) => ApiMessage::new(axum::http::StatusCode::NOT_FOUND, e.to_string()).into_response(),
    }
}

#[utoipa::path(
    post,
    security(("Bearer" = [])),
    path = "/v1/cms/landing/events/create",
    request_body = EventsCreateRequestDto,
    responses(
        (status = 201, description = "[ADMIN] Create new event")
    ),
    tag = "Events"
)]
pub async fn post_create_event(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
    Extension(service): Extension<Arc<dyn EventService>>,
    ValidatedJson(payload): ValidatedJson<EventsCreateRequestDto>,
) -> Result<impl IntoResponse, AppError> {
    require_permissions!(headers, state, [PermissionsEnum::Administrator], {
        let entity = payload.into();
        service.create(entity).await?;
        Ok(ApiMessage::created("Event created"))
    })
}

#[utoipa::path(
    patch,
    security(("Bearer" = [])),
    path = "/v1/cms/landing/events/update/{id}",
    params(
        ("id" = String, Path, description = "Event ID")
    ),
    request_body = EventsUpdateRequestDto,
    responses(
        (status = 200, description = "[ADMIN] Update event")
    ),
    tag = "Events"
)]
pub async fn patch_update_event(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
    Extension(service): Extension<Arc<dyn EventService>>,
    Path(id): Path<String>,
    ValidatedJson(payload): ValidatedJson<EventsUpdateRequestDto>,
) -> Result<impl IntoResponse, AppError> {
    require_permissions!(headers, state, [PermissionsEnum::Administrator], {
        let uuid = Uuid::parse_str(&id)
            .map_err(|e| AppError::BadRequestError(format!("Invalid UUID: {e}")))?;
        let existing = service.get(uuid).await?;
        let entity = crate::events::domain::EventEntity {
            id: existing.id,
            name: payload.name,
            description: payload.description,
            detail_link: payload.detail_link,
            price: payload.price,
            is_online: payload.is_online,
            location: payload.location,
            start_date: payload.start_date,
            end_date: payload.end_date,
            is_deleted: existing.is_deleted,
            created_at: existing.created_at,
            updated_at: chrono::Utc::now(),
        };
        service.update(entity).await?;
        Ok(ApiMessage::ok("Event updated"))
    })
}

#[utoipa::path(
    delete,
    security(("Bearer" = [])),
    path = "/v1/cms/landing/events/delete/{id}",
    params(
        ("id" = String, Path, description = "Event ID")
    ),
    responses(
        (status = 200, description = "[ADMIN] Soft delete event")
    ),
    tag = "Events"
)]
pub async fn delete_event(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
    Extension(service): Extension<Arc<dyn EventService>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    require_permissions!(headers, state, [PermissionsEnum::Administrator], {
        let uuid = Uuid::parse_str(&id)
            .map_err(|e| AppError::BadRequestError(format!("Invalid UUID: {e}")))?;
        service.delete(uuid).await?;
        Ok(ApiMessage::ok("Event deleted"))
    })
}
