#[cfg(test)]
mod tests {
	use imphnen_iam::{
		RolesRequestCreateDto, RolesRequestUpdateDto,
	};
	use imphnen_entities::MetaRequestDto;

	#[tokio::test]
	async fn test_query_create_role() {
	    let app_state = crate::get_app_state().await;
	    let repo = imphnen_iam::RolesRepository::new(&app_state);

		// Test data
		let role_name = "test_role_repo_create".to_string();
		let role = RolesRequestCreateDto {
			name: role_name.clone(),
			permissions: vec![],
		};

		// Create role
		let result = repo.query_create_role(role.clone()).await;
		assert!(result.is_ok(), "Failed to create role: {:?}", result.err());

		// Verify role was created
		let created_role = repo
			.query_role_by_name(role_name.clone())
			.await
			.unwrap();
		assert_eq!(created_role.name, role_name);

		// Clean up
		let _ = repo.query_delete_role(created_role.id).await;
	}

	#[tokio::test]
	async fn test_query_role_by_name() {
	    let app_state = crate::get_app_state().await;
	    let repo = imphnen_iam::RolesRepository::new(&app_state);

		// Test data
		let role_name = "test_role_repo_by_name".to_string();
		let role = RolesRequestCreateDto {
			name: role_name.clone(),
			permissions: vec![],
		};

		// Create role
		let create_result = repo.query_create_role(role.clone()).await;
		assert!(create_result.is_ok());

		// Query role by name
		let result = repo.query_role_by_name(role_name.clone()).await;
		assert!(result.is_ok());
		let found_role = result.unwrap();
		assert_eq!(found_role.name, role_name);

		// Query non-existent role
		let non_existent_result = repo.query_role_by_name("non_existent".to_string()).await;
		assert!(non_existent_result.is_err());
		assert!(non_existent_result.err().unwrap().to_string().contains("not found"));

		// Clean up
		let _ = repo.query_delete_role(found_role.id).await;
	}

	#[tokio::test]
	async fn test_query_role_list() {
	    let app_state = crate::get_app_state().await;
	    let repo = imphnen_iam::RolesRepository::new(&app_state);

		// Create test roles
		let role_names = vec![
			"test_role_list_1".to_string(),
			"test_role_list_2".to_string(),
			"test_role_list_3".to_string(),
		];

		for name in &role_names {
			let role = RolesRequestCreateDto {
				name: name.clone(),
				permissions: vec![],
			};
			let _ = repo.query_create_role(role).await;
		}

		// Query role list
		let meta = MetaRequestDto {
			page: Some(1),
			per_page: Some(10),
			search: None,
			filter: None,
			sort_by: None,
			order: None,
			filter_by: None,
		};
		let result = repo.query_role_list(meta).await;
		assert!(result.is_ok());
		let role_list = result.unwrap();
		assert_eq!(role_list.data.len(), 3);

		// Clean up
		for name in role_names {
			let role = repo.query_role_by_name(name).await.unwrap();
			let _ = repo.query_delete_role(role.id).await;
		}
	}

	#[tokio::test]
	async fn test_query_update_role() {
	    let app_state = crate::get_app_state().await;
	    let repo = imphnen_iam::RolesRepository::new(&app_state);

		// Test data
		let original_name = "test_role_update_original".to_string();
		let new_name = "test_role_update_updated".to_string();
		
		let role = RolesRequestCreateDto {
			name: original_name.clone(),
			permissions: vec![],
		};

		// Create role
		let create_result = repo.query_create_role(role.clone()).await;
		assert!(create_result.is_ok());

		// Get created role
		let created_role = repo.query_role_by_name(original_name.clone()).await.unwrap();
		
		// Update role
		let updated_role = RolesRequestUpdateDto {
			name: Some(new_name.clone()),
			permissions: None,
			overwrite: None,
		};

		let update_result = repo.query_update_role(created_role.id.clone(), updated_role).await;
		assert!(update_result.is_ok());

		// Verify role was updated
		let result = repo.query_role_by_name(new_name.clone()).await;
		assert!(result.is_ok());
		let found_role = result.unwrap();
		assert_eq!(found_role.name, new_name);

		// Clean up
		let _ = repo.query_delete_role(found_role.id).await;
	}

	#[tokio::test]
	async fn test_query_delete_role() {
	    let app_state = crate::get_app_state().await;
	    let repo = imphnen_iam::RolesRepository::new(&app_state);

		// Test data
		let role_name = "test_role_delete".to_string();
		let role = RolesRequestCreateDto {
			name: role_name.clone(),
			permissions: vec![],
		};

		// Create role
		let create_result = repo.query_create_role(role.clone()).await;
		assert!(create_result.is_ok());

		// Get created role
		let created_role = repo.query_role_by_name(role_name.clone()).await.unwrap();
		
		// Verify role exists before deletion
		let role_id = created_role.id.clone();
		let exists_before = repo.query_role_by_id(role_id.clone()).await.is_ok();
		assert!(exists_before);

		// Delete role
		let delete_result = repo.query_delete_role(role_id.clone()).await;
		assert!(delete_result.is_ok());

		// Verify role was deleted
		let exists_after = repo.query_role_by_id(role_id.clone()).await.is_ok();
		assert!(!exists_after);
	}
}
