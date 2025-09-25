#[cfg(test)]
mod tests {
	use imphnen_iam::{
		PermissionsSchema, ResourceEnum,
	};
	use imphnen_utils::{make_thing_from_enum};
	use surrealdb::Uuid;
	use imphnen_entities::MetaRequestDto;

	#[tokio::test]
	async fn test_query_create_permission() {
		let app_state = crate::get_app_state().await;
		let repo = imphnen_iam::PermissionsRepository::new(&app_state);

		// Test data
		let permission_name = "test_permission_repo_create".to_string();
		let permission = PermissionsSchema {
			id: make_thing_from_enum(ResourceEnum::Permissions, &Uuid::new_v4().to_string()),
			name: permission_name.clone(),
			is_deleted: false,
			created_at: None,
			updated_at: None,
		};

		// Create permission
		let result = repo.query_create_permission(permission.clone()).await;
		assert!(result.is_ok(), "Failed to create permission: {:?}", result.err());

		// Verify permission was created
		let created_permission = repo
			.query_permission_by_name(permission_name.clone())
			.await
			.unwrap();
		assert_eq!(created_permission.name, permission_name);

		// Clean up
		let _ = repo.query_delete_permission(created_permission.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_query_permission_by_name() {
		let app_state = crate::get_app_state().await;
		let repo = imphnen_iam::PermissionsRepository::new(&app_state);

		// Test data
		let permission_name = "test_permission_repo_by_name".to_string();
		let permission = PermissionsSchema {
			id: make_thing_from_enum(ResourceEnum::Permissions, &Uuid::new_v4().to_string()),
			name: permission_name.clone(),
			is_deleted: false,
			created_at: None,
			updated_at: None,
		};

		// Create permission
		let create_result = repo.query_create_permission(permission.clone()).await;
		assert!(create_result.is_ok());

		// Query permission by name
		let result = repo.query_permission_by_name(permission_name.clone()).await;
		assert!(result.is_ok());
		let found_permission = result.unwrap();
		assert_eq!(found_permission.name, permission_name);

		// Query non-existent permission
		let non_existent_result = repo.query_permission_by_name("non_existent".to_string()).await;
		assert!(non_existent_result.is_err());
		assert!(non_existent_result.err().unwrap().to_string().contains("not found"));

		// Clean up
		let _ = repo.query_delete_permission(found_permission.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_query_permission_list() {
		let app_state = crate::get_app_state().await;
		let repo = imphnen_iam::PermissionsRepository::new(&app_state);

		// Create test permissions
		let permission_names = vec![
			"test_permission_list_1".to_string(),
			"test_permission_list_2".to_string(),
			"test_permission_list_3".to_string(),
		];

		for name in &permission_names {
			let permission = PermissionsSchema {
				id: make_thing_from_enum(ResourceEnum::Permissions, &Uuid::new_v4().to_string()),
				name: name.clone(),
				is_deleted: false,
				created_at: None,
				updated_at: None,
			};
			let _ = repo.query_create_permission(permission).await;
		}

		// Query permission list
		let meta = MetaRequestDto {
			page: Some(1),
			per_page: Some(10),
			search: None,
			filter: None,
			sort_by: None,
			order: None,
			filter_by: None,
		};
		let result = repo.query_permission_list(meta).await;
		assert!(result.is_ok());
		let permission_list = result.unwrap();
		assert_eq!(permission_list.data.len(), 3);

		// Clean up
		for name in permission_names {
			let permission = repo.query_permission_by_name(name).await.unwrap();
			let _ = repo.query_delete_permission(permission.id.id.to_raw()).await;
		}
	}

	#[tokio::test]
	async fn test_query_update_permission() {
		let app_state = crate::get_app_state().await;
		let repo = imphnen_iam::PermissionsRepository::new(&app_state);

		// Test data
		let original_name = "test_permission_update_original".to_string();
		let new_name = "test_permission_update_updated".to_string();
		
		let permission = PermissionsSchema {
			id: make_thing_from_enum(ResourceEnum::Permissions, &Uuid::new_v4().to_string()),
			name: original_name.clone(),
			is_deleted: false,
			created_at: None,
			updated_at: None,
		};

		// Create permission
		let create_result = repo.query_create_permission(permission.clone()).await;
		assert!(create_result.is_ok());

		// Get created permission
		let created_permission = repo.query_permission_by_name(original_name).await.unwrap();
		
		// Update permission
		let updated_permission = PermissionsSchema {
			id: created_permission.id.clone(),
			name: new_name.clone(),
			is_deleted: created_permission.is_deleted,
			created_at: created_permission.created_at,
			updated_at: created_permission.updated_at,
		};

		let update_result = repo.query_update_permission(updated_permission).await;
		assert!(update_result.is_ok());

		// Verify permission was updated
		let result = repo.query_permission_by_name(new_name.clone()).await;
		assert!(result.is_ok());
		let found_permission = result.unwrap();
		assert_eq!(found_permission.name, new_name);

		// Clean up
		let _ = repo.query_delete_permission(found_permission.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_query_delete_permission() {
		let app_state = crate::get_app_state().await;
		let repo = imphnen_iam::PermissionsRepository::new(&app_state);

		// Test data
		let permission_name = "test_permission_delete".to_string();
		let permission = PermissionsSchema {
			id: make_thing_from_enum(ResourceEnum::Permissions, &Uuid::new_v4().to_string()),
			name: permission_name.clone(),
			is_deleted: false,
			created_at: None,
			updated_at: None,
		};

		// Create permission
		let create_result = repo.query_create_permission(permission.clone()).await;
		assert!(create_result.is_ok());

		// Get created permission
		let created_permission = repo.query_permission_by_name(permission_name).await.unwrap();
		
		// Verify permission exists before deletion
		let exists_before = repo.query_permission_by_id(created_permission.id.id.to_raw()).await.is_ok();
		assert!(exists_before);

		// Delete permission
		let delete_result = repo.query_delete_permission(created_permission.id.id.to_raw()).await;
		assert!(delete_result.is_ok());

		// Verify permission was deleted
		let exists_after = repo.query_permission_by_id(created_permission.id.id.to_raw()).await.is_ok();
		assert!(!exists_after);
	}
}
