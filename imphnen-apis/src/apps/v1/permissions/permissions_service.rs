use crate::{
	common_response, make_thing, success_list_response, success_response,
	validate_request, AppState, MetaRequestDto, PermissionsRepository,
	PermissionsSchema, ResourceEnum, ResponseListSuccessDto, ResponseSuccessDto,
};
use axum::http::StatusCode;
use axum::response::Response;

use super::PermissionsRequestDto;

pub struct PermissionsService;

impl PermissionsService {
	pub async fn get_permission_list(
		state: &AppState,
		meta: MetaRequestDto,
	) -> Response {
		let repo = PermissionsRepository::new(state);
		match repo.query_permission_list(meta).await {
			Ok(data) => {
				let response = ResponseListSuccessDto {
					data: data.data,
					meta: data.meta,
				};
				success_list_response(response)
			}
			Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
		}
	}

	pub async fn get_permission_by_id(state: &AppState, id: String) -> Response {
		let repo = PermissionsRepository::new(state);
		match repo.query_permission_by_id(id).await {
			Ok(permission) => success_response(ResponseSuccessDto { data: permission }),
			Err(e) => common_response(StatusCode::NOT_FOUND, &e.to_string()),
		}
	}

	pub async fn create_role(
		state: &AppState,
		payload: PermissionsRequestDto,
	) -> Response {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}
		let repo = PermissionsRepository::new(state);
		match repo.query_permission_by_name(payload.name.clone()).await {
			Ok(_role) => {
				return common_response(
					StatusCode::CONFLICT,
					"Permission name already exists",
				);
			}
			Err(err) if err.to_string().contains("not found") => {}
			Err(e) => {
				return common_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string());
			}
		}
		match repo
			.query_create_permission(PermissionsSchema {
				name: payload.name,
				..Default::default()
			})
			.await
		{
			Ok(msg) => common_response(StatusCode::CREATED, &msg),
			Err(e) => common_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
		}
	}

	pub async fn update_permission(
		state: &AppState,
		payload: PermissionsRequestDto,
		id: String,
	) -> Response {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}
		let repo = PermissionsRepository::new(state);
		match repo
			.query_update_permission(PermissionsSchema {
				id: make_thing(&ResourceEnum::Permissions.to_string(), &id),
				name: payload.name,
				..Default::default()
			})
			.await
		{
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(e) => {
				if e.to_string().contains("not found") {
					common_response(StatusCode::NOT_FOUND, "Permission not found")
				} else {
					common_response(StatusCode::BAD_REQUEST, &e.to_string())
				}
			}
		}
	}

	pub async fn delete_permission(state: &AppState, id: String) -> Response {
		let repo = PermissionsRepository::new(state);
		match repo.query_delete_permission(id).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(e) => {
				if e.to_string().contains("not found") {
					common_response(StatusCode::NOT_FOUND, "Permission not found")
				} else {
					common_response(StatusCode::BAD_REQUEST, &e.to_string())
				}
			}
		}
	}
}
