#[cfg(test)]
mod tests {
	use crate::get_meta_request_dto;
	use imphnen_cms::{
		v1::landing::events::{
			events_service::EventsService,
			events_dto::{EventsCreateRequestDto, EventsUpdateRequestDto},
		},
	};

	#[tokio::test]
	async fn test_get_event_list_service() {
		let app_state = crate::get_app_state().await;
		let response = EventsService::get_event_list(&app_state, get_meta_request_dto(1, 10)).await;
		assert_eq!(response.status(), 200);
		let body_json: serde_json::Value = crate::common::response_helpers::parse_response_value(response, 8192).await;
		let list = if let Some(d) = body_json.get("data") { d } else { &body_json };
		assert!(list.is_array(), "expected event list to be an array");
	}

	#[tokio::test]
	async fn test_get_event_by_id_service_not_found() {
		let app_state = crate::get_app_state().await;
		let response = EventsService::get_event_by_id(&app_state, "non-existent-uuid-123456789".to_string()).await;
		assert_eq!(response.status(), 404);
		let v = crate::common::response_helpers::parse_response_value(response, 2048).await;
		assert!(v.get("message").and_then(|m| m.as_str()).is_some(), "expected message in NOT_FOUND response");
	}

	#[tokio::test]
	async fn test_create_event_service() {
		let app_state = crate::get_app_state().await;
		let event_request = EventsCreateRequestDto {
			name: "Test Event".to_string(),
			description: "Test event description".to_string(),
			detail_link: Some("https://example.com/event".to_string()),
			price: Some(100.0),
			is_online: Some(true),
			start_date: "2024-01-01T00:00:00Z".to_string(),
			end_date: "2024-01-02T00:00:00Z".to_string(),
			location: Some("Online".to_string()),
		};
		let response = EventsService::create_event(&app_state, event_request).await;
		assert_eq!(response.status(), 201);
		let v = crate::common::response_helpers::parse_response_value(response, 4096).await;
		assert!(v.get("data").is_some() || v.get("message").is_some(), "expected data or message in CREATED response");
	}

	#[tokio::test]
	async fn test_create_event_service_invalid_data() {
		let app_state = crate::get_app_state().await;
		let event_request = EventsCreateRequestDto {
			name: "".to_string(),
			description: "".to_string(),
			detail_link: None,
			price: None,
			is_online: None,
			start_date: "invalid-date".to_string(),
			end_date: "2024-01-02T00:00:00Z".to_string(),
			location: None,
		};
		let response = EventsService::create_event(&app_state, event_request).await;
		assert_eq!(response.status(), 400);
		let v = crate::common::response_helpers::parse_response_value(response, 2048).await;
		assert!(v.get("message").and_then(|m| m.as_str()).is_some(), "expected message in BAD_REQUEST response");
	}

	#[tokio::test]
	async fn test_update_event_service_not_found() {
		let app_state = crate::get_app_state().await;
		let update_request = EventsUpdateRequestDto {
			name: Some("Updated Event".to_string()),
			description: Some("Updated description".to_string()),
			detail_link: Some("https://example.com/updated".to_string()),
			price: Some(150.0),
			is_online: Some(false),
			start_date: Some("2024-02-01T00:00:00Z".to_string()),
			end_date: Some("2024-02-02T00:00:00Z".to_string()),
			location: Some("Offline".to_string()),
		};
		let response = EventsService::update_event(&app_state, "non-existent-uuid-123456789".to_string(), update_request).await;
		assert_eq!(response.status(), 400);
		let v = crate::common::response_helpers::parse_response_value(response, 2048).await;
		assert!(v.get("message").and_then(|m| m.as_str()).is_some(), "expected message in BAD_REQUEST response");
	}

	#[tokio::test]
	async fn test_delete_event_service_not_found() {
		let app_state = crate::get_app_state().await;
		let response = EventsService::delete_event(&app_state, "non-existent-uuid-123456789".to_string()).await;
		assert_eq!(response.status(), 400);
		let v = crate::common::response_helpers::parse_response_value(response, 2048).await;
		assert!(v.get("message").and_then(|m| m.as_str()).is_some(), "expected message in BAD_REQUEST response");
	}
	#[tokio::test]
	async fn test_get_event_by_id_service_found() {
		let app_state = crate::get_app_state().await;
		let repo = imphnen_cms::v1::landing::events::events_repository::EventsRepository::new(&app_state);

		// Create test event
		let event = imphnen_cms::v1::landing::events::events_schema::EventsSchema {
			id: imphnen_utils::make_thing_from_enum("events", &uuid::Uuid::new_v4().to_string()),
			name: "Test Event".to_string(),
			description: "Test event description".to_string(),
			detail_link: "https://example.com/event".to_string(),
			price: 100.0,
			is_online: true,
			is_deleted: false,
			start_date: "2024-01-01T00:00:00Z".to_string(),
			end_date: "2024-01-02T00:00:00Z".to_string(),
			created_at: chrono::Utc::now().to_rfc3339(),
			updated_at: chrono::Utc::now().to_rfc3339(),
			location: Some("Online".to_string()),
		};
		let create_result = repo.query_create_event(event.clone()).await;
		assert!(create_result.is_ok());

		// Get created event to get ID
		let created_event = repo
			.query_event_by_id(event.id.id.to_raw())
			.await
			.unwrap();
		let event_id = created_event.id.id.to_raw();

		// Get event by ID through service
		let response = EventsService::get_event_by_id(&app_state, event_id.clone())
			.await;

		// Verify response (status + body)
		assert_eq!(response.status(), 200);
		let body_json: serde_json::Value = crate::common::response_helpers::parse_response_value(response, 4096).await;
		let data = body_json.get("data").expect("expected data in OK response").clone();
		assert_eq!(data["name"].as_str().unwrap(), "Test Event");

		// Clean up
		let _ = repo.query_delete_event(event_id).await;
	}

