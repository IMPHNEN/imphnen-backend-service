#[cfg(test)]
mod tests {
	use crate::{get_meta_request_dto, UsersRepository};
	use axum::{http::StatusCode, response::Response};
	use imphnen_cms::{
		v1::landing::testimonials::{
			testimonials_dto::{TestimonialsCreateRequestDto, TestimonialsUpdateRequestDto},
			testimonials_service::TestimonialsService,
			testimonials_schema::TestimonialsSchema,
		},
	};
	use imphnen_entities::UsersSchema;
	use imphnen_utils::make_thing_from_enum;

	#[tokio::test]
	async fn test_get_testimonial_list_service() {
		let app_state = crate::get_app_state().await;
		let repo = imphnen_cms::v1::landing::testimonials::testimonials_repository::TestimonialsRepository::new(&app_state);

		// Create test testimonials
		let testimonial_contents = vec![
			"Testimonial content 1".to_string(),
			"Testimonial content 2".to_string(),
			"Testimonial content 3".to_string(),
		];

		for content in &testimonial_contents {
			let user = UsersSchema {
				id: make_thing_from_enum("users", &uuid::Uuid::new_v4().to_string()),
				fullname: format!("Test User for {content}"),
				email: format!("testuser{}@example.com", content.chars().take(5).collect::<String>()),
				..Default::default()
			};
			let _ = UsersRepository::new(&app_state).query_create_user(user.clone()).await;

			let testimonial = TestimonialsSchema {
				id: make_thing_from_enum("testimonials", &uuid::Uuid::new_v4().to_string()),
				user: user.id,
				role: "Mentor".to_string(),
				content: content.clone(),
				created_at: chrono::Utc::now().to_rfc3339(),
				updated_at: chrono::Utc::now().to_rfc3339(),
				is_deleted: false,
			};
			let _ = repo.query_create_testimonial(testimonial).await;
		}

		// Get testimonial list through service
		let response = TestimonialsService::get_testimonial_list(&app_state, get_meta_request_dto(1, 10))
			.await;

		// Verify response (status + body)
		assert_eq!(response.status(), StatusCode::OK);
		let body_json: serde_json::Value =
			crate::common::response_helpers::parse_response_value(response, 8192).await;
		let list = if let Some(d) = body_json.get("data") { d } else { &body_json };
		assert!(list.is_array(), "expected testimonial list to be an array");

		// Clean up
		for content in testimonial_contents {
			let user = UsersRepository::new(&app_state)
				.query_user_by_email(format!("testuser{}@example.com", content.chars().take(5).collect::<String>()))
				.await
				.unwrap();
			let _ = UsersRepository::new(&app_state).query_delete_user(user.id.id.to_raw()).await;
		}
	}

	#[tokio::test]
	async fn test_get_testimonial_by_id_service_found() {
		let app_state = crate::get_app_state().await;
		let repo = imphnen_cms::v1::landing::testimonials::testimonials_repository::TestimonialsRepository::new(&app_state);

		// Create test user
		let user = UsersSchema {
			id: make_thing_from_enum("users", &uuid::Uuid::new_v4().to_string()),
			fullname: "Test User".to_string(),
			email: "testuser@example.com".to_string(),
			..Default::default()
		};
		let _ = UsersRepository::new(&app_state).query_create_user(user.clone()).await;

		// Create test testimonial
		let testimonial_content = "Test testimonial content for get by ID service test".to_string();
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

		// Get testimonial by ID through service
		let response = TestimonialsService::get_testimonial_by_id(&app_state, testimonial_id.clone())
			.await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);

		// Verify response body contains correct data
		let response_body: serde_json::Value =
			crate::common::response_helpers::parse_response_value(response, 8192).await;
		assert_eq!(response_body["data"]["content"].as_str().unwrap(), testimonial_content);

		// Clean up
		let _ = repo.query_delete_testimonial(testimonial_id).await;
		let _ = UsersRepository::new(&app_state).query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_get_testimonial_by_id_service_not_found() {
		let app_state = crate::get_app_state().await;

		// Use non-existent ID
		let non_existent_id = "non-existent-uuid-123456789".to_string();

		// Get non-existent testimonial by ID through service
		let response = TestimonialsService::get_testimonial_by_id(&app_state, non_existent_id)
			.await;

		// Verify not found response
		assert_eq!(response.status(), StatusCode::NOT_FOUND);
		let v = crate::common::response_helpers::parse_response_value(response, 2048).await;
		assert!(v.get("message").and_then(|m| m.as_str()).is_some(), "expected message in NOT_FOUND response");
	}

