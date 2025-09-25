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

		// Verify role was created in database
		let created_role = repo
			.query_role_by_name(role_name)
			.await
			.unwrap();
		assert_eq!(created_role.name, role_name);

		// Clean up
		let _ = repo.query_delete_role(created_role.id.id.to_raw()).await;
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
				description: Some(format!("Description for {}", name)),
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
	}
}