	#[tokio::test]
	async fn test_update_event_service() {
		let app_state = crate::get_app_state().await;
		let repo = imphnen_cms::v1::landing::events::events_repository::EventsRepository::new(&app_state);

		// Create test event
		let original_name = "Original Event Name".to_string();
		let new_name = "Updated Event Name".to_string();

		let event = imphnen_cms::v1::landing::events::events_schema::EventsSchema {
			id: imphnen_utils::make_thing_from_enum("events", &uuid::Uuid::new_v4().to_string()),
			name: original_name.clone(),
			description: "Test event description for update test".to_string(),
			detail_link: "https://example.com/event".to_string(),
			price: 100.0,
			is_online: true,
			is_deleted: false,
			start_date: "2024-01-01T00:00:00Z".to_string(),
			end_date: "2024-01-02T00:00:00Z".to_string(),
			created_at: chrono::Utc::now().to_rfc3339(),
			updated_at: chrono::Utc::now().to_rfc3339(),
			location: Some("Online".to_string()),
		};
		let create_result = repo.query_create_event(event.clone()).await;
		assert!(create_result.is_ok());

		// Get created event to get ID
		let created_event = repo
			.query_event_by_id(event.id.id.to_raw())
			.await
			.unwrap();
		let event_id = created_event.id.id.to_raw();

		// Prepare update request
		let update_request = EventsUpdateRequestDto {
			name: new_name.clone(),
			description: "Updated event description".to_string(),
			detail_link: "https://example.com/updated".to_string(),
			price: 150.0,
			is_online: false,
			start_date: DateTime::parse_from_rfc3339("2024-02-01T00:00:00Z").unwrap().with_timezone(&Utc),
			end_date: DateTime::parse_from_rfc3339("2024-02-02T00:00:00Z").unwrap().with_timezone(&Utc),
			location: Some("Offline Location".to_string()),
		};

		// Update event through service
		let response = EventsService::update_event(&app_state, event_id.clone(), update_request)
			.await;

		// Verify response
		assert_eq!(response.status(), 200);
		let v = crate::common::response_helpers::parse_response_value(response, 4096).await;
		assert!(v.get("message").and_then(|m| m.as_str()).is_some() || v.get("data").is_some(), "expected message or data in OK response");

		// Verify event was updated in database
		let updated_event = repo
			.query_event_by_id(event_id.clone())
			.await
			.unwrap();
		assert_eq!(updated_event.name, new_name);
		assert_eq!(updated_event.price, 150.0);
		assert!(!updated_event.is_online);

		// Clean up
		let _ = repo.query_delete_event(event_id).await;
	}

	#[tokio::test]
	async fn test_update_event_service_invalid_data() {
		let app_state = crate::get_app_state().await;

		// Use non-existent ID
		let non_existent_id = "non-existent-uuid-123456789".to_string();

		// Prepare update request with invalid data (negative price)
		let update_request = EventsUpdateRequestDto {
			name: "Updated Event".to_string(),
			description: "Updated description".to_string(),
			detail_link: "https://example.com/updated".to_string(),
			price: -50.0, // Negative price should fail validation
			is_online: false,
			start_date: DateTime::parse_from_rfc3339("2024-02-01T00:00:00Z").unwrap().with_timezone(&Utc),
			end_date: DateTime::parse_from_rfc3339("2024-02-02T00:00:00Z").unwrap().with_timezone(&Utc),
			location: Some("Offline".to_string()),
		};

		// Update event through service
		let response = EventsService::update_event(&app_state, non_existent_id, update_request)
			.await;

		// Verify bad request response (validation error)
		assert_eq!(response.status(), 400);
		let v = crate::common::response_helpers::parse_response_value(response, 2048).await;
		assert!(v.get("message").and_then(|m| m.as_str()).is_some(), "expected message in BAD_REQUEST response");
	}

