use crate::{AppState, MetaRequestDto, ResponseListSuccessDto, ResponseSuccessDto};
use imphnen_entities::MessageResponseDto;
use crate::v1::gacha_items::GachaItemDto;
use crate::v1::gacha_items::gacha_items_dto::{GachaItemRequestDto, GachaItemUpdateRequestDto};
use crate::v1::gacha_items::gacha_items_service::GachaItemService;
use axum::{
	Extension,
	extract::{Path, Query},
	http::HeaderMap,
	response::IntoResponse,
};
use imphnen_iam::{PermissionsEnum, require_permissions};
use imphnen_libs::ValidatedJson;

#[utoipa::path(
    get,
    path = "/v1/gacha/items",
    security(
        ("Bearer" = [])
    ),
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
        (status = 200, description = "[ADMIN] Get gacha item list", body = ResponseListSuccessDto<Vec<GachaItemDto>>)
    ),
    tag = "Gacha"
)]
pub async fn get_gacha_item_list(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Query(meta): Query<MetaRequestDto>,
) -> impl IntoResponse {
	require_permissions!(headers, state, [PermissionsEnum::ReadListGachaItems], {
		GachaItemService::get_gacha_item_list(&state, meta).await
	})
}

#[utoipa::path(
    get,
    path = "/v1/gacha/items/detail/{id}",
    security(
        ("Bearer" = [])
    ),
    params(("id" = String, Path, description = "Gacha Item ID")),
    responses(
        (status = 200, description = "[ADMIN] Get gacha item by ID", body = ResponseSuccessDto<GachaItemDto>)
    ),
    tag = "Gacha"
)]
pub async fn get_gacha_item_by_id(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
) -> impl IntoResponse {
	require_permissions!(headers, state, [PermissionsEnum::ReadDetailGachaItems], {
		GachaItemService::get_gacha_item_by_id(&state, id).await
	})
}

#[utoipa::path(
    post,
    path = "/v1/gacha/items/create",
    security(
        ("Bearer" = [])
    ),
    request_body = GachaItemRequestDto,
    responses(
        (status = 201, description = "[ADMIN] Create gacha item", body = MessageResponseDto)
    ),
    tag = "Gacha"
)]
pub async fn post_create_gacha_item(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	ValidatedJson(payload): ValidatedJson<GachaItemRequestDto>,
) -> impl IntoResponse {
	require_permissions!(headers, state, [PermissionsEnum::CreateGachaItems], {
		GachaItemService::create_gacha_item(&state, payload).await
	})
}

#[utoipa::path(
    put,
    path = "/v1/gacha/items/update/{id}",
    security(
        ("Bearer" = [])
    ),
    request_body = GachaItemUpdateRequestDto,
    responses(
        (status = 200, description = "[ADMIN] Update gacha item", body = MessageResponseDto)
    ),
    tag = "Gacha"
)]
pub async fn put_update_gacha_item(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
	ValidatedJson(payload): ValidatedJson<GachaItemUpdateRequestDto>,
) -> impl IntoResponse {
	require_permissions!(headers, state, [PermissionsEnum::UpdateGachaItems], {
		GachaItemService::update_gacha_item(&state, payload, id).await
	})
}

#[utoipa::path(
    delete,
    path = "/v1/gacha/items/delete/{id}",
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 200, description = "[ADMIN] Delete gacha item", body = MessageResponseDto)
    ),
    tag = "Gacha"
)]
pub async fn delete_gacha_item(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
) -> impl IntoResponse {
	require_permissions!(headers, state, [PermissionsEnum::DeleteGachaItems], {
		GachaItemService::delete_gacha_item(&state, id).await
	})
}
