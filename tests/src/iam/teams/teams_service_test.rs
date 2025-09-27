#[cfg(test)]
mod tests {
	use axum::http::StatusCode;
	use imphnen_iam::MetaRequestDto;
	use imphnen_iam::v1::teams::{TeamsSearchQueryDto, teams_service::{TeamsService, TeamsServiceTrait}};
	use uuid::Uuid;

	#[tokio::test]
	async fn test_get_team_list_service() {
		let app_state = crate::get_app_state().await;

		// Get team list through service
		let meta = MetaRequestDto {
			page: Some(1),
			per_page: Some(10),
			..Default::default()
		};
		let response = TeamsService::get_team_list(&app_state, meta).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);
	}

	#[tokio::test]
	async fn test_get_public_team_list_service() {
		let app_state = crate::get_app_state().await;

		// Get public team list through service
		let meta = MetaRequestDto {
			page: Some(1),
			per_page: Some(10),
			..Default::default()
		};
		let response = TeamsService::get_public_team_list(&app_state, meta).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);
	}

	#[tokio::test]
	async fn test_get_team_by_id_service_invalid_uuid() {
		let app_state = crate::get_app_state().await;

		// Use invalid UUID
		let invalid_id = "invalid-uuid".to_string();

		// Get team by ID through service
		let response = TeamsService::get_team_by_id(&app_state, invalid_id).await;

		// Verify response - should fail validation
		assert_eq!(response.status(), StatusCode::BAD_REQUEST);
	}

	#[tokio::test]
	async fn test_get_team_by_id_service_not_found() {
		let app_state = crate::get_app_state().await;

		// Use valid but non-existent UUID
		let non_existent_id = Uuid::new_v4().to_string();

		// Get team by ID through service
		let response = TeamsService::get_team_by_id(&app_state, non_existent_id).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::NOT_FOUND);
	}

	#[tokio::test]
	async fn test_get_public_team_by_id_service_invalid_uuid() {
		let app_state = crate::get_app_state().await;

		// Use invalid UUID
		let invalid_id = "invalid-uuid".to_string();

		// Get public team by ID through service
		let response = TeamsService::get_public_team_by_id(&app_state, invalid_id).await;

		// Verify response - should fail validation
		assert_eq!(response.status(), StatusCode::BAD_REQUEST);
	}

	#[tokio::test]
	async fn test_get_public_team_by_id_service_not_found() {
		let app_state = crate::get_app_state().await;

		// Use valid but non-existent UUID
		let non_existent_id = Uuid::new_v4().to_string();

		// Get public team by ID through service
		let response = TeamsService::get_public_team_by_id(&app_state, non_existent_id).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::NOT_FOUND);
	}

	#[tokio::test]
	async fn test_search_teams_service() {
		let app_state = crate::get_app_state().await;

		// Search teams
		let search_params = TeamsSearchQueryDto {
			query: Some("test".to_string()),
			location: None,
			open: None,
			skills: None,
			page: None,
			per_page: None,
		};
		let response = TeamsService::search_teams(&app_state, search_params).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);
	}

	#[tokio::test]
	async fn test_get_admin_team_list_service() {
		let app_state = crate::get_app_state().await;

		// Get admin team list through service
		let meta = MetaRequestDto {
			page: Some(1),
			per_page: Some(10),
			..Default::default()
		};
		let response = TeamsService::get_admin_team_list(&app_state, meta).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);
	}

	#[tokio::test]
	async fn test_get_admin_team_by_id_service_invalid_uuid() {
		let app_state = crate::get_app_state().await;

		// Use invalid UUID
		let invalid_id = "invalid-uuid".to_string();

		// Get admin team by ID through service
		let response = TeamsService::get_admin_team_by_id(&app_state, invalid_id).await;

		// Verify response - should fail validation
		assert_eq!(response.status(), StatusCode::BAD_REQUEST);
	}

	#[tokio::test]
	async fn test_get_admin_team_by_id_service_not_found() {
		let app_state = crate::get_app_state().await;

		// Use valid but non-existent UUID
		let non_existent_id = Uuid::new_v4().to_string();

		// Get admin team by ID through service
		let response = TeamsService::get_admin_team_by_id(&app_state, non_existent_id).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::NOT_FOUND);
	}

	#[tokio::test]
	async fn test_get_admin_team_members_service_invalid_uuid() {
		let app_state = crate::get_app_state().await;

		// Use invalid UUID
		let invalid_id = "invalid-uuid".to_string();

		// Get admin team members through service
		let response = TeamsService::get_admin_team_members(&app_state, invalid_id).await;

		// Verify response - should fail validation
		assert_eq!(response.status(), StatusCode::BAD_REQUEST);
	}
}