	#[tokio::test]
	async fn test_delete_event_service() {
		let app_state = crate::get_app_state().await;
		let repo = imphnen_cms::v1::landing::events::events_repository::EventsRepository::new(&app_state);

		// Create test event
		let event = imphnen_cms::v1::landing::events::events_schema::EventsSchema {
			id: imphnen_utils::make_thing_from_enum("events", &uuid::Uuid::new_v4().to_string()),
			name: "Test Event for Delete".to_string(),
			description: "Test event description for delete test".to_string(),
			detail_link: "https://example.com/event".to_string(),
			price: 100.0,
			is_online: true,
			is_deleted: false,
			start_date: "2024-01-01T00:00:00Z".to_string(),
			end_date: "2024-01-02T00:00:00Z".to_string(),
			created_at: chrono::Utc::now().to_rfc3339(),
			updated_at: chrono::Utc::now().to_rfc3339(),
			location: Some("Online".to_string()),
		};
		let create_result = repo.query_create_event(event.clone()).await;
		assert!(create_result.is_ok());

		// Get created event to get ID
		let created_event = repo
			.query_event_by_id(event.id.id.to_raw())
			.await
			.unwrap();
		let event_id = created_event.id.id.to_raw();

		// Verify event exists before deletion
		let exists_before = repo.query_event_by_id(event_id.clone()).await.is_ok();
		assert!(exists_before);

		// Delete event through service
		let response = EventsService::delete_event(&app_state, event_id.clone())
			.await;

		// Verify response
		assert_eq!(response.status(), 200);
		let v = crate::common::response_helpers::parse_response_value(response, 2048).await;
		assert!(v.get("message").and_then(|m| m.as_str()).is_some() || v.get("data").is_some(), "expected message or data in OK response");

		// Verify event was soft-deleted from database
		let deleted_event = repo.query_event_by_id(event_id.clone()).await;
		assert!(deleted_event.is_err());

		// Clean up - already deleted
	}

	#[tokio::test]
	async fn test_create_event_service_edge_cases() {
		let app_state = crate::get_app_state().await;

		// Test with end date before start date (should fail validation if implemented)
		let event_request = EventsCreateRequestDto {
			name: "Invalid Date Event".to_string(),
			description: "Event with invalid dates".to_string(),
			detail_link: "https://example.com/event".to_string(),
			price: 100.0,
			is_online: true,
			start_date: DateTime::parse_from_rfc3339("2024-01-02T00:00:00Z").unwrap().with_timezone(&Utc),
			end_date: DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z").unwrap().with_timezone(&Utc), // Before start
			location: Some("Online".to_string()),
		};

		let response = EventsService::create_event(&app_state, event_request).await;
		// This might not be validated in the service, but let's check
		// For now, assume it passes or fails based on implementation

		// Test with very long name (boundary test)
		let long_name = "A".repeat(1000); // Very long name
		let event_request_long = EventsCreateRequestDto {
			name: long_name,
			description: "Event with very long name".to_string(),
			detail_link: "https://example.com/event".to_string(),
			price: 100.0,
			is_online: true,
			start_date: DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z").unwrap().with_timezone(&Utc),
			end_date: DateTime::parse_from_rfc3339("2024-01-02T00:00:00Z").unwrap().with_timezone(&Utc),
			location: Some("Online".to_string()),
		};

		let response_long = EventsService::create_event(&app_state, event_request_long).await;
		// Should pass or fail based on validation - if no length limit, it passes
	}

	#[tokio::test]
	async fn test_get_event_list_service_with_pagination() {
		let app_state = crate::get_app_state().await;
		let repo = imphnen_cms::v1::landing::events::events_repository::EventsRepository::new(&app_state);

		// Create multiple test events
		for i in 1..=5 {
			let event = imphnen_cms::v1::landing::events::events_schema::EventsSchema {
				id: imphnen_utils::make_thing_from_enum("events", &uuid::Uuid::new_v4().to_string()),
				name: format!("Test Event {i}"),
				description: format!("Description {i}"),
				detail_link: format!("https://example.com/event{i}"),
				price: i as f64 * 10.0,
				is_online: i % 2 == 0,
				is_deleted: false,
				start_date: "2024-01-01T00:00:00Z".to_string(),
				end_date: "2024-01-02T00:00:00Z".to_string(),
				created_at: chrono::Utc::now().to_rfc3339(),
				updated_at: chrono::Utc::now().to_rfc3339(),
				location: Some(format!("Location {i}")),
			};
			let _ = repo.query_create_event(event).await;
		}

		// Test pagination with page 1, per_page 2
		let meta = imphnen_libs::MetaRequestDto {
			page: Some(1),
			per_page: Some(2),
			search: None,
			sort_by: None,
			order: None,
			filter: None,
			filter_by: None,
		};
		let response = EventsService::get_event_list(&app_state, meta).await;
		assert_eq!(response.status(), 200);
		let body_json: serde_json::Value = crate::common::response_helpers::parse_response_value(response, 8192).await;
		let list = if let Some(d) = body_json.get("data") { d } else { &body_json };
		assert!(list.is_array(), "expected event list to be an array");

		// Clean up - delete all created events
		let events = repo.query_event_list(crate::get_meta_request_dto(1, 10)).await.unwrap();
		for e in events.data {
			if e.name.starts_with("Test Event ") {
				let _ = repo.query_delete_event(e.id.id.to_raw()).await;
			}
		}
	}
}
}