	#[tokio::test]
	async fn test_get_testimonial_by_id_service_deleted() {
		let app_state = crate::get_app_state().await;
		let repo = imphnen_cms::v1::landing::testimonials::testimonials_repository::TestimonialsRepository::new(&app_state);

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

		// Try to get deleted testimonial through service
		let response = TestimonialsService::get_testimonial_by_id(&app_state, testimonial_id)
			.await;

		// Verify not found response (service should filter out deleted items)
		assert_eq!(response.status(), StatusCode::NOT_FOUND);
		let v = crate::common::response_helpers::parse_response_value(response, 2048).await;
		assert!(v.get("message").and_then(|m| m.as_str()).is_some(), "expected message in NOT_FOUND response");

		// Clean up
		let _ = UsersRepository::new(&app_state).query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_create_testimonial_service() {
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

		// Test data
		let testimonial_request = TestimonialsCreateRequestDto {
			role: "Mentor".to_string(),
			content: "Test testimonial content for service create test".to_string(),
		};

		// Create testimonial through service
		let response = TestimonialsService::create_testimonial(
			&app_state, testimonial_request.clone(), &user,
		)
		.await;

		// Verify response (status + body)
		assert_eq!(response.status(), StatusCode::CREATED);
		let v = crate::common::response_helpers::parse_response_value(response, 4096).await;
		// either data or message expected depending on implementation
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
	async fn test_create_testimonial_service_invalid_data() {
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

		// Create testimonial through service
		let response = TestimonialsService::create_testimonial(
			&app_state, testimonial_request, &user,
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
	async fn test_update_testimonial_service() {
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
		let original_content = "Original testimonial content for service update test".to_string();
		let new_content = "Updated testimonial content for service update test".to_string();

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

		// Update testimonial through service
		let response = TestimonialsService::update_testimonial(
			&app_state, update_request, testimonial_id.clone(), &user,
		)
		.await;

		// Verify response (status + body)
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
	async fn test_update_testimonial_service_not_found() {
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

		// Update non-existent testimonial through service
		let response = TestimonialsService::update_testimonial(
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
	async fn test_update_testimonial_service_deleted() {
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

		// Prepare update request
		let update_request = TestimonialsUpdateRequestDto {
			role: Some("Updated Mentor".to_string()),
			content: Some("Updated content".to_string()),
		};

		// Try to update deleted testimonial through service
		let response = TestimonialsService::update_testimonial(
			&app_state, update_request, testimonial_id, &user,
		)
		.await;

		// Verify bad request response (should fail because it's deleted)
		assert_eq!(response.status(), StatusCode::BAD_REQUEST);
		let v = crate::common::response_helpers::parse_response_value(response, 2048).await;
		assert!(v.get("message").and_then(|m| m.as_str()).is_some(), "expected message in BAD_REQUEST response");

		// Clean up
		let _ = UsersRepository::new(&app_state).query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_delete_testimonial_service() {
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
			content: "Test testimonial content for service delete test".to_string(),
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

		// Delete testimonial through service
		let response = TestimonialsService::delete_testimonial(
			&app_state, testimonial_id.clone(), &user,
		)
		.await;

		// Verify response
		// Verify response (status + body)
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
	async fn test_delete_testimonial_service_not_found() {
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

		// Delete non-existent testimonial through service
		let response = TestimonialsService::delete_testimonial(
			&app_state, non_existent_id, &user,
		)
		.await;

		// Verify bad request response
		assert_eq!(response.status(), StatusCode::BAD_REQUEST);
		let v = crate::common::response_helpers::parse_response_value(response, 2048).await;
		assert!(v.get("message").and_then(|m| m.as_str()).is_some(), "expected message in BAD_REQUEST response");

		// Clean up
		let _ = UsersRepository::new(&app_state).query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_delete_testimonial_service_deleted_twice() {
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
			content: "Test testimonial content for double delete test".to_string(),
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

		// Delete testimonial once
		let _ = repo.query_delete_testimonial(testimonial_id.clone()).await;

		// Try to delete again through service
		let response = TestimonialsService::delete_testimonial(
			&app_state, testimonial_id, &user,
		)
		.await;

		// Verify bad request response (should fail because it's already deleted)
		assert_eq!(response.status(), StatusCode::BAD_REQUEST);
		let v = crate::common::response_helpers::parse_response_value(response, 2048).await;
		assert!(v.get("message").and_then(|m| m.as_str()).is_some(), "expected message in BAD_REQUEST response");

		// Clean up
		let _ = UsersRepository::new(&app_state).query_delete_user(user.id.id.to_raw()).await;
	}
}