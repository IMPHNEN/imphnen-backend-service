use super::super::dto::{UsersDetailItemDto, UsersListItemDto};
use crate::require_permissions;
use crate::users::domain::UserService;
use axum::{Extension, extract::Path, http::HeaderMap, response::IntoResponse};
use imphnen_entities::{
	PermissionsEnum, ResponseListSuccessDto, ResponseSuccessDto,
};
use imphnen_libs::AppState;
use imphnen_utils::{ApiPaginated, ApiSuccess, AppError};
use paginator_axum::PaginationQuery;
use paginator_utils::PaginatorResponse;
use std::sync::Arc;
use uuid::Uuid;

#[utoipa::path(
    get,
    path = "/v1/users",
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
    path = "/v1/users/detail/{id}",
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
    path = "/v1/users/me",
    security(("Bearer" = [])),
    responses(
        (status = 200, description = "[USER] Get current user", body = ResponseSuccessDto<UsersDetailItemDto>)
    ),
    tag = "Users"
)]
pub async fn get_user_me(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Extension(service): Extension<Arc<dyn UserService>>,
) -> Result<impl IntoResponse, AppError> {
	let (claims, _) = crate::permissions_guard(
		headers,
		axum::extract::Extension(state.clone()),
		vec![],
	)
	.await?;
	let user = service.get_me(claims.user_id).await?;
	if user.is_deleted {
		return Err(AppError::NotFoundError("User not found".to_string()));
	}
	Ok(ApiSuccess(UsersDetailItemDto::from(user)))
}
