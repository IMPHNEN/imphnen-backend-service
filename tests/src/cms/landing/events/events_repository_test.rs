#[cfg(test)]
mod tests {
	use crate::get_meta_request_dto;
	use imphnen_cms::v1::landing::events::{
		events_repository::EventsRepository,
		events_schema::EventsSchema,
	};
	use imphnen_utils::make_thing_from_enum;

	#[tokio::test]
	async fn test_query_event_list() {
		let app_state = crate::get_app_state().await;
		let repo = EventsRepository::new(&app_state);

		// Create test events
		let num_events = 5;
		let event_names = vec![
			"Event 1".to_string(),
			"Event 2".to_string(),
			"Event 3".to_string(),
			"Event 4".to_string(),
			"Event 5".to_string(),
		];

		for (i, name) in event_names.iter().enumerate() {
			let event = EventsSchema {
				id: make_thing_from_enum("events", &uuid::Uuid::new_v4().to_string()),
				name: name.clone(),
				description: format!("Description for {name}"),
				detail_link: format!("https://example.com/event{}", i + 1),
				price: (i + 1) as f64 * 10.0,
				is_online: i % 2 == 0,
				is_deleted: false,
				start_date: "2024-01-01T00:00:00Z".to_string(),
				end_date: "2024-01-02T00:00:00Z".to_string(),
				created_at: chrono::Utc::now().to_rfc3339(),
				updated_at: chrono::Utc::now().to_rfc3339(),
				location: if i % 2 == 0 { Some("Online".to_string()) } else { Some("Offline Venue".to_string()) },
			};
			let _ = repo.query_create_event(event).await;
		}

		// Test with pagination
		let result = repo.query_event_list(get_meta_request_dto(1, 10)).await;
		assert!(result.is_ok());
		let response = result.unwrap();
		assert_eq!(response.data.len(), num_events);

		// Test with smaller page size
		let result = repo.query_event_list(get_meta_request_dto(1, 2)).await;
		assert!(result.is_ok());
		let response = result.unwrap();
		assert_eq!(response.data.len(), 2);

		// Clean up - delete all created events
		for name in event_names {
			let events = repo.query_event_list(get_meta_request_dto(1, 10)).await.unwrap();
			for e in events.data {
				if e.name == name {
					let _ = repo.query_delete_event(e.id.id.to_raw()).await;
				}
			}
		}
	}

