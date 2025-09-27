#[cfg(test)]
mod tests {
	use crate::get_meta_request_dto;
	use imphnen_cms::{
		v1::landing::events::{
			events_service::EventsService,
			events_dto::{EventsCreateRequestDto, EventsUpdateRequestDto},
			events_schema::EventsSchema,
		},
	};
	use chrono::{DateTime, Utc};

	#[tokio::test]
	async fn test_get_event_list_controller() {
		let app_state = crate::get_app_state().await;
		let response = EventsService::get_event_list(&app_state, get_meta_request_dto(1, 10))
			.await;
		assert_eq!(response.status(), 200);
	}

	#[tokio::test]
	async fn test_get_event_by_id_controller_found() {
		let app_state = crate::get_app_state().await;
		let repo = imphnen_cms::v1::landing::events::events_repository::EventsRepository::new(&app_state);

		// Create test event
		let event = EventsSchema {
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

		// Get event by ID through controller
		let response = EventsService::get_event_by_id(&app_state, event_id.clone())
			.await;

		// Verify response
		assert_eq!(response.status(), 200);

		// Clean up
		let _ = repo.query_delete_event(event_id).await;
	}

	#[tokio::test]
	async fn test_get_event_by_id_controller_not_found() {
		let app_state = crate::get_app_state().await;

		// Use non-existent ID
		let non_existent_id = "non-existent-uuid-123456789".to_string();

		// Get non-existent event by ID through controller
		let response = EventsService::get_event_by_id(&app_state, non_existent_id)
			.await;

		// Verify not found response
		assert_eq!(response.status(), 404);
	}

	#[tokio::test]
	async fn test_create_event_controller() {
		let app_state = crate::get_app_state().await;
		let repo = imphnen_cms::v1::landing::events::events_repository::EventsRepository::new(&app_state);

		// Test data
		let event_request = EventsCreateRequestDto {
			name: "Test Event".to_string(),
			description: "Test event description for controller test".to_string(),
			detail_link: "https://example.com/event".to_string(),
			price: 100.0,
			is_online: true,
			start_date: DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z").unwrap().with_timezone(&Utc),
			end_date: DateTime::parse_from_rfc3339("2024-01-02T00:00:00Z").unwrap().with_timezone(&Utc),
			location: Some("Online".to_string()),
		};

		// Create event through controller
		let response = EventsService::create_event(&app_state, event_request.clone())
			.await;

		// Verify response
		assert_eq!(response.status(), 201);

		// Verify event was created in database
		let created_events = repo.query_event_list(get_meta_request_dto(1, 10)).await.unwrap();
		assert!(created_events.data.iter().any(|e| e.name == event_request.name));

		// Clean up
		let created_event = repo.query_event_list(get_meta_request_dto(1, 10)).await.unwrap();
		for e in created_events.data {
			if e.name == event_request.name {
				let _ = repo.query_delete_event(e.id.id.to_raw()).await;
			}
		}
	}

	#[tokio::test]
	async fn test_create_event_controller_invalid_data() {
		let app_state = crate::get_app_state().await;

		// Test data with empty name (should fail validation)
		let event_request = EventsCreateRequestDto {
			name: "".to_string(),
			description: "Test event description".to_string(),
			detail_link: "https://example.com/event".to_string(),
			price: 100.0,
			is_online: true,
			start_date: DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z").unwrap().with_timezone(&Utc),
			end_date: DateTime::parse_from_rfc3339("2024-01-02T00:00:00Z").unwrap().with_timezone(&Utc),
			location: Some("Online".to_string()),
		};

		// Create event through controller
		let response = EventsService::create_event(&app_state, event_request)
			.await;

		// Verify bad request response (validation error)
		assert_eq!(response.status(), 400);
	}

	#[tokio::test]
	async fn test_update_event_controller() {
		let app_state = crate::get_app_state().await;
		let repo = imphnen_cms::v1::landing::events::events_repository::EventsRepository::new(&app_state);

		// Create test event
		let original_name = "Original Event Name".to_string();
		let new_name = "Updated Event Name".to_string();

		let event = EventsSchema {
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

		// Update event through controller
		let response = EventsService::update_event(&app_state, event_id.clone(), update_request)
			.await;

		// Verify response
		assert_eq!(response.status(), 200);

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
	async fn test_update_event_controller_not_found() {
		let app_state = crate::get_app_state().await;

		// Use non-existent ID
		let non_existent_id = "non-existent-uuid-123456789".to_string();

		// Prepare update request
		let update_request = EventsUpdateRequestDto {
			name: "Updated Event".to_string(),
			description: "Updated description".to_string(),
			detail_link: "https://example.com/updated".to_string(),
			price: 150.0,
			is_online: false,
			start_date: DateTime::parse_from_rfc3339("2024-02-01T00:00:00Z").unwrap().with_timezone(&Utc),
			end_date: DateTime::parse_from_rfc3339("2024-02-02T00:00:00Z").unwrap().with_timezone(&Utc),
			location: Some("Offline".to_string()),
		};

		// Update non-existent event through controller
		let response = EventsService::update_event(&app_state, non_existent_id, update_request)
			.await;

		// Verify not found response
		assert_eq!(response.status(), 400);
	}

	#[tokio::test]
	async fn test_delete_event_controller() {
		let app_state = crate::get_app_state().await;
		let repo = imphnen_cms::v1::landing::events::events_repository::EventsRepository::new(&app_state);

		// Create test event
		let event = EventsSchema {
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

		// Delete event through controller
		let response = EventsService::delete_event(&app_state, event_id.clone())
			.await;

		// Verify response
		assert_eq!(response.status(), 200);

		// Verify event was soft-deleted from database
		let deleted_event = repo.query_event_by_id(event_id.clone()).await;
		assert!(deleted_event.is_err());

		// Clean up - no need since it's already soft-deleted
	}

	#[tokio::test]
	async fn test_delete_event_controller_not_found() {
		let app_state = crate::get_app_state().await;

		// Use non-existent ID
		let non_existent_id = "non-existent-uuid-123456789".to_string();

		// Delete non-existent event through controller
		let response = EventsService::delete_event(&app_state, non_existent_id)
			.await;

		// Verify not found response
		assert_eq!(response.status(), 400);
	}
}