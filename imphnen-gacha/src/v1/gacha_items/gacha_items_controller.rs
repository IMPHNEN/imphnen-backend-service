use crate::{
	AppState, GachaItemDto, GachaItemRequestDto, GachaItemService, MessageResponseDto,
	MetaRequestDto, ResponseListSuccessDto, ResponseSuccessDto,
};
use axum::{
	Extension, Json,
	extract::{Path, Query},
	http::HeaderMap,
	response::IntoResponse,
};
use imphnen_iam::{PermissionsEnum, permissions_guard};

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
        (status = 200, description = "Get gacha item list", body = ResponseListSuccessDto<Vec<GachaItemDto>>)
    ),
    tag = "Gacha"
)]
pub async fn get_gacha_item_list(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Query(meta): Query<MetaRequestDto>,
) -> impl IntoResponse {
	match permissions_guard(
		&headers,
		state.clone(),
		vec![PermissionsEnum::ReadListGachaItems],
	)
	.await
	{
		Ok(_) => GachaItemService::get_gacha_item_list(&state, meta).await,
		Err(response) => response,
	}
}

#[utoipa::path(
    get,
    path = "/v1/gacha/items/detail/{id}",
    security(
        ("Bearer" = [])
    ),
    params(("id" = String, Path, description = "Gacha Item ID")),
    responses(
        (status = 200, description = "Get gacha item by ID", body = ResponseSuccessDto<GachaItemDto>)
    ),
    tag = "Gacha"
)]
pub async fn get_gacha_item_by_id(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
) -> impl IntoResponse {
	match permissions_guard(
		&headers,
		state.clone(),
		vec![PermissionsEnum::ReadDetailGachaItems],
	)
	.await
	{
		Ok(_) => GachaItemService::get_gacha_item_by_id(&state, id).await,
		Err(response) => response,
	}
}

#[utoipa::path(
    post,
    path = "/v1/gacha/items/create",
    security(
        ("Bearer" = [])
    ),
    request_body = GachaItemRequestDto,
    responses(
        (status = 201, description = "Create gacha item", body = MessageResponseDto)
    ),
    tag = "Gacha"
)]
pub async fn post_create_gacha_item(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Json(payload): Json<GachaItemRequestDto>,
) -> impl IntoResponse {
	match permissions_guard(
		&headers,
		state.clone(),
		vec![PermissionsEnum::CreateGachaItems],
	)
	.await
	{
		Ok(_) => GachaItemService::create_gacha_item(&state, payload).await,
		Err(response) => response,
	}
}

#[utoipa::path(
    put,
    path = "/v1/gacha/items/update/{id}",
    security(
        ("Bearer" = [])
    ),
    request_body = GachaItemRequestDto,
    responses(
        (status = 200, description = "Update gacha item", body = MessageResponseDto)
    ),
    tag = "Gacha"
)]
pub async fn put_update_gacha_item(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
	Json(payload): Json<GachaItemRequestDto>,
) -> impl IntoResponse {
	match permissions_guard(
		&headers,
		state.clone(),
		vec![PermissionsEnum::UpdateGachaItems],
	)
	.await
	{
		Ok(_) => GachaItemService::update_gacha_item(&state, payload, id).await,
		Err(response) => response,
	}
}

#[utoipa::path(
    delete,
    path = "/v1/gacha/items/delete/{id}",
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 200, description = "Delete gacha item", body = MessageResponseDto)
    ),
    tag = "Gacha"
)]
pub async fn delete_gacha_item(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
) -> impl IntoResponse {
	match permissions_guard(
		&headers,
		state.clone(),
		vec![PermissionsEnum::DeleteGachaItems],
	)
	.await
	{
		Ok(_) => GachaItemService::delete_gacha_item(&state, id).await,
		Err(response) => response,
	}
}
