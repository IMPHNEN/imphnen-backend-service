#[cfg(test)]
mod tests {
	use crate::get_meta_request_dto;
	use imphnen_cms::{
		v1::landing::events::{
			events_controller::EventsController,
			events_dto::{EventsCreateRequestDto, EventsUpdateRequestDto},
		},
	};

	#[tokio::test]
	async fn test_get_event_list_controller() {
		let app_state = crate::get_app_state().await;
		let response = EventsController::get_event_list(&app_state, get_meta_request_dto(1, 10)).await;
		assert_eq!(response.status(), 200);
	}

	#[tokio::test]
	async fn test_get_event_by_id_controller_not_found() {
		let app_state = crate::get_app_state().await;
		let response = EventsController::get_event_by_id(&app_state, "non-existent-uuid-123456789".to_string()).await;
		assert_eq!(response.status(), 404);
	}

	#[tokio::test]
	async fn test_create_event_controller() {
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
		let response = EventsController::create_event(&app_state, event_request).await;
		assert_eq!(response.status(), 201);
	}

	#[tokio::test]
	async fn test_create_event_controller_invalid_data() {
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
		let response = EventsController::create_event(&app_state, event_request).await;
		assert_eq!(response.status(), 400);
	}

	#[tokio::test]
	async fn test_update_event_controller_not_found() {
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
		let response = EventsController::update_event(&app_state, "non-existent-uuid-123456789".to_string(), update_request).await;
		assert_eq!(response.status(), 400);
	}

	#[tokio::test]
	async fn test_delete_event_controller_not_found() {
		let app_state = crate::get_app_state().await;
		let response = EventsController::delete_event(&app_state, "non-existent-uuid-123456789".to_string()).await;
		assert_eq!(response.status(), 400);
	}
}