	#[tokio::test]
	async fn test_query_event_by_id_found() {
		let app_state = crate::get_app_state().await;
		let repo = EventsRepository::new(&app_state);

		// Create test event
		let event_name = "Test Event for By ID".to_string();
		let event = EventsSchema {
			id: make_thing_from_enum("events", &uuid::Uuid::new_v4().to_string()),
			name: event_name.clone(),
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

		// Query event by ID
		let result = repo.query_event_by_id(event_id.clone()).await;
		assert!(result.is_ok());
		let found_event = result.unwrap();
		assert_eq!(found_event.name, event_name);
		assert_eq!(found_event.price, 100.0);
		assert!(found_event.is_online);
		assert!(!found_event.is_deleted);

		// Clean up
		let _ = repo.query_delete_event(event_id).await;
	}

	#[tokio::test]
	async fn test_query_event_by_id_not_found() {
		let app_state = crate::get_app_state().await;
		let repo = EventsRepository::new(&app_state);

		// Use non-existent ID
		let non_existent_id = "non-existent-uuid-123456789".to_string();

		// Query non-existent event by ID
		let result = repo.query_event_by_id(non_existent_id).await;
		assert!(result.is_err());
		assert_eq!(result.unwrap_err().to_string(), "Event not found");
	}

	#[tokio::test]
	async fn test_query_event_by_id_deleted() {
		let app_state = crate::get_app_state().await;
		let repo = EventsRepository::new(&app_state);

		// Create test event
		let event = EventsSchema {
			id: make_thing_from_enum("events", &uuid::Uuid::new_v4().to_string()),
			name: "Test Event for Deleted".to_string(),
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

		// Soft delete the event
		let _ = repo.query_delete_event(event_id.clone()).await;

		// Try to query deleted event by ID
		let result = repo.query_event_by_id(event_id).await;
		assert!(result.is_err());
		assert_eq!(result.unwrap_err().to_string(), "Event not found");

		// Clean up - already deleted
	}

	#[tokio::test]
	async fn test_query_create_event() {
		let app_state = crate::get_app_state().await;
		let repo = EventsRepository::new(&app_state);

		// Create test event data
		let event = EventsSchema {
			id: make_thing_from_enum("events", &uuid::Uuid::new_v4().to_string()),
			name: "Test Event for Create".to_string(),
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

		// Create event
		let result = repo.query_create_event(event.clone()).await;
		assert!(result.is_ok());
		assert_eq!(result.unwrap(), "Success create event");

		// Verify it was created in database
		let found_event = repo.query_event_by_id(event.id.id.to_raw()).await;
		assert!(found_event.is_ok());
		assert_eq!(found_event.unwrap().name, event.name);

		// Clean up
		let _ = repo.query_delete_event(event.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_query_update_event() {
		let app_state = crate::get_app_state().await;
		let repo = EventsRepository::new(&app_state);

		// Create test event
		let original_name = "Original Event Name".to_string();
		let new_name = "Updated Event Name".to_string();

		let event = EventsSchema {
			id: make_thing_from_enum("events", &uuid::Uuid::new_v4().to_string()),
			name: original_name.clone(),
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

		// Prepare updated event
		let updated_event = EventsSchema {
			id: created_event.id,
			name: new_name.clone(),
			description: "Updated description".to_string(),
			detail_link: "https://example.com/updated".to_string(),
			price: 150.0,
			is_online: false,
			is_deleted: false,
			start_date: "2024-02-01T00:00:00Z".to_string(),
			end_date: "2024-02-02T00:00:00Z".to_string(),
			created_at: created_event.created_at,
			updated_at: chrono::Utc::now().to_rfc3339(),
			location: Some("Offline Venue".to_string()),
		};

		// Update event
		let result = repo.query_update_event(updated_event).await;
		assert!(result.is_ok());
		assert_eq!(result.unwrap(), "Success update event");

		// Verify it was updated in database
		let found_event = repo.query_event_by_id(event_id).await;
		assert!(found_event.is_ok());
		let updated = found_event.unwrap();
		assert_eq!(updated.name, new_name);
		assert_eq!(updated.price, 150.0);
		assert!(!updated.is_online);

		// Clean up
		let _ = repo.query_delete_event(event_id).await;
	}

	#[tokio::test]
	async fn test_query_update_event_not_found() {
		let app_state = crate::get_app_state().await;
		let repo = EventsRepository::new(&app_state);

		// Create non-existent event ID
		let non_existent_id = "non-existent-uuid-123456789".to_string();

		// Prepare updated event with non-existent ID
		let updated_event = EventsSchema {
			id: make_thing_from_enum("events", &non_existent_id),
			name: "Updated Event".to_string(),
			description: "Updated description".to_string(),
			detail_link: "https://example.com/updated".to_string(),
			price: 150.0,
			is_online: false,
			is_deleted: false,
			start_date: "2024-02-01T00:00:00Z".to_string(),
			end_date: "2024-02-02T00:00:00Z".to_string(),
			created_at: chrono::Utc::now().to_rfc3339(),
			updated_at: chrono::Utc::now().to_rfc3339(),
			location: Some("Offline".to_string()),
		};

		// Try to update non-existent event
		let result = repo.query_update_event(updated_event).await;
		assert!(result.is_err());
		assert_eq!(result.unwrap_err().to_string(), "Event not found");
	}

	#[tokio::test]
	async fn test_query_update_event_deleted() {
		let app_state = crate::get_app_state().await;
		let repo = EventsRepository::new(&app_state);

		// Create test event
		let event = EventsSchema {
			id: make_thing_from_enum("events", &uuid::Uuid::new_v4().to_string()),
			name: "Test Event for Deleted Update".to_string(),
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

		// Soft delete the event
		let _ = repo.query_delete_event(event_id.clone()).await;

		// Prepare updated event
		let updated_event = EventsSchema {
			id: created_event.id,
			name: "Updated Event".to_string(),
			description: "Updated description".to_string(),
			detail_link: "https://example.com/updated".to_string(),
			price: 150.0,
			is_online: false,
			is_deleted: false,
			start_date: "2024-02-01T00:00:00Z".to_string(),
			end_date: "2024-02-02T00:00:00Z".to_string(),
			created_at: created_event.created_at,
			updated_at: chrono::Utc::now().to_rfc3339(),
			location: Some("Offline".to_string()),
		};

		// Try to update deleted event
		let result = repo.query_update_event(updated_event).await;
		assert!(result.is_err());
		assert_eq!(result.unwrap_err().to_string(), "Event already deleted");

		// Clean up - already deleted
	}

	#[tokio::test]
	async fn test_query_delete_event() {
		let app_state = crate::get_app_state().await;
		let repo = EventsRepository::new(&app_state);

		// Create test event
		let event = EventsSchema {
			id: make_thing_from_enum("events", &uuid::Uuid::new_v4().to_string()),
			name: "Test Event for Delete".to_string(),
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

		// Verify event exists before deletion
		let exists_before = repo.query_event_by_id(event_id.clone()).await.is_ok();
		assert!(exists_before);

		// Delete event
		let result = repo.query_delete_event(event_id.clone()).await;
		assert!(result.is_ok());
		assert_eq!(result.unwrap(), "Success delete event");

		// Verify event was soft-deleted from database
		let deleted_event = repo.query_event_by_id(event_id.clone()).await;
		assert!(deleted_event.is_err());
		assert_eq!(deleted_event.unwrap_err().to_string(), "Event not found");

		// Clean up - already deleted
	}

	#[tokio::test]
	async fn test_query_delete_event_not_found() {
		let app_state = crate::get_app_state().await;
		let repo = EventsRepository::new(&app_state);

		// Use non-existent ID
		let non_existent_id = "non-existent-uuid-123456789".to_string();

		// Try to delete non-existent event
		let result = repo.query_delete_event(non_existent_id).await;
		assert!(result.is_err());
		assert_eq!(result.unwrap_err().to_string(), "Event not found");
	}

	#[tokio::test]
	async fn test_query_delete_event_already_deleted() {
		let app_state = crate::get_app_state().await;
		let repo = EventsRepository::new(&app_state);

		// Create test event
		let event = EventsSchema {
			id: make_thing_from_enum("events", &uuid::Uuid::new_v4().to_string()),
			name: "Test Event for Already Deleted".to_string(),
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

		// Soft delete the event twice
		let _ = repo.query_delete_event(event_id.clone()).await;
		let result = repo.query_delete_event(event_id).await;

		// Verify second deletion fails
		assert!(result.is_err());
		assert_eq!(result.unwrap_err().to_string(), "Event not found");

		// Clean up - already deleted
	}
}