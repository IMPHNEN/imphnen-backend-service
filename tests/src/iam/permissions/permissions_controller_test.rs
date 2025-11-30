#[cfg(test)]
mod tests {
	use crate::{generate_unique_email, get_role_id, UsersRepository};
	use axum::{http::StatusCode, response::Response};
	use imphnen_iam::{
		PermissionsCreateRequestDto, PermissionsUpdateRequestDto, PermissionsSchema,
		ResourceEnum,
	};
	use imphnen_utils::{make_thing_from_enum, ResourceEnum as UtilsResourceEnum};

	#[tokio::test]
	async fn test_create_permission_controller() {
		let app_state = crate::get_app_state().await;
		let repo = imphnen_iam::PermissionsRepository::new(&app_state);

		// Test data
		let permission_name = "test_permission_controller".to_string();
		let permission_request = PermissionsCreateRequestDto {
			name: permission_name.clone(),
		};

		// Create permission through controller
		let response = imphnen_iam::PermissionsController::create_permission(
			&app_state,
			permission_request.clone(),
		)
		.await;

		// Verify response
		assert_eq!(response.status(), StatusCode::CREATED);

		// Verify response body contains permission data
		let created_permission: PermissionsSchema =
			crate::common::response_helpers::parse_response(response, 1024).await;
		
		// Validate all required fields in PermissionsSchema
		assert!(!created_permission.id.id.to_raw().is_empty(), "Created permission must have non-empty id");
		assert_eq!(created_permission.name, permission_name, "Created permission name must match request");
		assert!(created_permission.created_at.is_some(), "Created permission must have created_at timestamp");
		assert!(created_permission.updated_at.is_some(), "Created permission must have updated_at timestamp");
		assert!(created_permission.is_active == true, "Created permission should be active by default");
		assert!(created_permission.is_deleted == false, "Created permission should not be deleted by default");

		// Verify permission was created in database
		let db_permission = repo
			.query_permission_by_name(permission_name)
			.await
			.unwrap();
		assert_eq!(db_permission.name, permission_name);

		// Clean up
		let _ = repo.query_delete_permission(db_permission.id.id.to_raw()).await;
	}
}