#[cfg(test)]
mod tests {
	use crate::{get_meta_request_dto, UsersRepository};
	use imphnen_cms::{
		v1::landing::testimonials::{
			testimonials_repository::TestimonialsRepository,
			testimonials_schema::TestimonialsSchema,
		},
	};
	use imphnen_entities::UsersSchema;
	use imphnen_utils::make_thing_from_enum;

	#[tokio::test]
	async fn test_query_testimonial_list() {
		let app_state = crate::get_app_state().await;
		let repo = TestimonialsRepository::new(&app_state);

		// Create test users and testimonials
		let num_testimonials = 5;
		let testimonial_contents = vec![
			"Testimonial content 1".to_string(),
			"Testimonial content 2".to_string(),
			"Testimonial content 3".to_string(),
			"Testimonial content 4".to_string(),
			"Testimonial content 5".to_string(),
		];

		for (i, content) in testimonial_contents.iter().enumerate() {
			let user = UsersSchema {
				id: make_thing_from_enum("users", &uuid::Uuid::new_v4().to_string()),
				fullname: format!("Test User {}", i + 1),
				email: format!("testuser{}@example.com", i + 1),
				..Default::default()
			};
			let _ = UsersRepository::new(&app_state).query_create_user(user.clone()).await;

			let testimonial = TestimonialsSchema {
				id: make_thing_from_enum("testimonials", &uuid::Uuid::new_v4().to_string()),
				user: user.id,
				role: format!("Role {}", i + 1),
				content: content.clone(),
				created_at: chrono::Utc::now().to_rfc3339(),
				updated_at: chrono::Utc::now().to_rfc3339(),
				is_deleted: false,
			};
			let _ = repo.query_create_testimonial(testimonial).await;
		}

		// Test with pagination
		let result = repo.query_testimonial_list(get_meta_request_dto(1, 10)).await;
		assert!(result.is_ok());
		let response = result.unwrap();
		assert_eq!(response.data.len(), num_testimonials as usize);

		// Test with smaller page size
		let result = repo.query_testimonial_list(get_meta_request_dto(1, 2)).await;
		assert!(result.is_ok());
		let response = result.unwrap();
		assert_eq!(response.data.len(), 2);

		// Clean up
		for content in testimonial_contents {
			let user = UsersRepository::new(&app_state)
				.query_user_by_email(format!("testuser{}@example.com", content.chars().take(8).collect::<String>()))
				.await
				.unwrap();
			let _ = UsersRepository::new(&app_state).query_delete_user(user.id.id.to_raw()).await;
		}
	}

