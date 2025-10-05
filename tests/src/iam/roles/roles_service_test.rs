#[cfg(test)]
mod tests {
	use axum::http::StatusCode;
	use imphnen_entities::MessageResponseDto;
	use serde_json;
	use imphnen_iam::MetaRequestDto;
	use imphnen_iam::v1::roles::{RolesRepository, RolesRequestCreateDto, RolesRequestUpdateDto, roles_service::RolesService};

	#[tokio::test]
	async fn test_create_role_service() {
		let app_state = crate::get_app_state().await;
		let repo = RolesRepository::new(&app_state);

		// Test data
		let role_name = "test_role_service".to_string();
		let role_request = RolesRequestCreateDto {
			name: role_name.clone(),
			permissions: vec![], // Empty permissions for simplicity
		};

		// Create role through service
		let response = RolesService::create_role(
			&app_state,
			role_request.clone(),
		)
		.await;

		// Verify response
		assert_eq!(response.status(), StatusCode::CREATED);

		// Verify response body contains role data
		let created_role: imphnen_iam::v1::roles::roles_dto::RolesDetailItemDto =
			crate::common::response_helpers::parse_response_data(response, 1024).await;
		assert!(!created_role.id.is_empty(), "Created role must have non-empty id");
		assert_eq!(created_role.name, role_name, "Created role name must match request");

		// Verify role was created in database
		let db_role = repo
			.query_role_by_name(role_name.clone())
			.await
			.unwrap();
		assert_eq!(db_role.name, role_name);

		// Clean up
		let _ = repo.query_delete_role(db_role.id).await;
	}

	#[tokio::test]
	async fn test_get_role_by_id_service() {
		let app_state = crate::get_app_state().await;
		let repo = RolesRepository::new(&app_state);

		// Create test role
		let role_name = "test_role_by_id_service".to_string();
		let role_request = RolesRequestCreateDto {
			name: role_name.clone(),
			permissions: vec![],
		};
		let create_response = RolesService::create_role(&app_state, role_request).await;
		assert_eq!(create_response.status(), StatusCode::CREATED);

		// Get created role to get ID
		let created_role = repo
		    .query_role_by_name(role_name.clone())
		    .await
		    .unwrap();
		let role_id = created_role.id;

		// Get role by ID through service
		let response = RolesService::get_role_by_id(
			&app_state, role_id.clone(),
		)
		.await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);

		// Parse response and verify role data
		let v = crate::common::response_helpers::parse_response_value(response, 1024).await;
		let role_data = if let Some(inner) = v.get("data") {
			serde_json::from_value(inner.clone()).expect("Response 'data' must deserialize into RolesDetailItemDto")
		} else {
			serde_json::from_value(v).expect("Response must deserialize into RolesDetailItemDto")
		};
		let role: imphnen_iam::v1::roles::roles_dto::RolesDetailItemDto = role_data;
		
		assert!(!role.id.is_empty(), "Role must have non-empty id");
		assert_eq!(role.name, role_name, "Role name must match created role");

