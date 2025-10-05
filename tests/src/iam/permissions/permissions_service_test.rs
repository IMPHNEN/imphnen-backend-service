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

		// Verify response body contains created permission data
		let created: PermissionsSchema =
			crate::common::response_helpers::parse_response(response, 1024).await;
		
		// Validate all required fields in PermissionsSchema
		assert!(!created.id.is_empty(), "Created permission must have non-empty id");
		assert_eq!(created.name, permission_name, "Created permission name must match request");
		assert!(created.created_at.is_some(), "Created permission must have created_at timestamp");
		assert!(created.updated_at.is_some(), "Created permission must have updated_at timestamp");
		assert!(created.is_active == true, "Created permission should be active by default");
		assert!(created.is_deleted == false, "Created permission should not be deleted by default");

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

		// Verify response body contains permission data
		let body: PermissionsSchema =
			crate::common::response_helpers::parse_response(response, 1024).await;
		
		// Validate all required fields in PermissionsSchema
		assert!(!body.id.id.to_raw().is_empty(), "Permission must have non-empty id");
		assert_eq!(body.id.id.to_raw(), permission_id, "Permission ID must match");
		assert_eq!(body.name, permission_name, "Permission name must match");
		assert!(body.created_at.is_some(), "Permission must have created_at timestamp");
		assert!(body.updated_at.is_some(), "Permission must have updated_at timestamp");
		assert!(body.is_active == true, "Permission should be active");
		assert!(body.is_deleted == false, "Permission should not be deleted");

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

		// Verify response body contains updated permission data
		let body: PermissionsSchema =
			crate::common::response_helpers::parse_response(response, 1024).await;
		
		// Validate all required fields in PermissionsSchema
		assert!(!body.id.id.to_raw().is_empty(), "Updated permission must have non-empty id");
		assert_eq!(body.name, new_name, "Updated permission name must match request");
		assert!(body.created_at.is_some(), "Updated permission must have created_at timestamp");
		assert!(body.updated_at.is_some(), "Updated permission must have updated_at timestamp");
		assert!(body.is_active == true, "Updated permission should be active");
		assert!(body.is_deleted == false, "Updated permission should not be deleted");

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

		#[cfg(test)]
		mod tests {
		    use axum::http::StatusCode;
		    use imphnen_iam::{
		        PermissionsCreateRequestDto, PermissionsUpdateRequestDto, PermissionsSchema,
		    };

		    #[tokio::test]
		    async fn test_create_permission_service() {
		        let app_state = crate::get_app_state().await;
		        let repo = imphnen_iam::PermissionsRepository::new(&app_state);

		        let permission_name = "test_permission_service".to_string();
		        let permission_request = PermissionsCreateRequestDto { name: permission_name.clone() };

		        let response = imphnen_iam::PermissionsService::create_role(&app_state, permission_request.clone()).await;
		        assert_eq!(response.status(), StatusCode::CREATED);

		        let created: PermissionsSchema = crate::common::response_helpers::parse_response(response, 1024).await;
		        assert_eq!(created.name, permission_name);

		        let created_permission = repo.query_permission_by_name(permission_name).await.unwrap();
		        let _ = repo.query_delete_permission(created_permission.id.id.to_raw()).await;
		    }

		    #[tokio::test]
		    async fn test_get_permission_by_id_service() {
		        let app_state = crate::get_app_state().await;
		        let repo = imphnen_iam::PermissionsRepository::new(&app_state);

		        let permission_name = "test_permission_by_id_service".to_string();
		        let permission = PermissionsSchema { name: permission_name.clone(), ..Default::default() };
		        let _ = repo.query_create_permission(permission.clone()).await;

		        let created_permission = repo.query_permission_by_name(permission_name).await.unwrap();
		        let permission_id = created_permission.id.id.to_raw();

		        let response = imphnen_iam::PermissionsService::get_permission_by_id(&app_state, permission_id.clone()).await;
		        assert_eq!(response.status(), StatusCode::OK);

		        let body: PermissionsSchema = crate::common::response_helpers::parse_response(response, 1024).await;
		        assert_eq!(body.id.id.to_raw(), permission_id);

		        let _ = repo.query_delete_permission(permission_id).await;
		    }

		    #[tokio::test]
		    async fn test_update_permission_service() {
		        let app_state = crate::get_app_state().await;
		        let repo = imphnen_iam::PermissionsRepository::new(&app_state);

		        let original_name = "test_permission_update_original_service".to_string();
		        let new_name = "test_permission_update_updated_service".to_string();
		        let permission = PermissionsSchema { name: original_name.clone(), ..Default::default() };
		        let _ = repo.query_create_permission(permission.clone()).await;

		        let created_permission = repo.query_permission_by_name(original_name).await.unwrap();
		        let permission_id = created_permission.id.id.to_raw();

		        let update_request = PermissionsUpdateRequestDto { name: Some(new_name.clone()) };
		        let response = imphnen_iam::PermissionsService::update_permission(&app_state, update_request, permission_id.clone()).await;
		        assert_eq!(response.status(), StatusCode::OK);

		        let body: PermissionsSchema = crate::common::response_helpers::parse_response(response, 1024).await;
		        assert_eq!(body.name, new_name);

		        let _ = repo.query_delete_permission(permission_id).await;
		    }

		    #[tokio::test]
		    async fn test_delete_permission_service() {
		        let app_state = crate::get_app_state().await;
		        let repo = imphnen_iam::PermissionsRepository::new(&app_state);

		        let permission_name = "test_permission_delete_service".to_string();
		        let permission = PermissionsSchema { name: permission_name.clone(), ..Default::default() };
		        let _ = repo.query_create_permission(permission.clone()).await;

		        let created_permission = repo.query_permission_by_name(permission_name).await.unwrap();
		        let permission_id = created_permission.id.id.to_raw();

		        let response = imphnen_iam::PermissionsService::delete_permission(&app_state, permission_id.clone()).await;
		        assert_eq!(response.status(), StatusCode::OK);

		        let msg: crate::MessageResponseDto = crate::common::response_helpers::parse_response(response, 1024).await;
		        assert!(msg.message.to_lowercase().contains("deleted") || msg.message.to_lowercase().contains("success"));

		        let exists_after = repo.query_permission_by_id(permission_id).await.is_ok();
		        assert!(!exists_after);
		    }

		    #[tokio::test]
		    async fn test_get_permission_by_id_service_not_found() {
		        let app_state = crate::get_app_state().await;
		        let non_existent_id = "non-existent-uuid-123456789".to_string();

		        let response = imphnen_iam::PermissionsService::get_permission_by_id(&app_state, non_existent_id).await;
		        assert_eq!(response.status(), StatusCode::NOT_FOUND);

		        let err: crate::MessageResponseDto = crate::common::response_helpers::parse_response(response, 1024).await;
		        assert!(err.message.to_lowercase().contains("not found") || err.message.to_lowercase().contains("permission not found"));
		    }

		    #[tokio::test]
		    async fn test_update_permission_service_not_found() {
		        let app_state = crate::get_app_state().await;
		        let non_existent_id = "non-existent-uuid-123456789".to_string();

		        let update_request = PermissionsUpdateRequestDto { name: Some("new_name".to_string()) };
		        let response = imphnen_iam::PermissionsService::update_permission(&app_state, update_request, non_existent_id).await;
		        assert_eq!(response.status(), StatusCode::NOT_FOUND);

		        let err: crate::MessageResponseDto = crate::common::response_helpers::parse_response(response, 1024).await;
		        assert!(err.message.to_lowercase().contains("not found") || err.message.to_lowercase().contains("permission not found"));
		    }

		    #[tokio::test]
		    async fn test_delete_permission_service_not_found() {
		        let app_state = crate::get_app_state().await;
		        let non_existent_id = "non-existent-uuid-123456789".to_string();

		        let response = imphnen_iam::PermissionsService::delete_permission(&app_state, non_existent_id).await;
		        assert_eq!(response.status(), StatusCode::NOT_FOUND);

		        let err: crate::MessageResponseDto = crate::common::response_helpers::parse_response(response, 1024).await;
		        assert!(err.message.to_lowercase().contains("not found") || err.message.to_lowercase().contains("permission not found"));
		    }

		    #[tokio::test]
		    async fn test_create_permission_service_duplicate_name() {
		        let app_state = crate::get_app_state().await;
		        let repo = imphnen_iam::PermissionsRepository::new(&app_state);

		        let permission_name = "test_permission_duplicate_service".to_string();
		        let permission_request = PermissionsCreateRequestDto { name: permission_name.clone() };

		        let response1 = imphnen_iam::PermissionsService::create_role(&app_state, permission_request.clone()).await;
		        assert_eq!(response1.status(), StatusCode::CREATED);

		        let response2 = imphnen_iam::PermissionsService::create_role(&app_state, permission_request).await;
		        assert_eq!(response2.status(), StatusCode::CONFLICT);

		        let error_response: crate::MessageResponseDto = crate::common::response_helpers::parse_response(response2, 1024).await;
		        assert_eq!(error_response.message, "Permission name already exists");

		        let created_permission = repo.query_permission_by_name("test_permission_duplicate_service".to_string()).await.unwrap();
		        let _ = repo.query_delete_permission(created_permission.id.id.to_raw()).await;
		    }

		    #[tokio::test]
		    async fn test_get_permission_list_service() {
		        let app_state = crate::get_app_state().await;
		        let repo = imphnen_iam::PermissionsRepository::new(&app_state);

		        let permission_name = "test_permission_list_service".to_string();
		        let permission = PermissionsSchema { name: permission_name.clone(), ..Default::default() };
		        let _ = repo.query_create_permission(permission.clone()).await;

		        let meta = imphnen_iam::MetaRequestDto { page: Some(1), limit: Some(10), ..Default::default() };
		        let response = imphnen_iam::PermissionsService::get_permission_list(&app_state, meta).await;
		        assert_eq!(response.status(), StatusCode::OK);

		        let created_permission = repo.query_permission_by_name(permission_name).await.unwrap();
		        let _ = repo.query_delete_permission(created_permission.id.id.to_raw()).await;
		    }

		    #[tokio::test]
		    async fn test_update_permission_service_duplicate_name() {
		        let app_state = crate::get_app_state().await;
		        let repo = imphnen_iam::PermissionsRepository::new(&app_state);

		        let permission_name1 = "test_permission_update_dup1_service".to_string();
		        let permission_name2 = "test_permission_update_dup2_service".to_string();
		        let permission1 = PermissionsSchema { name: permission_name1.clone(), ..Default::default() };
		        let permission2 = PermissionsSchema { name: permission_name2.clone(), ..Default::default() };
		        let _ = repo.query_create_permission(permission1.clone()).await;
		        let _ = repo.query_create_permission(permission2.clone()).await;

		        let created_permission1 = repo.query_permission_by_name(permission_name1.clone()).await.unwrap();
		        let permission_id1 = created_permission1.id.id.to_raw();

		        let update_request = PermissionsUpdateRequestDto { name: Some(permission_name2.clone()) };
		        let response = imphnen_iam::PermissionsService::update_permission(&app_state, update_request, permission_id1.clone()).await;
		        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

		        let error_response: crate::MessageResponseDto = crate::common::response_helpers::parse_response(response, 1024).await;
		        assert!(error_response.message.contains("not found") || error_response.message.contains("Permission not found"));

		        let _ = repo.query_delete_permission(permission_id1).await;
		        let created_permission2 = repo.query_permission_by_name(permission_name2).await.unwrap();
		        let _ = repo.query_delete_permission(created_permission2.id.id.to_raw()).await;
		    }

		    #[tokio::test]
		    async fn test_update_permission_service_no_changes() {
		        let app_state = crate::get_app_state().await;
		        let repo = imphnen_iam::PermissionsRepository::new(&app_state);

		        let permission_name = "test_permission_no_change_service".to_string();
		        let permission = PermissionsSchema { name: permission_name.clone(), ..Default::default() };
		        let _ = repo.query_create_permission(permission.clone()).await;

		        let created_permission = repo.query_permission_by_name(permission_name).await.unwrap();
		        let permission_id = created_permission.id.id.to_raw();

		        let update_request = PermissionsUpdateRequestDto { name: None };
		        let response = imphnen_iam::PermissionsService::update_permission(&app_state, update_request, permission_id.clone()).await;
		        assert_eq!(response.status(), StatusCode::OK);

		        let _ = repo.query_delete_permission(permission_id).await;
		    }
		}