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
	async fn test_create_permission_service() {
		let app_state = crate::get_app_state().await;
		let repo = imphnen_iam::PermissionsRepository::new(&app_state);

		// Test data
		let permission_name = "test_permission_service".to_string();
		let permission_request = PermissionsCreateRequestDto {
			name: permission_name.clone(),
		};

		// Create permission through service
		let response = imphnen_iam::PermissionsService::create_role(
			&app_state,
			permission_request.clone(),
		)
		.await;

		// Verify response
		assert_eq!(response.status(), StatusCode::CREATED);

		// Verify permission was created in database
		let created_permission = repo
			.query_permission_by_name(permission_name)
			.await
			.unwrap();
		assert_eq!(created_permission.name, permission_name);

		// Clean up
		let _ = repo.query_delete_permission(created_permission.id.id.to_raw()).await;
	}
}

	#[tokio::test]
	async fn test_get_permission_by_id_service() {
		let app_state = crate::get_app_state().await;
		let repo = imphnen_iam::PermissionsRepository::new(&app_state);

		// Create test permission
		let permission_name = "test_permission_by_id_service".to_string();
		let permission = PermissionsSchema {
			name: permission_name.clone(),
			..Default::default()
		};
		let create_result = repo.query_create_permission(permission.clone()).await;
		assert!(create_result.is_ok());

		// Get created permission to get ID
		let created_permission = repo
			.query_permission_by_name(permission_name)
			.await
			.unwrap();
		let permission_id = created_permission.id.id.to_raw();

		// Get permission by ID through service
		let response = imphnen_iam::PermissionsService::get_permission_by_id(
			&app_state, permission_id.clone(),
		)
		.await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);

		// Clean up
		let _ = repo.query_delete_permission(permission_id).await;
	}

	#[tokio::test]
	async fn test_update_permission_service() {
		let app_state = crate::get_app_state().await;
		let repo = imphnen_iam::PermissionsRepository::new(&app_state);

		// Create test permission
		let original_name = "test_permission_update_original_service".to_string();
		let new_name = "test_permission_update_updated_service".to_string();
		
		let permission = PermissionsSchema {
			name: original_name.clone(),
			..Default::default()
		};
		let create_result = repo.query_create_permission(permission.clone()).await;
		assert!(create_result.is_ok());

		// Get created permission to get ID
		let created_permission = repo
			.query_permission_by_name(original_name)
			.await
			.unwrap();
		let permission_id = created_permission.id.id.to_raw();

		// Prepare update request
		let update_request = PermissionsUpdateRequestDto {
			name: Some(new_name.clone()),
		};

		// Update permission through service
		let response = imphnen_iam::PermissionsService::update_permission(
			&app_state, update_request, permission_id.clone(),
		)
		.await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);

		// Verify permission was updated in database
		let updated_permission = repo
			.query_permission_by_id(permission_id.clone())
			.await
			.unwrap();
		assert_eq!(updated_permission.name, new_name);

		// Clean up
		let _ = repo.query_delete_permission(permission_id).await;
	}

	#[tokio::test]
	async fn test_delete_permission_service() {
		let app_state = crate::get_app_state().await;
		let repo = imphnen_iam::PermissionsRepository::new(&app_state);

		// Create test permission
		let permission_name = "test_permission_delete_service".to_string();
		let permission = PermissionsSchema {
			name: permission_name.clone(),
			..Default::default()
		};
		let create_result = repo.query_create_permission(permission.clone()).await;
		assert!(create_result.is_ok());

		// Get created permission to get ID
		let created_permission = repo
			.query_permission_by_name(permission_name)
			.await
			.unwrap();
		let permission_id = created_permission.id.id.to_raw();

		// Verify permission exists before deletion
		let exists_before = repo.query_permission_by_id(permission_id.clone()).await.is_ok();
		assert!(exists_before);

		// Delete permission through service
		let response = imphnen_iam::PermissionsService::delete_permission(
			&app_state, permission_id.clone(),
		)
		.await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);

		// Verify permission was deleted from database
		let exists_after = repo.query_permission_by_id(permission_id).await.is_ok();
		assert!(!exists_after);
	}
}

	#[tokio::test]
	async fn test_get_permission_by_id_service_not_found() {
		let app_state = crate::get_app_state().await;

		// Use non-existent ID
		let non_existent_id = "non-existent-uuid-123456789".to_string();

		// Get non-existent permission by ID through service
		let response = imphnen_iam::PermissionsService::get_permission_by_id(
			&app_state, non_existent_id,
		)
		.await;

		// Verify not found response
		assert_eq!(response.status(), StatusCode::NOT_FOUND);
	}

	#[tokio::test]
	async fn test_update_permission_service_not_found() {
		let app_state = crate::get_app_state().await;

		// Use non-existent ID
		let non_existent_id = "non-existent-uuid-123456789".to_string();

		// Prepare update request
		let update_request = PermissionsUpdateRequestDto {
			name: Some("new_name".to_string()),
		};

		// Update non-existent permission through service
		let response = imphnen_iam::PermissionsService::update_permission(
			&app_state, update_request, non_existent_id,
		)
		.await;

		// Verify not found response
		assert_eq!(response.status(), StatusCode::NOT_FOUND);
	}

	#[tokio::test]
	async fn test_delete_permission_service_not_found() {
		let app_state = crate::get_app_state().await;

		// Use non-existent ID
		let non_existent_id = "non-existent-uuid-123456789".to_string();

		// Delete non-existent permission through service
		let response = imphnen_iam::PermissionsService::delete_permission(
			&app_state, non_existent_id,
		)
		.await;

		// Verify not found response
		assert_eq!(response.status(), StatusCode::NOT_FOUND);
	#[tokio::test]
	async fn test_create_permission_service_duplicate_name() {
		let app_state = crate::get_app_state().await;
		let repo = imphnen_iam::PermissionsRepository::new(&app_state);

		// Test data
		let permission_name = "test_permission_duplicate_service".to_string();
		let permission_request = PermissionsCreateRequestDto {
			name: permission_name.clone(),
		};

		// Create permission first
		let response1 = imphnen_iam::PermissionsService::create_role(
			&app_state,
			permission_request.clone(),
		)
		.await;
		assert_eq!(response1.status(), StatusCode::CREATED);

		// Try to create again with same name
		let response2 = imphnen_iam::PermissionsService::create_role(
			&app_state,
			permission_request,
		)
		.await;

		// Verify response - should fail
		assert_eq!(response2.status(), StatusCode::CONFLICT);

		let error_response: crate::MessageResponseDto = response2.into_body().await.unwrap();
		assert_eq!(error_response.message, "Permission name already exists");

		// Clean up
		let created_permission = repo
			.query_permission_by_name(permission_name)
			.await
			.unwrap();
		let _ = repo.query_delete_permission(created_permission.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_get_permission_list_service() {
		let app_state = crate::get_app_state().await;
		let repo = imphnen_iam::PermissionsRepository::new(&app_state);

		// Create test permission
		let permission_name = "test_permission_list_service".to_string();
		let permission = PermissionsSchema {
			name: permission_name.clone(),
			..Default::default()
		};
		let create_result = repo.query_create_permission(permission.clone()).await;
		assert!(create_result.is_ok());

		// Get permission list through service
		let meta = imphnen_iam::MetaRequestDto {
			page: Some(1),
			limit: Some(10),
			..Default::default()
		};
		let response = imphnen_iam::PermissionsService::get_permission_list(
			&app_state, meta,
		)
		.await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);

		// Clean up
		let created_permission = repo
			.query_permission_by_name(permission_name)
			.await
			.unwrap();
		let _ = repo.query_delete_permission(created_permission.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_update_permission_service_duplicate_name() {
		let app_state = crate::get_app_state().await;
		let repo = imphnen_iam::PermissionsRepository::new(&app_state);

		// Create two test permissions
		let permission_name1 = "test_permission_update_dup1_service".to_string();
		let permission_name2 = "test_permission_update_dup2_service".to_string();

		let permission1 = PermissionsSchema {
			name: permission_name1.clone(),
			..Default::default()
		};
		let permission2 = PermissionsSchema {
			name: permission_name2.clone(),
			..Default::default()
		};

		let create_result1 = repo.query_create_permission(permission1.clone()).await;
		assert!(create_result1.is_ok());
		let create_result2 = repo.query_create_permission(permission2.clone()).await;
		assert!(create_result2.is_ok());

		// Get created permissions
		let created_permission1 = repo
			.query_permission_by_name(permission_name1.clone())
			.await
			.unwrap();
		let permission_id1 = created_permission1.id.id.to_raw();

		// Try to update permission1 to have same name as permission2
		let update_request = PermissionsUpdateRequestDto {
			name: Some(permission_name2.clone()),
		};

		let response = imphnen_iam::PermissionsService::update_permission(
			&app_state, update_request, permission_id1.clone(),
		)
		.await;

		// Verify response - should fail
		assert_eq!(response.status(), StatusCode::BAD_REQUEST);

		let error_response: crate::MessageResponseDto = response.into_body().await.unwrap();
		assert!(error_response.message.contains("not found") || error_response.message.contains("Permission not found"));

		// Clean up
		let _ = repo.query_delete_permission(permission_id1).await;
		let created_permission2 = repo
			.query_permission_by_name(permission_name2)
			.await
			.unwrap();
		let _ = repo.query_delete_permission(created_permission2.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_update_permission_service_no_changes() {
		let app_state = crate::get_app_state().await;
		let repo = imphnen_iam::PermissionsRepository::new(&app_state);

		// Create test permission
		let permission_name = "test_permission_no_change_service".to_string();
		let permission = PermissionsSchema {
			name: permission_name.clone(),
			..Default::default()
		};
		let create_result = repo.query_create_permission(permission.clone()).await;
		assert!(create_result.is_ok());

		// Get created permission to get ID
		let created_permission = repo
			.query_permission_by_name(permission_name)
			.await
			.unwrap();
		let permission_id = created_permission.id.id.to_raw();

		// Update with no changes
		let update_request = PermissionsUpdateRequestDto {
			name: None, // No changes
		};

		let response = imphnen_iam::PermissionsService::update_permission(
			&app_state, update_request, permission_id.clone(),
		)
		.await;

		// Verify response - should succeed
		assert_eq!(response.status(), StatusCode::OK);

		// Clean up
		let _ = repo.query_delete_permission(permission_id).await;
	}
	}
}