use super::{
	testimonials_dto::{
		TestimonialsCreateRequestDto, TestimonialsDetailItemDto,
		TestimonialsListItemDto, TestimonialsUpdateRequestDto,
	},
	testimonials_service::TestimonialsService,
};
use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::{Extension, http::HeaderMap};
use imphnen_iam::{UsersDetailQueryDto, require_auth};
use imphnen_libs::{
    AppState, MessageResponseDto, MetaRequestDto, ResponseListSuccessDto,
    ResponseSuccessDto, ValidatedJson,
};

#[utoipa::path(
    get,
    path = "/v1/cms/landing/testimonials",
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
        (status = 200, description = "[PUBLIC] Get testimonial list", body = ResponseListSuccessDto<Vec<TestimonialsListItemDto>>)
    ),
    tag = "Testimonials"
)]
pub async fn get_testimonial_list(
	Extension(state): Extension<AppState>,
	Query(meta): Query<MetaRequestDto>,
) -> impl IntoResponse {
	TestimonialsService::get_testimonial_list(&state, meta).await
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
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
) -> impl IntoResponse {
	TestimonialsService::get_testimonial_by_id(&state, id).await
}

#[utoipa::path(
    post,
    security(
        ("Bearer" = [])
    ),
    path = "/v1/cms/landing/testimonials/create",
    request_body = TestimonialsCreateRequestDto,
    responses(
        (status = 201, description = "[USER] Create new testimonial", body = MessageResponseDto)
    ),
    tag = "Testimonials"
)]
pub async fn post_create_testimonial(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
    Extension(authenticated_user): Extension<UsersDetailQueryDto>,
    ValidatedJson(payload): ValidatedJson<TestimonialsCreateRequestDto>,
) -> impl IntoResponse {
    require_auth!(headers, state, {
        TestimonialsService::create_testimonial(&state, payload, &authenticated_user).await
    })
}

#[utoipa::path(
    patch,
    security(
        ("Bearer" = [])
    ),
    path = "/v1/cms/landing/testimonials/update/{id}",
    params(
        ("id" = String, Path, description = "Testimonial ID")
    ),
    request_body = TestimonialsUpdateRequestDto,
    responses(
        (status = 200, description = "[USER] Update testimonial", body = MessageResponseDto)
    ),
    tag = "Testimonials"
)]
pub async fn patch_update_testimonial(
    headers: HeaderMap,
    Path(id): Path<String>,
    Extension(state): Extension<AppState>,
    Extension(authenticated_user): Extension<UsersDetailQueryDto>,
    ValidatedJson(payload): ValidatedJson<TestimonialsUpdateRequestDto>,
) -> impl IntoResponse {
    require_auth!(headers, state, {
        TestimonialsService::update_testimonial(&state, id, payload, &authenticated_user).await
    })
}

#[utoipa::path(
    delete,
    security(
        ("Bearer" = [])
    ),
    path = "/v1/cms/landing/testimonials/delete/{id}",
    params(
        ("id" = String, Path, description = "Testimonial ID")
    ),
    responses(
        (status = 200, description = "[USER] Soft delete testimonial", body = MessageResponseDto)
    ),
    tag = "Testimonials"
)]
pub async fn delete_testimonial(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
    Extension(authenticated_user): Extension<UsersDetailQueryDto>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    require_auth!(headers, state, {
        TestimonialsService::delete_testimonial(&state, id, &authenticated_user).await
    })
}