	#[tokio::test]
	async fn test_query_testimonial_by_id_found() {
		let app_state = crate::get_app_state().await;
		let repo = TestimonialsRepository::new(&app_state);

		// Create test user
		let user = UsersSchema {
			id: make_thing_from_enum("users", &uuid::Uuid::new_v4().to_string()),
			fullname: "Test User".to_string(),
			email: "testuser@example.com".to_string(),
			..Default::default()
		};
		let _ = UsersRepository::new(&app_state).query_create_user(user.clone()).await;

		// Create test testimonial
		let testimonial_content = "Test testimonial content for by ID test".to_string();
		let testimonial = TestimonialsSchema {
			id: make_thing_from_enum("testimonials", &uuid::Uuid::new_v4().to_string()),
			user: user.id,
			role: "Mentor".to_string(),
			content: testimonial_content.clone(),
			created_at: chrono::Utc::now().to_rfc3339(),
			updated_at: chrono::Utc::now().to_rfc3339(),
			is_deleted: false,
		};
		let create_result = repo.query_create_testimonial(testimonial.clone()).await;
		assert!(create_result.is_ok());

		// Get created testimonial to get ID
		let created_testimonial = repo
			.query_testimonial_by_id(testimonial.id.id.to_raw())
			.await
			.unwrap();
		let testimonial_id = created_testimonial.id.id.to_raw();

		// Query testimonial by ID
		let result = repo.query_testimonial_by_id(testimonial_id.clone()).await;
		assert!(result.is_ok());
		let found_testimonial = result.unwrap();
		assert_eq!(found_testimonial.content, testimonial_content);
		assert_eq!(found_testimonial.role, "Mentor");
		assert!(!found_testimonial.is_deleted);

		// Clean up
		let _ = repo.query_delete_testimonial(testimonial_id).await;
		let _ = UsersRepository::new(&app_state).query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_query_testimonial_by_id_not_found() {
		let app_state = crate::get_app_state().await;
		let repo = TestimonialsRepository::new(&app_state);

		// Use non-existent ID
		let non_existent_id = "non-existent-uuid-123456789".to_string();

		// Query non-existent testimonial by ID
		let result = repo.query_testimonial_by_id(non_existent_id).await;
		assert!(result.is_err());
		assert_eq!(result.unwrap_err().to_string(), "Testimonial not found");
	}

	#[tokio::test]
	async fn test_query_testimonial_by_id_deleted() {
		let app_state = crate::get_app_state().await;
		let repo = TestimonialsRepository::new(&app_state);

		// Create test user
		let user = UsersSchema {
			id: make_thing_from_enum("users", &uuid::Uuid::new_v4().to_string()),
			fullname: "Test User".to_string(),
			email: "testuser@example.com".to_string(),
			..Default::default()
		};
		let _ = UsersRepository::new(&app_state).query_create_user(user.clone()).await;

		// Create test testimonial
		let testimonial = TestimonialsSchema {
			id: make_thing_from_enum("testimonials", &uuid::Uuid::new_v4().to_string()),
			user: user.id,
			role: "Mentor".to_string(),
			content: "Test testimonial content for deleted test".to_string(),
			created_at: chrono::Utc::now().to_rfc3339(),
			updated_at: chrono::Utc::now().to_rfc3339(),
			is_deleted: false,
		};
		let create_result = repo.query_create_testimonial(testimonial.clone()).await;
		assert!(create_result.is_ok());

		// Get created testimonial to get ID
		let created_testimonial = repo
			.query_testimonial_by_id(testimonial.id.id.to_raw())
			.await
			.unwrap();
		let testimonial_id = created_testimonial.id.id.to_raw();

		// Soft delete the testimonial
		let _ = repo.query_delete_testimonial(testimonial_id.clone()).await;

		// Try to query deleted testimonial by ID
		let result = repo.query_testimonial_by_id(testimonial_id).await;
		assert!(result.is_err());
		assert_eq!(result.unwrap_err().to_string(), "Testimonial not found");

		// Clean up
		let _ = UsersRepository::new(&app_state).query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_query_create_testimonial() {
		let app_state = crate::get_app_state().await;
		let repo = TestimonialsRepository::new(&app_state);

		// Create test user
		let user = UsersSchema {
			id: make_thing_from_enum("users", &uuid::Uuid::new_v4().to_string()),
			fullname: "Test User".to_string(),
			email: "testuser@example.com".to_string(),
			..Default::default()
		};
		let _ = UsersRepository::new(&app_state).query_create_user(user.clone()).await;

		// Create test testimonial data
		let testimonial = TestimonialsSchema {
			id: make_thing_from_enum("testimonials", &uuid::Uuid::new_v4().to_string()),
			user: user.id,
			role: "Mentor".to_string(),
			content: "Test testimonial content for create test".to_string(),
			created_at: chrono::Utc::now().to_rfc3339(),
			updated_at: chrono::Utc::now().to_rfc3339(),
			is_deleted: false,
		};

		// Create testimonial
		let result = repo.query_create_testimonial(testimonial.clone()).await;
		assert!(result.is_ok());
		let created_testimonial = result.unwrap();
		assert_eq!(created_testimonial.content, testimonial.content);
		assert_eq!(created_testimonial.role, testimonial.role);
		assert!(!created_testimonial.is_deleted);

		// Verify it was created in database
		let found_testimonial = repo.query_testimonial_by_id(created_testimonial.id.id.to_raw()).await;
		assert!(found_testimonial.is_ok());
		assert_eq!(found_testimonial.unwrap().content, testimonial.content);

		// Clean up
		let _ = repo.query_delete_testimonial(created_testimonial.id.id.to_raw()).await;
		let _ = UsersRepository::new(&app_state).query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_query_update_testimonial() {
		let app_state = crate::get_app_state().await;
		let repo = TestimonialsRepository::new(&app_state);

		// Create test user
		let user = UsersSchema {
			id: make_thing_from_enum("users", &uuid::Uuid::new_v4().to_string()),
			fullname: "Test User".to_string(),
			email: "testuser@example.com".to_string(),
			..Default::default()
		};
		let _ = UsersRepository::new(&app_state).query_create_user(user.clone()).await;

		// Create test testimonial
		let original_content = "Original testimonial content for update test".to_string();
		let new_content = "Updated testimonial content for update test".to_string();

		let testimonial = TestimonialsSchema {
			id: make_thing_from_enum("testimonials", &uuid::Uuid::new_v4().to_string()),
			user: user.id,
			role: "Mentor".to_string(),
			content: original_content.clone(),
			created_at: chrono::Utc::now().to_rfc3339(),
			updated_at: chrono::Utc::now().to_rfc3339(),
			is_deleted: false,
		};
		let create_result = repo.query_create_testimonial(testimonial.clone()).await;
		assert!(create_result.is_ok());

		// Get created testimonial to get ID
		let created_testimonial = repo
			.query_testimonial_by_id(testimonial.id.id.to_raw())
			.await
			.unwrap();
		let testimonial_id = created_testimonial.id.id.to_raw();

		// Prepare updated testimonial
		let updated_testimonial = TestimonialsSchema {
			id: created_testimonial.id,
			user: created_testimonial.user,
			role: "Updated Mentor".to_string(),
			content: new_content.clone(),
			created_at: created_testimonial.created_at,
			updated_at: chrono::Utc::now().to_rfc3339(),
			is_deleted: false,
		};

		// Update testimonial
		let result = repo.query_update_testimonial(updated_testimonial).await;
		assert!(result.is_ok());
		assert_eq!(result.unwrap(), "Success update testimonial");

		// Verify it was updated in database
		let found_testimonial = repo.query_testimonial_by_id(testimonial_id).await;
		assert!(found_testimonial.is_ok());
		let updated = found_testimonial.unwrap();
		assert_eq!(updated.content, new_content);
		assert_eq!(updated.role, "Updated Mentor");
		assert!(!updated.is_deleted);

		// Clean up
		let _ = repo.query_delete_testimonial(testimonial_id).await;
		let _ = UsersRepository::new(&app_state).query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_query_update_testimonial_not_found() {
		let app_state = crate::get_app_state().await;
		let repo = TestimonialsRepository::new(&app_state);

		// Create test user
		let user = UsersSchema {
			id: make_thing_from_enum("users", &uuid::Uuid::new_v4().to_string()),
			fullname: "Test User".to_string(),
			email: "testuser@example.com".to_string(),
			..Default::default()
		};
		let _ = UsersRepository::new(&app_state).query_create_user(user.clone()).await;

		// Create non-existent testimonial ID
		let non_existent_id = "non-existent-uuid-123456789".to_string();

		// Prepare updated testimonial with non-existent ID
		let updated_testimonial = TestimonialsSchema {
			id: make_thing_from_enum("testimonials", &non_existent_id),
			user: user.id,
			role: "Updated Mentor".to_string(),
			content: "Updated content".to_string(),
			created_at: chrono::Utc::now().to_rfc3339(),
			updated_at: chrono::Utc::now().to_rfc3339(),
			is_deleted: false,
		};

		// Try to update non-existent testimonial
		let result = repo.query_update_testimonial(updated_testimonial).await;
		assert!(result.is_err());
		assert_eq!(result.unwrap_err().to_string(), "Testimonial not found");

		// Clean up
		let _ = UsersRepository::new(&app_state).query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_query_update_testimonial_deleted() {
		let app_state = crate::get_app_state().await;
		let repo = TestimonialsRepository::new(&app_state);

		// Create test user
		let user = UsersSchema {
			id: make_thing_from_enum("users", &uuid::Uuid::new_v4().to_string()),
			fullname: "Test User".to_string(),
			email: "testuser@example.com".to_string(),
			..Default::default()
		};
		let _ = UsersRepository::new(&app_state).query_create_user(user.clone()).await;

		// Create test testimonial
		let testimonial = TestimonialsSchema {
			id: make_thing_from_enum("testimonials", &uuid::Uuid::new_v4().to_string()),
			user: user.id,
			role: "Mentor".to_string(),
			content: "Test testimonial content for deleted update test".to_string(),
			created_at: chrono::Utc::now().to_rfc3339(),
			updated_at: chrono::Utc::now().to_rfc3339(),
			is_deleted: false,
		};
		let create_result = repo.query_create_testimonial(testimonial.clone()).await;
		assert!(create_result.is_ok());

		// Get created testimonial to get ID
		let created_testimonial = repo
			.query_testimonial_by_id(testimonial.id.id.to_raw())
			.await
			.unwrap();
		let testimonial_id = created_testimonial.id.id.to_raw();

		// Soft delete the testimonial
		let _ = repo.query_delete_testimonial(testimonial_id.clone()).await;

		// Prepare updated testimonial
		let updated_testimonial = TestimonialsSchema {
			id: created_testimonial.id,
			user: created_testimonial.user,
			role: "Updated Mentor".to_string(),
			content: "Updated content".to_string(),
			created_at: created_testimonial.created_at,
			updated_at: chrono::Utc::now().to_rfc3339(),
			is_deleted: false,
		};

		// Try to update deleted testimonial
		let result = repo.query_update_testimonial(updated_testimonial).await;
		assert!(result.is_err());
		assert_eq!(result.unwrap_err().to_string(), "Testimonial already deleted");

		// Clean up
		let _ = UsersRepository::new(&app_state).query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_query_delete_testimonial() {
		let app_state = crate::get_app_state().await;
		let repo = TestimonialsRepository::new(&app_state);

		// Create test user
		let user = UsersSchema {
			id: make_thing_from_enum("users", &uuid::Uuid::new_v4().to_string()),
			fullname: "Test User".to_string(),
			email: "testuser@example.com".to_string(),
			..Default::default()
		};
		let _ = UsersRepository::new(&app_state).query_create_user(user.clone()).await;

		// Create test testimonial
		let testimonial = TestimonialsSchema {
			id: make_thing_from_enum("testimonials", &uuid::Uuid::new_v4().to_string()),
			user: user.id,
			role: "Mentor".to_string(),
			content: "Test testimonial content for delete test".to_string(),
			created_at: chrono::Utc::now().to_rfc3339(),
			updated_at: chrono::Utc::now().to_rfc3339(),
			is_deleted: false,
		};
		let create_result = repo.query_create_testimonial(testimonial.clone()).await;
		assert!(create_result.is_ok());

		// Get created testimonial to get ID
		let created_testimonial = repo
			.query_testimonial_by_id(testimonial.id.id.to_raw())
			.await
			.unwrap();
		let testimonial_id = created_testimonial.id.id.to_raw();

		// Verify testimonial exists before deletion
		let exists_before = repo.query_testimonial_by_id(testimonial_id.clone()).await.is_ok();
		assert!(exists_before);

		// Delete testimonial
		let result = repo.query_delete_testimonial(testimonial_id.clone()).await;
		assert!(result.is_ok());
		assert_eq!(result.unwrap(), "Success delete testimonial");

		// Verify testimonial was soft-deleted from database
		let deleted_testimonial = repo.query_testimonial_by_id(testimonial_id.clone()).await;
		assert!(deleted_testimonial.is_err());
		assert_eq!(deleted_testimonial.unwrap_err().to_string(), "Testimonial not found");

		// Clean up - no need since it's already soft-deleted
		let _ = UsersRepository::new(&app_state).query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_query_delete_testimonial_not_found() {
		let app_state = crate::get_app_state().await;
		let repo = TestimonialsRepository::new(&app_state);

		// Use non-existent ID
		let non_existent_id = "non-existent-uuid-123456789".to_string();

		// Try to delete non-existent testimonial
		let result = repo.query_delete_testimonial(non_existent_id).await;
		assert!(result.is_err());
		assert_eq!(result.unwrap_err().to_string(), "Testimonial not found");
	}

	#[tokio::test]
	async fn test_query_delete_testimonial_already_deleted() {
		let app_state = crate::get_app_state().await;
		let repo = TestimonialsRepository::new(&app_state);

		// Create test user
		let user = UsersSchema {
			id: make_thing_from_enum("users", &uuid::Uuid::new_v4().to_string()),
			fullname: "Test User".to_string(),
			email: "testuser@example.com".to_string(),
			..Default::default()
		};
		let _ = UsersRepository::new(&app_state).query_create_user(user.clone()).await;

		// Create test testimonial
		let testimonial = TestimonialsSchema {
			id: make_thing_from_enum("testimonials", &uuid::Uuid::new_v4().to_string()),
			user: user.id,
			role: "Mentor".to_string(),
			content: "Test testimonial content for already deleted test".to_string(),
			created_at: chrono::Utc::now().to_rfc3339(),
			updated_at: chrono::Utc::now().to_rfc3339(),
			is_deleted: false,
		};
		let create_result = repo.query_create_testimonial(testimonial.clone()).await;
		assert!(create_result.is_ok());

		// Get created testimonial to get ID
		let created_testimonial = repo
			.query_testimonial_by_id(testimonial.id.id.to_raw())
			.await
			.unwrap();
		let testimonial_id = created_testimonial.id.id.to_raw();

		// Soft delete the testimonial twice
		let _ = repo.query_delete_testimonial(testimonial_id.clone()).await;
		let result = repo.query_delete_testimonial(testimonial_id).await;

		// Verify second deletion fails
		assert!(result.is_err());
		assert_eq!(result.unwrap_err().to_string(), "Testimonial not found");

		// Clean up
		let _ = UsersRepository::new(&app_state).query_delete_user(user.id.id.to_raw()).await;
	}
}