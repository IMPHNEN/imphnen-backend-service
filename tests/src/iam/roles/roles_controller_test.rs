#[cfg(test)]
mod tests {
	use crate::{generate_unique_email, get_role_id, UsersRepository};
	use axum::{http::StatusCode, response::Response};
	use imphnen_iam::{
		RolesCreateRequestDto, RolesUpdateRequestDto, RolesSchema, ResourceEnum,
	};
	use imphnen_utils::{make_thing_from_enum, ResourceEnum as UtilsResourceEnum};

	#[tokio::test]
	async fn test_create_role_controller() {
		let app_state = crate::get_app_state().await;
		let repo = imphnen_iam::RolesRepository::new(&app_state);

		// Test data
		let role_name = "test_role_controller".to_string();
		let role_request = RolesCreateRequestDto {
			name: role_name.clone(),
			description: Some("Test role for controller".to_string()),
		};

		// Create role through controller
		let response = imphnen_iam::RolesController::create_role(
			&app_state,
			role_request.clone(),
		)
		.await;

		// Verify response
		assert_eq!(response.status(), StatusCode::CREATED);

		// Verify response body contains role data
		let created_role: imphnen_iam::v1::roles::roles_dto::RolesDetailItemDto =
			crate::common::response_helpers::parse_response(response, 1024).await;
		
		// Validate all required fields in RolesDetailItemDto
		assert!(!created_role.id.is_empty(), "Created role must have non-empty id");
		assert_eq!(created_role.name, role_name, "Created role name must match request");
		assert_eq!(created_role.description, Some("Test role for controller".to_string()), "Created role description must match request");
		assert!(created_role.is_deleted == false, "Created role should not be marked as deleted");
		assert!(created_role.permissions.len() >= 0, "Created role must have permissions array");
		assert!(created_role.created_at.is_some(), "Created role must have created_at timestamp");
		assert!(created_role.updated_at.is_some(), "Created role must have updated_at timestamp");

		// Verify role was created in database
		let db_role = repo
			.query_role_by_name(role_name)
			.await
			.unwrap();
		assert_eq!(db_role.name, role_name);

		// Clean up
		let _ = repo.query_delete_role(db_role.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_create_role_controller_duplicate() {
		let app_state = crate::get_app_state().await;
		let repo = imphnen_iam::RolesRepository::new(&app_state);

		// Test data
		let role_name = "test_role_duplicate".to_string();
		let role_request = RolesCreateRequestDto {
			name: role_name.clone(),
			description: Some("Test role for duplicate check".to_string()),
		};

		// Create role first time
		let response1 = imphnen_iam::RolesController::create_role(
			&app_state,
			role_request.clone(),
		)
		.await;
		assert_eq!(response1.status(), StatusCode::CREATED);

		// Try to create same role again
		let response2 = imphnen_iam::RolesController::create_role(
			&app_state,
			role_request,
		)
		.await;

		// Verify conflict response
		assert_eq!(response2.status(), StatusCode::CONFLICT);

		let err: imphnen_entities::MessageResponseDto =
			crate::common::response_helpers::parse_response(response2, 1024).await;
		assert!(err.message.to_lowercase().contains("already exists") || err.message.to_lowercase().contains("duplicate"));

		// Clean up
		let created_role = repo
			.query_role_by_name(role_name)
			.await
			.unwrap();
		let _ = repo.query_delete_role(created_role.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_get_role_list_controller() {
		let app_state = crate::get_app_state().await;
		let repo = imphnen_iam::RolesRepository::new(&app_state);

		// Create test roles
		let role_names = vec![
			"test_role_list_1".to_string(),
			"test_role_list_2".to_string(),
			"test_role_list_3".to_string(),
		];

		for name in &role_names {
			let role = RolesSchema {
				name: name.clone(),
				description: Some(format!("Description for {name}")),
				..Default::default()
			};
			let _ = repo.query_create_role(role).await;
		}

		// Get role list through controller
		let response = imphnen_iam::RolesController::get_role_list(
			&app_state,
			crate::get_meta_request_dto(1, 10),
		)
		.await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);

	let v = crate::common::response_helpers::parse_response_value(response, 2048).await;
		// Expect wrapped { data: [...] } or raw array. Normalize to array and check created roles are present
		let list_val = if let Some(d) = v.get("data") { d.clone() } else { v };
		let arr = list_val.as_array().expect("role list should be an array");
		
		// Verify all items have required fields in RolesListItemDto
		for item in arr.iter() {
			assert!(item.get("id").is_some(), "Role list items must have id");
			assert!(item.get("name").is_some(), "Role list items must have name");
			let name = item.get("name").and_then(|n| n.as_str()).expect("Role name must be string");
			assert!(!name.is_empty(), "Role name must not be empty");
			assert!(item.get("permissions_count").is_some(), "Role list items must have permissions_count");
			assert!(item.get("created_at").is_some(), "Role list items must have created_at timestamp");
			assert!(item.get("updated_at").is_some(), "Role list items must have updated_at timestamp");
		}

		let names: Vec<String> = arr.iter().filter_map(|it| it.get("name").and_then(|n| n.as_str()).map(|s| s.to_string())).collect();
		for name in ["test_role_list_1", "test_role_list_2", "test_role_list_3"].iter() {
			assert!(names.contains(&name.to_string()), "expected role {} in list", name);
		}

		// Clean up
		for name in role_names {
			let role = repo.query_role_by_name(name).await.unwrap();
			let _ = repo.query_delete_role(role.id.id.to_raw()).await;
		}
	}

	#[tokio::test]
	async fn test_get_role_by_id_controller() {
		let app_state = crate::get_app_state().await;
		let repo = imphnen_iam::RolesRepository::new(&app_state);

		// Create test role
		let role_name = "test_role_by_id".to_string();
		let role = RolesSchema {
			name: role_name.clone(),
			description: Some("Test role for by ID test".to_string()),
			..Default::default()
		};
		let create_result = repo.query_create_role(role.clone()).await;
		assert!(create_result.is_ok());

		// Get created role to get ID
		let created_role = repo
			.query_role_by_name(role_name)
			.await
			.unwrap();
		let role_id = created_role.id.id.to_raw();

		// Get role by ID through controller
		let response = imphnen_iam::RolesController::get_role_by_id(
			&app_state, role_id.clone(),
		)
		.await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);

		// Parse and verify role data
		let role: imphnen_iam::v1::roles::roles_dto::RolesDetailItemDto =
			crate::common::response_helpers::parse_response(response, 1024).await;
		
		// Validate all required fields in RolesDetailItemDto
		assert!(!role.id.is_empty(), "Role must have non-empty id");
		assert_eq!(role.name, role_name, "Role name must match created role");
		assert_eq!(role.description, Some("Test role for by ID test".to_string()), "Role description must match created role");
		assert!(role.is_deleted == false, "Role should not be marked as deleted");
		assert!(role.permissions.len() >= 0, "Role must have permissions array");
		assert!(role.created_at.is_some(), "Role must have created_at timestamp");
		assert!(role.updated_at.is_some(), "Role must have updated_at timestamp");

		// Clean up
		let _ = repo.query_delete_role(role_id).await;
	}
}
#[tokio::test]
async fn test_get_role_by_id_controller_not_found() {
		let app_state = crate::get_app_state().await;

		// Use non-existent ID
		let non_existent_id = "non-existent-uuid-123456789".to_string();

		// Get non-existent role by ID through controller
		let response = imphnen_iam::RolesController::get_role_by_id(
			&app_state, non_existent_id,
		)
		.await;

		// Verify not found response
		assert_eq!(response.status(), StatusCode::NOT_FOUND);

		let err: imphnen_entities::MessageResponseDto =
			crate::common::response_helpers::parse_response(response, 1024).await;
		assert!(err.message.to_lowercase().contains("not found"));
	}

	#[tokio::test]
	async fn test_update_role_controller() {
		let app_state = crate::get_app_state().await;
		let repo = imphnen_iam::RolesRepository::new(&app_state);

		// Create test role
		let original_name = "test_role_update_original_controller".to_string();
		let new_name = "test_role_update_updated_controller".to_string();
		
		let role = RolesSchema {
			name: original_name.clone(),
			description: Some("Original description for controller test".to_string()),
			..Default::default()
		};
		let create_result = repo.query_create_role(role.clone()).await;
		assert!(create_result.is_ok());

		// Get created role to get ID
		let created_role = repo
			.query_role_by_name(original_name)
			.await
			.unwrap();
		let role_id = created_role.id.id.to_raw();

		// Prepare update request
		let update_request = RolesUpdateRequestDto {
			name: Some(new_name.clone()),
			description: Some("Updated description for controller test".to_string()),
		};

		// Update role through controller
		let response = imphnen_iam::RolesController::update_role(
			&app_state, update_request, role_id.clone(),
		)
		.await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);

		let msg: imphnen_entities::MessageResponseDto =
			crate::common::response_helpers::parse_response(response, 1024).await;
		assert!(msg.message.to_lowercase().contains("updated") || msg.message.to_lowercase().contains("success"));

		// Verify role was updated in database
		let updated_role = repo
			.query_role_by_id(role_id.clone())
			.await
			.unwrap();
		assert_eq!(updated_role.name, new_name);
		assert_eq!(updated_role.description, Some("Updated description for controller test".to_string()));

		// Clean up
		let _ = repo.query_delete_role(role_id).await;
	}

