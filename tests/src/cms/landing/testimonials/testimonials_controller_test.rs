#[cfg(test)]
mod tests {
	use crate::{get_meta_request_dto, UsersRepository};
	use axum::{http::StatusCode, response::Response};
	use imphnen_cms::{
		v1::landing::testimonials::{
			testimonials_controller::TestimonialsController,
			testimonials_dto::{TestimonialsCreateRequestDto, TestimonialsUpdateRequestDto},
			testimonials_schema::TestimonialsSchema,
		},
	};
	use imphnen_entities::UsersSchema;
	use imphnen_utils::make_thing_from_enum;

	#[tokio::test]
	async fn test_get_testimonial_list_controller() {
		let app_state = crate::get_app_state().await;
		let repo = imphnen_cms::v1::landing::testimonials::testimonials_repository::TestimonialsRepository::new(&app_state);

		// Create test testimonials
		let testimonial_names = vec![
			"testimonial_list_1".to_string(),
			"testimonial_list_2".to_string(),
			"testimonial_list_3".to_string(),
		];

		for name in &testimonial_names {
			let user = UsersSchema {
				id: make_thing_from_enum("users", &uuid::Uuid::new_v4().to_string()),
				fullname: format!("Test User {}", name),
				email: format!("test{}@example.com", name),
				..Default::default()
			};
			let _ = UsersRepository::new(&app_state).query_create_user(user).await;

			let testimonial = TestimonialsSchema {
				id: make_thing_from_enum("testimonials", &uuid::Uuid::new_v4().to_string()),
				user: make_thing_from_enum("users", &uuid::Uuid::new_v4().to_string()),
				role: "Mentor".to_string(),
				content: format!("Great testimonial content for {}", name),
				created_at: chrono::Utc::now().to_rfc3339(),
				updated_at: chrono::Utc::now().to_rfc3339(),
				is_deleted: false,
			};
			let _ = repo.query_create_testimonial(testimonial).await;
		}

		// Get testimonial list through controller
		let response = TestimonialsController::get_testimonial_list(&app_state, get_meta_request_dto(1, 10))
			.await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);
		let body_json: serde_json::Value = crate::common::response_helpers::parse_response_value(response, 8192).await;
		let list = if let Some(d) = body_json.get("data") { d } else { &body_json };
		assert!(list.is_array(), "expected testimonial list to be an array");

