use std::sync::Arc;
use axum::{Extension, extract::Path, http::HeaderMap, http::StatusCode, response::{IntoResponse, Response}};
use paginator_axum::PaginationQuery;
use paginator_utils::PaginatorResponse;
use uuid::Uuid;
use imphnen_libs::{AppState, ValidatedJson};
use imphnen_utils::{ApiSuccess, ApiCreated, ApiPaginated, ApiMessage, extract_email};
use imphnen_entities::ResponseSuccessDto;
use imphnen_iam::require_auth;
use imphnen_utils::AppError;
use super::dto::{
    TestimonialsCreateRequestDto, TestimonialsDetailItemDto,
    TestimonialsListItemDto, TestimonialsUpdateRequestDto,
};
use crate::testimonials::domain::{TestimonialEntity, TestimonialService};

#[utoipa::path(
    get,
    path = "/v1/cms/landing/testimonials",
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("per_page" = Option<i64>, Query, description = "Items per page"),
        ("search" = Option<String>, Query, description = "Search keyword"),
        ("sort_by" = Option<String>, Query, description = "Sort by field"),
        ("order" = Option<String>, Query, description = "Order ASC or DESC"),
    ),
    responses(
        (status = 200, description = "[PUBLIC] Get testimonial list")
    ),
    tag = "Testimonials"
)]
pub async fn get_testimonial_list(
    Extension(service): Extension<Arc<dyn TestimonialService>>,
    PaginationQuery(params): PaginationQuery,
) -> Response {
    match service.list(params).await {
        Ok(result) => {
            let mapped = PaginatorResponse {
                data: result.data.into_iter()
                    .filter(|e| !e.is_deleted)
                    .map(TestimonialsListItemDto::from)
                    .collect::<Vec<_>>(),
                meta: result.meta,
            };
            ApiPaginated(mapped).into_response()
        }
        Err(e) => ApiMessage::new(StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/v1/cms/landing/testimonials/detail/{id}",
    params(
        ("id" = String, Path, description = "Testimonial ID")
    ),
    responses(
        (status = 200, description = "[PUBLIC] Get testimonial by ID", body = ResponseSuccessDto<TestimonialsDetailItemDto>)
    ),
    tag = "Testimonials"
)]
pub async fn get_testimonial_by_id(
    Extension(service): Extension<Arc<dyn TestimonialService>>,
    Path(id): Path<String>,
) -> Response {
    let uuid = match Uuid::parse_str(&id) {
        Ok(u) => u,
        Err(e) => return ApiMessage::new(StatusCode::BAD_REQUEST, format!("Invalid UUID: {e}")).into_response(),
    };
    match service.get(uuid).await {
        Ok(t) if !t.is_deleted => {
            ApiSuccess(TestimonialsDetailItemDto::from(t)).into_response()
        }
        Ok(_) => ApiMessage::new(StatusCode::NOT_FOUND, "Testimonial not found").into_response(),
        Err(e) => ApiMessage::new(StatusCode::NOT_FOUND, e.to_string()).into_response(),
    }
}

#[utoipa::path(
    post,
    security(("Bearer" = [])),
    path = "/v1/cms/landing/testimonials/create",
    request_body = TestimonialsCreateRequestDto,
    responses(
        (status = 201, description = "[USER] Create new testimonial", body = ResponseSuccessDto<TestimonialsDetailItemDto>)
    ),
    tag = "Testimonials"
)]
pub async fn post_create_testimonial(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
    Extension(service): Extension<Arc<dyn TestimonialService>>,
    ValidatedJson(payload): ValidatedJson<TestimonialsCreateRequestDto>,
) -> Result<impl IntoResponse, AppError> {
    require_auth!(headers.clone(), state, {
        let email = extract_email(&headers)
            .ok_or_else(|| AppError::AuthenticationError("Token tidak valid".to_string()))?;
        let user_info = state.user_lookup_service.get_user_by_email(&email, &state).await
            .map_err(|_| AppError::NotFoundError("User not found".to_string()))?;
        let user = user_info.basic_info;
        let user_id = Uuid::parse_str(&user.id)
            .map_err(|e| AppError::BadRequestError(format!("Invalid user ID: {e}")))?;
        let entity = TestimonialEntity {
            id: Uuid::new_v4(),
            user_id,
            user_fullname: user.fullname.clone(),
            role: payload.role,
            content: payload.content,
            is_deleted: false,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };
        let created = service.create(entity).await?;
        Ok(ApiCreated(TestimonialsDetailItemDto::from(created)))
    })
}

#[utoipa::path(
    patch,
    security(("Bearer" = [])),
    path = "/v1/cms/landing/testimonials/update/{id}",
    params(
        ("id" = String, Path, description = "Testimonial ID")
    ),
    request_body = TestimonialsUpdateRequestDto,
    responses(
        (status = 200, description = "[USER] Update testimonial")
    ),
    tag = "Testimonials"
)]
pub async fn patch_update_testimonial(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
    Extension(service): Extension<Arc<dyn TestimonialService>>,
    Path(id): Path<String>,
    ValidatedJson(payload): ValidatedJson<TestimonialsUpdateRequestDto>,
) -> Result<impl IntoResponse, AppError> {
    require_auth!(headers, state, {
        let uuid = Uuid::parse_str(&id)
            .map_err(|e| AppError::BadRequestError(format!("Invalid UUID: {e}")))?;
        let existing = service.get(uuid).await?;
        let entity = TestimonialEntity {
            id: existing.id,
            user_id: existing.user_id,
            user_fullname: existing.user_fullname,
            role: payload.role,
            content: payload.content,
            is_deleted: existing.is_deleted,
            created_at: existing.created_at,
            updated_at: chrono::Utc::now().to_rfc3339(),
        };
        service.update(entity).await?;
        Ok(ApiMessage::ok("Testimonial updated"))
    })
}

#[utoipa::path(
    delete,
    security(("Bearer" = [])),
    path = "/v1/cms/landing/testimonials/delete/{id}",
    params(
        ("id" = String, Path, description = "Testimonial ID")
    ),
    responses(
        (status = 200, description = "[USER] Soft delete testimonial")
    ),
    tag = "Testimonials"
)]
pub async fn delete_testimonial(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
    Extension(service): Extension<Arc<dyn TestimonialService>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    require_auth!(headers, state, {
        let uuid = Uuid::parse_str(&id)
            .map_err(|e| AppError::BadRequestError(format!("Invalid UUID: {e}")))?;
        service.delete(uuid).await?;
        Ok(ApiMessage::ok("Testimonial deleted"))
    })
}
