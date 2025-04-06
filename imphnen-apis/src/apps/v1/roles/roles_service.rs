use super::{RolesRepository, RolesRequestCreateDto, RolesRequestUpdateDto};
use crate::{
	common_response, success_list_response, success_response, validate_request,
	AppState, MetaRequestDto, ResponseListSuccessDto, ResponseSuccessDto,
};
use axum::{http::StatusCode, response::Response};

pub struct RolesService;

impl RolesService {
	pub async fn get_role_list(state: &AppState, meta: MetaRequestDto) -> Response {
		let repo = RolesRepository::new(state);
		match repo.query_role_list(meta).await {
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

	pub async fn get_role_by_id(state: &AppState, id: String) -> Response {
		let repo = RolesRepository::new(state);
		match repo.query_role_by_id(id).await {
			Ok(role) => success_response(ResponseSuccessDto { data: role }),
			Err(e) => common_response(StatusCode::NOT_FOUND, &e.to_string()),
		}
	}

	pub async fn create_role(
		state: &AppState,
		payload: RolesRequestCreateDto,
	) -> Response {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}
		let repo = RolesRepository::new(state);
		match repo.query_role_by_name(payload.name.clone()).await {
			Ok(_role) => {
				return common_response(StatusCode::CONFLICT, "Role name already exists");
			}
			Err(err) if err.to_string().contains("not found") => {}
			Err(e) => {
				return common_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string());
			}
		}
		match repo.query_create_role(payload).await {
			Ok(msg) => common_response(StatusCode::CREATED, &msg),
			Err(e) => common_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
		}
	}

	pub async fn update_role(
		state: &AppState,
		id: String,
		payload: RolesRequestUpdateDto,
	) -> Response {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}
		let repo = RolesRepository::new(state);
		let existing_role = match repo.query_role_by_id(id.clone()).await {
			Ok(role) => role,
			Err(err) if err.to_string().contains("not found") => {
				return common_response(StatusCode::NOT_FOUND, "Role not found");
			}
			Err(e) => {
				return common_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string());
			}
		};
		if let Some(new_name) = payload.name.clone() {
			match repo.query_role_by_name(new_name.clone()).await {
				Ok(role_with_same_name) => {
					if role_with_same_name.id != existing_role.id {
						return common_response(
							StatusCode::CONFLICT,
							"Role name already exists",
						);
					}
				}
				Err(err) if err.to_string().contains("not found") => {}
				Err(e) => {
					return common_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string());
				}
			}
		}
		match repo.query_update_role(id, payload).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
		}
	}

	pub async fn delete_role(state: &AppState, id: String) -> Response {
		let repo = RolesRepository::new(state);
		match repo.query_role_by_id(id.clone()).await {
			Ok(_) => {}
			Err(err) if err.to_string().contains("not found") => {
				return common_response(StatusCode::NOT_FOUND, "Role not found");
			}
			Err(e) => {
				return common_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string());
			}
		}
		match repo.query_delete_role(id).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
		}
	}
}