		// Clean up
		for name in testimonial_names {
			let user = UsersRepository::new(&app_state)
				.query_user_by_email(format!("test{}@example.com", name))
				.await
				.unwrap();
			let _ = UsersRepository::new(&app_state).query_delete_user(user.id.id.to_raw()).await;
		}
	}

	#[tokio::test]
	async fn test_get_testimonial_by_id_controller() {
		let app_state = crate::get_app_state().await;
		let repo = imphnen_cms::v1::landing::testimonials::testimonials_repository::TestimonialsRepository::new(&app_state);

		// Create test testimonial
		let user = UsersSchema {
			id: make_thing_from_enum("users", &uuid::Uuid::new_v4().to_string()),
			fullname: "Test User".to_string(),
			email: "test@example.com".to_string(),
			..Default::default()
		};
		let create_user_result = UsersRepository::new(&app_state).query_create_user(user.clone()).await;
		assert!(create_user_result.is_ok());

		let testimonial = TestimonialsSchema {
			id: make_thing_from_enum("testimonials", &uuid::Uuid::new_v4().to_string()),
			user: user.id,
			role: "Mentor".to_string(),
			content: "Great testimonial content".to_string(),
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

		// Get testimonial by ID through controller
		let response = TestimonialsController::get_testimonial_by_id(&app_state, testimonial_id.clone())
			.await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);
		let body_json: serde_json::Value = crate::common::response_helpers::parse_response_value(response, 4096).await;
		let data = body_json.get("data").expect("expected data in OK response").clone();
		assert_eq!(data["content"].as_str().unwrap(), "Great testimonial content");

		// Clean up
		let _ = repo.query_delete_testimonial(testimonial_id).await;
		let _ = UsersRepository::new(&app_state).query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_get_testimonial_by_id_controller_not_found() {
		let app_state = crate::get_app_state().await;

		// Use non-existent ID
		let non_existent_id = "non-existent-uuid-123456789".to_string();

		// Get non-existent testimonial by ID through controller
		let response = TestimonialsController::get_testimonial_by_id(&app_state, non_existent_id)
			.await;

		// Verify not found response
		assert_eq!(response.status(), StatusCode::NOT_FOUND);
		let v = crate::common::response_helpers::parse_response_value(response, 2048).await;
		assert!(v.get("message").and_then(|m| m.as_str()).is_some(), "expected message in NOT_FOUND response");
	}

	#[tokio::test]
	async fn test_create_testimonial_controller() {
		let app_state = crate::get_app_state().await;
		let repo = imphnen_cms::v1::landing::testimonials::testimonials_repository::TestimonialsRepository::new(&app_state);

		// Create test user for authentication
		let user = UsersSchema {
			id: make_thing_from_enum("users", &uuid::Uuid::new_v4().to_string()),
			fullname: "Test Admin".to_string(),
			email: "admin@example.com".to_string(),
			..Default::default()
		};
		let create_user_result = UsersRepository::new(&app_state).query_create_user(user.clone()).await;
		assert!(create_user_result.is_ok());

		// Test data
		let testimonial_request = TestimonialsCreateRequestDto {
			role: "Mentor".to_string(),
			content: "This is a test testimonial content for controller test".to_string(),
		};

		// Create testimonial through controller
		let response = TestimonialsController::create_testimonial(
			&app_state,
			testimonial_request.clone(),
			&user,
		)
		.await;

		// Verify response
		assert_eq!(response.status(), StatusCode::CREATED);
		let v = crate::common::response_helpers::parse_response_value(response, 4096).await;
		assert!(v.get("data").is_some() || v.get("message").is_some(), "expected data or message in CREATED response");

		// Verify testimonial was created in database
		let created_testimonials = repo.query_testimonial_list(get_meta_request_dto(1, 10)).await.unwrap();
		assert!(created_testimonials.data.iter().any(|t| t.content == testimonial_request.content));

		// Clean up
		let created_testimonial = repo.query_testimonial_list(get_meta_request_dto(1, 10)).await.unwrap();
		for t in created_testimonials.data {
			if t.content == testimonial_request.content {
				let _ = repo.query_delete_testimonial(t.id.id.to_raw()).await;
			}
		}
		let _ = UsersRepository::new(&app_state).query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_create_testimonial_controller_invalid_data() {
		let app_state = crate::get_app_state().await;

		// Create test user for authentication
		let user = UsersSchema {
			id: make_thing_from_enum("users", &uuid::Uuid::new_v4().to_string()),
			fullname: "Test Admin".to_string(),
			email: "admin@example.com".to_string(),
			..Default::default()
		};
		let _ = UsersRepository::new(&app_state).query_create_user(user.clone()).await;

		// Test data with empty content (should fail validation)
		let testimonial_request = TestimonialsCreateRequestDto {
			role: "Mentor".to_string(),
			content: "".to_string(), // Empty content should fail validation
		};

		// Create testimonial through controller
		let response = TestimonialsController::create_testimonial(
			&app_state,
			testimonial_request,
			&user,
		)
		.await;

		// Verify bad request response (validation error)
		assert_eq!(response.status(), StatusCode::BAD_REQUEST);
		let v = crate::common::response_helpers::parse_response_value(response, 2048).await;
		assert!(v.get("message").and_then(|m| m.as_str()).is_some(), "expected message in BAD_REQUEST response");

		// Clean up
		let _ = UsersRepository::new(&app_state).query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_update_testimonial_controller() {
		let app_state = crate::get_app_state().await;
		let repo = imphnen_cms::v1::landing::testimonials::testimonials_repository::TestimonialsRepository::new(&app_state);

		// Create test user for authentication
		let user = UsersSchema {
			id: make_thing_from_enum("users", &uuid::Uuid::new_v4().to_string()),
			fullname: "Test Admin".to_string(),
			email: "admin@example.com".to_string(),
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

		// Prepare update request
		let update_request = TestimonialsUpdateRequestDto {
			role: Some("Updated Mentor".to_string()),
			content: Some(new_content.clone()),
		};

		// Update testimonial through controller
		let response = TestimonialsController::update_testimonial(
			&app_state, update_request, testimonial_id.clone(), &user,
		)
		.await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);
		let v = crate::common::response_helpers::parse_response_value(response, 2048).await;
		assert!(v.get("message").and_then(|m| m.as_str()).is_some() || v.get("data").is_some(), "expected message or data in OK response");

		// Verify testimonial was updated in database
		let updated_testimonial = repo
			.query_testimonial_by_id(testimonial_id.clone())
			.await
			.unwrap();
		assert_eq!(updated_testimonial.content, new_content);
		assert_eq!(updated_testimonial.role, "Updated Mentor");

		// Clean up
		let _ = repo.query_delete_testimonial(testimonial_id).await;
		let _ = UsersRepository::new(&app_state).query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_update_testimonial_controller_not_found() {
		let app_state = crate::get_app_state().await;

		// Create test user for authentication
		let user = UsersSchema {
			id: make_thing_from_enum("users", &uuid::Uuid::new_v4().to_string()),
			fullname: "Test Admin".to_string(),
			email: "admin@example.com".to_string(),
			..Default::default()
		};
		let _ = UsersRepository::new(&app_state).query_create_user(user.clone()).await;

		// Use non-existent ID
		let non_existent_id = "non-existent-uuid-123456789".to_string();

		// Prepare update request
		let update_request = TestimonialsUpdateRequestDto {
			role: Some("Updated Mentor".to_string()),
			content: Some("Updated content".to_string()),
		};

		// Update non-existent testimonial through controller
		let response = TestimonialsController::update_testimonial(
			&app_state, update_request, non_existent_id, &user,
		)
		.await;

		// Verify not found response
		assert_eq!(response.status(), StatusCode::NOT_FOUND);
		let v = crate::common::response_helpers::parse_response_value(response, 2048).await;
		assert!(v.get("message").and_then(|m| m.as_str()).is_some(), "expected message in NOT_FOUND response");

		// Clean up
		let _ = UsersRepository::new(&app_state).query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_delete_testimonial_controller() {
		let app_state = crate::get_app_state().await;
		let repo = imphnen_cms::v1::landing::testimonials::testimonials_repository::TestimonialsRepository::new(&app_state);

		// Create test user for authentication
		let user = UsersSchema {
			id: make_thing_from_enum("users", &uuid::Uuid::new_v4().to_string()),
			fullname: "Test Admin".to_string(),
			email: "admin@example.com".to_string(),
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

		// Delete testimonial through controller
		let response = TestimonialsController::delete_testimonial(
			&app_state, testimonial_id.clone(), &user,
		)
		.await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);
		let v = crate::common::response_helpers::parse_response_value(response, 2048).await;
		assert!(v.get("message").and_then(|m| m.as_str()).is_some() || v.get("data").is_some(), "expected message or data in OK response");

		// Verify testimonial was soft-deleted from database
		let deleted_testimonial = repo.query_testimonial_by_id(testimonial_id.clone()).await;
		assert!(deleted_testimonial.is_err());

		// Clean up - no need since it's already soft-deleted
		let _ = UsersRepository::new(&app_state).query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_delete_testimonial_controller_not_found() {
		let app_state = crate::get_app_state().await;

		// Create test user for authentication
		let user = UsersSchema {
			id: make_thing_from_enum("users", &uuid::Uuid::new_v4().to_string()),
			fullname: "Test Admin".to_string(),
			email: "admin@example.com".to_string(),
			..Default::default()
		};
		let _ = UsersRepository::new(&app_state).query_create_user(user.clone()).await;

		// Use non-existent ID
		let non_existent_id = "non-existent-uuid-123456789".to_string();

		// Delete non-existent testimonial through controller
		let response = TestimonialsController::delete_testimonial(
			&app_state, non_existent_id, &user,
		)
		.await;

		// Verify not found response
		assert_eq!(response.status(), StatusCode::NOT_FOUND);
		let v = crate::common::response_helpers::parse_response_value(response, 2048).await;
		assert!(v.get("message").and_then(|m| m.as_str()).is_some(), "expected message in NOT_FOUND response");

		// Clean up
		let _ = UsersRepository::new(&app_state).query_delete_user(user.id.id.to_raw()).await;
	}
	#[tokio::test]
	async fn test_create_testimonial_controller_content_boundary() {
		let app_state = crate::get_app_state().await;

		// Create test user for authentication
		let user = UsersSchema {
			id: make_thing_from_enum("users", &uuid::Uuid::new_v4().to_string()),
			fullname: "Test Admin".to_string(),
			email: "admin@example.com".to_string(),
			..Default::default()
		};
		let _ = UsersRepository::new(&app_state).query_create_user(user.clone()).await;

		// Test data with content exactly 500 characters (boundary test)
		let content_500 = "A".repeat(500);
		let testimonial_request = TestimonialsCreateRequestDto {
			role: "Mentor".to_string(),
			content: content_500.clone(),
		};

		// Create testimonial through controller
		let response = TestimonialsController::create_testimonial(
			&app_state,
			testimonial_request.clone(),
			&user,
		)
		.await;

		// Should succeed (boundary)
		assert_eq!(response.status(), StatusCode::CREATED);
		let v = crate::common::response_helpers::parse_response_value(response, 4096).await;
		assert!(v.get("data").is_some() || v.get("message").is_some(), "expected data or message in CREATED response");

		// Verify testimonial was created
		let created_testimonials = repo.query_testimonial_list(get_meta_request_dto(1, 10)).await.unwrap();
		assert!(created_testimonials.data.iter().any(|t| t.content == testimonial_request.content));

		// Clean up
		for t in created_testimonials.data {
			if t.content == testimonial_request.content {
				let _ = repo.query_delete_testimonial(t.id.id.to_raw()).await;
			}
		}
		let _ = UsersRepository::new(&app_state).query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_create_testimonial_controller_content_too_long() {
		let app_state = crate::get_app_state().await;

		// Create test user for authentication
		let user = UsersSchema {
			id: make_thing_from_enum("users", &uuid::Uuid::new_v4().to_string()),
			fullname: "Test Admin".to_string(),
			email: "admin@example.com".to_string(),
			..Default::default()
		};
		let _ = UsersRepository::new(&app_state).query_create_user(user.clone()).await;

		// Test data with content over 500 characters (should fail validation)
		let content_501 = "A".repeat(501);
		let testimonial_request = TestimonialsCreateRequestDto {
			role: "Mentor".to_string(),
			content: content_501,
		};

		// Create testimonial through controller
		let response = TestimonialsController::create_testimonial(
			&app_state,
			testimonial_request,
			&user,
		)
		.await;

		// Should fail validation
		assert_eq!(response.status(), StatusCode::BAD_REQUEST);
		let v = crate::common::response_helpers::parse_response_value(response, 2048).await;
		assert!(v.get("message").and_then(|m| m.as_str()).is_some(), "expected message in BAD_REQUEST response");

		// Clean up
		let _ = UsersRepository::new(&app_state).query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_create_testimonial_controller_empty_role() {
		let app_state = crate::get_app_state().await;

		// Create test user for authentication
		let user = UsersSchema {
			id: make_thing_from_enum("users", &uuid::Uuid::new_v4().to_string()),
			fullname: "Test Admin".to_string(),
			email: "admin@example.com".to_string(),
			..Default::default()
		};
		let _ = UsersRepository::new(&app_state).query_create_user(user.clone()).await;

		// Test data with empty role (should fail validation)
		let testimonial_request = TestimonialsCreateRequestDto {
			role: "".to_string(),
			content: "Valid content".to_string(),
		};

		// Create testimonial through controller
		let response = TestimonialsController::create_testimonial(
			&app_state,
			testimonial_request,
			&user,
		)
		.await;

		// Should fail validation
		assert_eq!(response.status(), StatusCode::BAD_REQUEST);
		let v = crate::common::response_helpers::parse_response_value(response, 2048).await;
		assert!(v.get("message").and_then(|m| m.as_str()).is_some(), "expected message in BAD_REQUEST response");

		// Clean up
		let _ = UsersRepository::new(&app_state).query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_update_testimonial_controller_content_boundary() {
		let app_state = crate::get_app_state().await;
		let repo = imphnen_cms::v1::landing::testimonials::testimonials_repository::TestimonialsRepository::new(&app_state);

		// Create test user for authentication
		let user = UsersSchema {
			id: make_thing_from_enum("users", &uuid::Uuid::new_v4().to_string()),
			fullname: "Test Admin".to_string(),
			email: "admin@example.com".to_string(),
			..Default::default()
		};
		let _ = UsersRepository::new(&app_state).query_create_user(user.clone()).await;

		// Create test testimonial
		let testimonial = TestimonialsSchema {
			id: make_thing_from_enum("testimonials", &uuid::Uuid::new_v4().to_string()),
			user: user.id,
			role: "Mentor".to_string(),
			content: "Original content".to_string(),
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

		// Prepare update request with content exactly 500 characters
		let content_500 = "B".repeat(500);
		let update_request = TestimonialsUpdateRequestDto {
			role: Some("Updated Mentor".to_string()),
			content: Some(content_500.clone()),
		};

		// Update testimonial through controller
		let response = TestimonialsController::update_testimonial(
			&app_state, update_request, testimonial_id.clone(), &user,
		)
		.await;

		// Should succeed
		assert_eq!(response.status(), StatusCode::OK);
		let v = crate::common::response_helpers::parse_response_value(response, 2048).await;
		assert!(v.get("message").and_then(|m| m.as_str()).is_some() || v.get("data").is_some(), "expected message or data in OK response");

		// Verify testimonial was updated
		let updated_testimonial = repo
			.query_testimonial_by_id(testimonial_id.clone())
			.await
			.unwrap();
		assert_eq!(updated_testimonial.content, content_500);

		// Clean up
		let _ = repo.query_delete_testimonial(testimonial_id).await;
		let _ = UsersRepository::new(&app_state).query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_update_testimonial_controller_content_too_long() {
		let app_state = crate::get_app_state().await;
		let repo = imphnen_cms::v1::landing::testimonials::testimonials_repository::TestimonialsRepository::new(&app_state);

		// Create test user for authentication
		let user = UsersSchema {
			id: make_thing_from_enum("users", &uuid::Uuid::new_v4().to_string()),
			fullname: "Test Admin".to_string(),
			email: "admin@example.com".to_string(),
			..Default::default()
		};
		let _ = UsersRepository::new(&app_state).query_create_user(user.clone()).await;

		// Create test testimonial
		let testimonial = TestimonialsSchema {
			id: make_thing_from_enum("testimonials", &uuid::Uuid::new_v4().to_string()),
			user: user.id,
			role: "Mentor".to_string(),
			content: "Original content".to_string(),
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

		// Prepare update request with content over 500 characters
		let content_501 = "B".repeat(501);
		let update_request = TestimonialsUpdateRequestDto {
			role: Some("Updated Mentor".to_string()),
			content: Some(content_501),
		};

		// Update testimonial through controller
		let response = TestimonialsController::update_testimonial(
			&app_state, update_request, testimonial_id.clone(), &user,
		)
		.await;

		// Should fail validation
		assert_eq!(response.status(), StatusCode::BAD_REQUEST);
		let v = crate::common::response_helpers::parse_response_value(response, 2048).await;
		assert!(v.get("message").and_then(|m| m.as_str()).is_some(), "expected message in BAD_REQUEST response");

		// Clean up
		let _ = repo.query_delete_testimonial(testimonial_id).await;
		let _ = UsersRepository::new(&app_state).query_delete_user(user.id.id.to_raw()).await;
	}
}
}