	#[tokio::test]
	async fn test_update_role_controller_not_found() {
		let app_state = crate::get_app_state().await;

		// Use non-existent ID
		let non_existent_id = "non-existent-uuid-123456789".to_string();

		// Prepare update request
		let update_request = RolesUpdateRequestDto {
			name: Some("new_name".to_string()),
			description: Some("new description".to_string()),
		};

		// Update non-existent role through controller
		let response = imphnen_iam::RolesController::update_role(
			&app_state, update_request, non_existent_id,
		)
		.await;

		// Verify not found response
		assert_eq!(response.status(), StatusCode::NOT_FOUND);

		let err: imphnen_entities::MessageResponseDto =
			crate::common::response_helpers::parse_response(response, 1024).await;
		assert!(err.message.to_lowercase().contains("not found"));
	}

	#[tokio::test]
	async fn test_delete_role_controller() {
		let app_state = crate::get_app_state().await;
		let repo = imphnen_iam::RolesRepository::new(&app_state);

		// Create test role
		let role_name = "test_role_delete_controller".to_string();
		let role = RolesSchema {
			name: role_name.clone(),
			description: Some("Test role for delete test in controller".to_string()),
			..Default::default()
		};
		let create_result = repo.query_create_role(role.clone()).await;
		assert!(create_result.is_ok());

		// Get created role to get ID
		let created_role = repo
			.query_role_by_name(role_name)
			.await
			.unwrap();
		let role_id = created_role.id.id.to_raw();

		// Verify role exists before deletion
		let exists_before = repo.query_role_by_id(role_id.clone()).await.is_ok();
		assert!(exists_before);

		// Delete role through controller
		let response = imphnen_iam::RolesController::delete_role(
			&app_state, role_id.clone(),
		)
		.await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);

		let msg: imphnen_entities::MessageResponseDto =
			crate::common::response_helpers::parse_response(response, 1024).await;
		assert!(msg.message.to_lowercase().contains("deleted") || msg.message.to_lowercase().contains("success"));

		// Verify role was deleted from database
		let exists_after = repo.query_role_by_id(role_id).await.is_ok();
		assert!(!exists_after);
	}

	#[tokio::test]
	async fn test_delete_role_controller_not_found() {
		let app_state = crate::get_app_state().await;

		// Use non-existent ID
		let non_existent_id = "non-existent-uuid-123456789".to_string();

		// Delete non-existent role through controller
		let response = imphnen_iam::RolesController::delete_role(
			&app_state, non_existent_id,
		)
		.await;

		// Verify not found response
		assert_eq!(response.status(), StatusCode::NOT_FOUND);

		let err: imphnen_entities::MessageResponseDto =
			crate::common::response_helpers::parse_response(response, 1024).await;
		assert!(err.message.to_lowercase().contains("not found"));
	}
}