		// Clean up
		let _ = repo.query_delete_role(role_id).await;
	}

	#[tokio::test]
	async fn test_update_role_service() {
		let app_state = crate::get_app_state().await;
		let repo = RolesRepository::new(&app_state);

		// Create test role
		let original_name = "test_role_update_original_service".to_string();
		let new_name = "test_role_update_updated_service".to_string();

		let role_request = RolesRequestCreateDto {
			name: original_name.clone(),
			permissions: vec![],
		};
		let create_response = RolesService::create_role(&app_state, role_request).await;
		assert_eq!(create_response.status(), StatusCode::CREATED);

		// Get created role to get ID
		let created_role = repo
		    .query_role_by_name(original_name.clone())
		    .await
		    .unwrap();
		let role_id = created_role.id;

		// Prepare update request
		let update_request = RolesRequestUpdateDto {
			name: Some(new_name.clone()),
			permissions: None,
			overwrite: None,
		};

		// Update role through service
		let response = RolesService::update_role(
			&app_state, role_id.clone(), update_request,
		)
		.await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);

		let msg: MessageResponseDto =
			crate::common::response_helpers::parse_response(response, 1024).await;
		assert!(msg.message.to_lowercase().contains("updated") || msg.message.to_lowercase().contains("success"));

		// Verify role was updated in database
		let updated_role = repo
			.query_role_by_id(role_id.clone())
			.await
			.unwrap();
		assert_eq!(updated_role.name, new_name);

		// Clean up
		let _ = repo.query_delete_role(role_id).await;
	}

	#[tokio::test]
	async fn test_delete_role_service() {
		let app_state = crate::get_app_state().await;
		let repo = RolesRepository::new(&app_state);

		// Create test role
		let role_name = "test_role_delete_service".to_string();
		let role_request = RolesRequestCreateDto {
			name: role_name.clone(),
			permissions: vec![],
		};
		let create_response = RolesService::create_role(&app_state, role_request).await;
		assert_eq!(create_response.status(), StatusCode::CREATED);

		// Get created role to get ID
		let created_role = repo
		    .query_role_by_name(role_name.clone())
		    .await
		    .unwrap();
		let role_id = created_role.id;

		// Verify role exists before deletion
		let exists_before = repo.query_role_by_id(role_id.clone()).await.is_ok();
		assert!(exists_before);

		// Delete role through service
		let response = RolesService::delete_role(
			&app_state, role_id.clone(),
		)
		.await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);

		let msg: MessageResponseDto =
			crate::common::response_helpers::parse_response(response, 1024).await;
		assert!(msg.message.to_lowercase().contains("deleted") || msg.message.to_lowercase().contains("success"));

		// Verify role was deleted from database
		let exists_after = repo.query_role_by_id(role_id).await.is_ok();
		assert!(!exists_after);
	}

	#[tokio::test]
	async fn test_get_role_list_service() {
		let app_state = crate::get_app_state().await;
		let repo = RolesRepository::new(&app_state);

		// Create test role
		let role_name = "test_role_list_service".to_string();
		let role_request = RolesRequestCreateDto {
			name: role_name.clone(),
			permissions: vec![],
		};
		let create_response = RolesService::create_role(&app_state, role_request).await;
		assert_eq!(create_response.status(), StatusCode::CREATED);

		// Get role list through service
		let meta = MetaRequestDto {
			page: Some(1),
			per_page: Some(10),
			..Default::default()
		};
		let response = RolesService::get_role_list(
			&app_state, meta,
		)
		.await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);

		let v = crate::common::response_helpers::parse_response_value(response, 1024).await;
		if let Some(inner) = v.get("data") {
			let list: imphnen_entities::ResponseListSuccessDto<Vec<imphnen_iam::v1::roles::roles_dto::RolesListItemDto>> =
				serde_json::from_value(inner.clone()).unwrap_or(imphnen_entities::ResponseListSuccessDto { data: vec![], meta: None });
			if !list.data.is_empty() {
				assert!(!list.data[0].id.is_empty(), "Role list items must have non-empty id");
				assert!(!list.data[0].name.is_empty(), "Role list items must have non-empty name");
			}
		} else if v.is_array() {
			let arr: Vec<imphnen_iam::v1::roles::roles_dto::RolesListItemDto> = serde_json::from_value(v).unwrap_or_default();
			if !arr.is_empty() {
				assert!(!arr[0].id.is_empty(), "Role list items must have non-empty id");
				assert!(!arr[0].name.is_empty(), "Role list items must have non-empty name");
			}
		} else {
			// accept other object shapes
		}

		// Clean up
		let created_role = repo
			.query_role_by_name(role_name.clone())
			.await
			.unwrap();
		let _ = repo.query_delete_role(created_role.id).await;
	}

	#[tokio::test]
	async fn test_create_role_service_duplicate_name() {
		let app_state = crate::get_app_state().await;
		let repo = RolesRepository::new(&app_state);

		// Test data
		let role_name = "test_role_duplicate_service".to_string();
		let role_request = RolesRequestCreateDto {
			name: role_name.clone(),
			permissions: vec![],
		};

		// Create role first
		let response1 = RolesService::create_role(
			&app_state,
			role_request.clone(),
		)
		.await;
		assert_eq!(response1.status(), StatusCode::CREATED);

		// Try to create again with same name
		let response2 = RolesService::create_role(
			&app_state,
			role_request,
		)
		.await;

		// Verify response - should fail
		assert_eq!(response2.status(), StatusCode::CONFLICT);

		let error_response: MessageResponseDto =
			crate::common::response_helpers::parse_response(response2, 1024).await;
		assert_eq!(error_response.message, "Role name already exists");

		// Clean up
		let created_role = repo
		    .query_role_by_name(role_name.clone())
		    .await
		    .unwrap();
		let _ = repo.query_delete_role(created_role.id).await;
	}

	#[tokio::test]
	async fn test_update_role_service_duplicate_name() {
		let app_state = crate::get_app_state().await;
		let repo = RolesRepository::new(&app_state);

		// Create two test roles
		let role_name1 = "test_role_update_dup1_service".to_string();
		let role_name2 = "test_role_update_dup2_service".to_string();

		let role_request1 = RolesRequestCreateDto {
			name: role_name1.clone(),
			permissions: vec![],
		};
		let role_request2 = RolesRequestCreateDto {
			name: role_name2.clone(),
			permissions: vec![],
		};

		let create_response1 = RolesService::create_role(&app_state, role_request1).await;
		assert_eq!(create_response1.status(), StatusCode::CREATED);
		let create_response2 = RolesService::create_role(&app_state, role_request2).await;
		assert_eq!(create_response2.status(), StatusCode::CREATED);

		// Get created roles
		let created_role1 = repo
			.query_role_by_name(role_name1.clone())
			.await
			.unwrap();
		let role_id1 = created_role1.id;

		// Try to update role1 to have same name as role2
		let update_request = RolesRequestUpdateDto {
			name: Some(role_name2.clone()),
			permissions: None,
			overwrite: None,
		};

		let response = RolesService::update_role(
			&app_state, role_id1.clone(), update_request,
		)
		.await;

		// Verify response - should fail
		assert_eq!(response.status(), StatusCode::CONFLICT);

		let error_response: MessageResponseDto =
			crate::common::response_helpers::parse_response(response, 1024).await;
		assert_eq!(error_response.message, "Role name already exists");

		// Clean up
		let _ = repo.query_delete_role(role_id1).await;
		let created_role2 = repo
		    .query_role_by_name(role_name2.clone())
		    .await
		    .unwrap();
		let _ = repo.query_delete_role(created_role2.id).await;
	}

	#[tokio::test]
	async fn test_get_role_by_id_service_not_found() {
		let app_state = crate::get_app_state().await;

		// Use non-existent ID
		let non_existent_id = "non-existent-uuid-123456789".to_string();

		// Get non-existent role by ID through service
		let response = RolesService::get_role_by_id(
			&app_state, non_existent_id,
		)
		.await;

		// Verify not found response
		assert_eq!(response.status(), StatusCode::NOT_FOUND);

		let err: MessageResponseDto = crate::common::response_helpers::parse_response(response, 1024).await;
		assert!(err.message.to_lowercase().contains("not found") || err.message.to_lowercase().contains("role not found"));
	}

	#[tokio::test]
	async fn test_update_role_service_not_found() {
		let app_state = crate::get_app_state().await;

		// Use non-existent ID
		let non_existent_id = "non-existent-uuid-123456789".to_string();

		// Prepare update request
		let update_request = RolesRequestUpdateDto {
			name: Some("new_name".to_string()),
			permissions: None,
			overwrite: None,
		};

		// Update non-existent role through service
		let response = RolesService::update_role(
			&app_state, non_existent_id, update_request,
		)
		.await;

		// Verify not found response and message
		assert_eq!(response.status(), StatusCode::NOT_FOUND);

		let err: MessageResponseDto = crate::common::response_helpers::parse_response(response, 1024).await;
		assert!(err.message.to_lowercase().contains("not found") || err.message.to_lowercase().contains("role not found"));
	}

	#[tokio::test]
	async fn test_delete_role_service_not_found() {
		let app_state = crate::get_app_state().await;

		// Use non-existent ID
		let non_existent_id = "non-existent-uuid-123456789".to_string();

		// Delete non-existent role through service
		let response = RolesService::delete_role(
			&app_state, non_existent_id,
		)
		.await;

		// Verify not found response
		assert_eq!(response.status(), StatusCode::NOT_FOUND);

		let err: MessageResponseDto = crate::common::response_helpers::parse_response(response, 1024).await;
		assert!(err.message.to_lowercase().contains("not found") || err.message.to_lowercase().contains("role not found"));
	}
}