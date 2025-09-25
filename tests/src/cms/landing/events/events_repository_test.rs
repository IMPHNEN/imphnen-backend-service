#[cfg(test)]
mod tests {
	use imphnen_cms::{
		v1::landing::events::{
			events_repository::EventsRepository,
			events_schema::EventsSchema,
		},
	};
	use imphnen_utils::make_thing_from_enum;

	#[tokio::test]
	async fn test_query_event_list() {
		let app_state = crate::get_app_state().await;
		let repo = EventsRepository::new(&app_state);
		let result = repo.query_event_list(crate::get_meta_request_dto(1, 10)).await;
		assert!(result.is_ok());
	}

	#[tokio::test]
	async fn test_query_event_by_id_not_found() {
		let app_state = crate::get_app_state().await;
		let repo = EventsRepository::new(&app_state);
		let result = repo.query_event_by_id("non-existent-uuid-123456789".to_string()).await;
		assert!(result.is_err());
	}

	#[tokio::test]
	async fn test_query_create_event() {
		let app_state = crate::get_app_state().await;
		let repo = EventsRepository::new(&app_state);
		let event = EventsSchema {
			id: make_thing_from_enum("events", &uuid::Uuid::new_v4().to_string()),
			name: "Test Event".to_string(),
			description: "Test description".to_string(),
			detail_link: Some("https://example.com".to_string()),
			price: Some(100.0),
			is_online: true,
			start_date: "2024-01-01T00:00:00Z".to_string(),
			end_date: "2024-01-02T00:00:00Z".to_string(),
			location: Some("Online".to_string()),
			created_at: chrono::Utc::now().to_rfc3339(),
			updated_at: chrono::Utc::now().to_rfc3339(),
			is_deleted: false,
		};
		let result = repo.query_create_event(event).await;
		assert!(result.is_ok());
	}

	#[tokio::test]
	async fn test_query_update_event_not_found() {
		let app_state = crate::get_app_state().await;
		let repo = EventsRepository::new(&app_state);
		let event = EventsSchema {
			id: make_thing_from_enum("events", &"non-existent-uuid-123456789".to_string()),
			name: "Updated Event".to_string(),
			description: "Updated description".to_string(),
			detail_link: Some("https://example.com/updated".to_string()),
			price: Some(150.0),
			is_online: false,
			start_date: "2024-02-01T00:00:00Z".to_string(),
			end_date: "2024-02-02T00:00:00Z".to_string(),
			location: Some("Offline".to_string()),
			created_at: chrono::Utc::now().to_rfc3339(),
			updated_at: chrono::Utc::now().to_rfc3339(),
			is_deleted: false,
		};
		let result = repo.query_update_event(event).await;
		assert!(result.is_err());
	}

	#[tokio::test]
	async fn test_query_delete_event_not_found() {
		let app_state = crate::get_app_state().await;
		let repo = EventsRepository::new(&app_state);
		let result = repo.query_delete_event("non-existent-uuid-123456789".to_string()).await;
		assert!(result.is_err());
	}
}