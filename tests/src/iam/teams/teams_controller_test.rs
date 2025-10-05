#[cfg(test)]
mod tests {
	use crate::{generate_unique_email, get_role_id, UsersRepository};
	use axum::{http::StatusCode, response::Response};
	use imphnen_iam::{
		TeamsCreateRequestDto, TeamsSearchQueryDto, TeamsSchema, TeamMembersSchema, TeamsRepository
	};
	use imphnen_utils::{make_thing_from_enum, ResourceEnum};

	#[tokio::test]
	async fn test_create_team_controller() {
		let app_state = crate::get_app_state().await;
		let users_repo = UsersRepository::new(&app_state);
		let repo = TeamsRepository::new(&app_state);

		let email = generate_unique_email("team_creator");
		let role_id = get_role_id("mentee", &app_state).await;

		let user_data = crate::create_test_user(&email, "password123", true, &role_id);
		let user_result = users_repo.query_create_user(user_data.clone()).await;
		assert!(user_result.is_ok(), "Failed to create test user");
		let user = users_repo.query_user_by_email(email.clone()).await.unwrap();

		let team_request = TeamsCreateRequestDto {
			name: "Test Controller Team".to_string(),
			description: Some("Team created via controller".to_string()),
			is_open: Some(true),
			max_members: Some(10),
			skills_required: Some(vec!["Rust".to_string(), "Testing".to_string()]),
			location: Some("Remote".to_string()),
			website_url: None,
			github_url: None,
			avatar: None,
			member_emails: vec![],
		};

		// Create team through controller
		let response = imphnen_iam::TeamsController::create_team(
			&app_state, user.id.id.to_raw(), team_request.clone()
		).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::CREATED);

		// Expect a response with data containing the created team id and stats
		let v = crate::common::response_helpers::parse_response_value(response, 2048).await;
		// If wrapped in { "data": ... }, extract
		let data = if let Some(d) = v.get("data") { d.clone() } else { v };
		assert!(data.get("team_id").is_some(), "create response must include team_id");
		assert!(data.get("invitations_sent").is_some(), "create response should report invitations_sent");

		// Verify team was created in database
		let team_thing = make_thing_from_enum(ResourceEnum::Teams, &user.id.id.to_raw());
		let teams = repo.query_user_teams(&team_thing).await.unwrap();
		assert!(!teams.is_empty());
		assert_eq!(teams[0].name, "Test Controller Team");

		// Clean up
		let team_id = teams[0].id.id.to_raw();
		let _ = repo.query_delete_team(team_id).await;
		let _ = users_repo.query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_get_team_controller() {
		let app_state = crate::get_app_state().await;
		let users_repo = UsersRepository::new(&app_state);
		let repo = TeamsRepository::new(&app_state);

		let email = generate_unique_email("team_getter");
		let role_id = get_role_id("mentee", &app_state).await;

		let user_data = crate::create_test_user(&email, "password123", true, &role_id);
		let user_result = users_repo.query_create_user(user_data.clone()).await;
		assert!(user_result.is_ok(), "Failed to create test user");
		let user = users_repo.query_user_by_email(email.clone()).await.unwrap();

		let team_request = TeamsCreateRequestDto {
			name: "Test Get Team".to_string(),
			description: Some("Team for testing retrieval".to_string()),
			is_open: Some(true),
			max_members: Some(10),
			skills_required: None,
			location: None,
			website_url: None,
			github_url: None,
			avatar: None,
			member_emails: vec![],
		};

		// Create team directly for testing
		let team_schema = TeamsSchema::create(team_request, user.id.id.to_raw());
		let create_result = repo.query_create_team(team_schema.clone()).await;
		assert!(create_result.is_ok(), "Failed to create team");

		let team_thing = make_thing_from_enum(ResourceEnum::Teams, &team_schema.id.id.to_raw());
		let team = repo.query_team_by_id(&team_thing).await.unwrap();
		let team_id = team_schema.id.id.to_raw();

		// Get team by ID through controller
		let response = imphnen_iam::TeamsController::get_team(
			&app_state, team_id.clone()
		).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);

		let v = crate::common::response_helpers::parse_response_value(response, 2048).await;
		let inner = v.get("data").expect("get team should return data field").clone();
		let team: imphnen_iam::v1::teams::teams_dto::TeamsDetailResponseDto =
			serde_json::from_value(inner).expect("response data must deserialize to TeamsDetailResponseDto");
		assert_eq!(team.name, "Test Get Team");

		// Clean up
		let _ = repo.query_delete_team(team_id).await;
		let _ = users_repo.query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_get_team_controller_not_found() {
		let app_state = crate::get_app_state().await;

		// Use non-existent ID
		let non_existent_id = "non-existent-uuid-123456789".to_string();

		// Get non-existent team by ID through controller
		let response = imphnen_iam::TeamsController::get_team(
			&app_state, non_existent_id
		).await;

		// Verify not found response
		assert_eq!(response.status(), StatusCode::NOT_FOUND);

		let err: imphnen_entities::MessageResponseDto =
			crate::common::response_helpers::parse_response(response, 1024).await;
		assert!(err.message.to_lowercase().contains("not found") || err.message.to_lowercase().contains("team not found"));
	}
}

	#[tokio::test]
	async fn test_update_team_controller() {
		let app_state = crate::get_app_state().await;
		let users_repo = UsersRepository::new(&app_state);
		let repo = TeamsRepository::new(&app_state);

		let email = generate_unique_email("team_updater");
		let role_id = get_role_id("mentee", &app_state).await;

		let user_data = crate::create_test_user(&email, "password123", true, &role_id);
		let user_result = users_repo.query_create_user(user_data.clone()).await;
		assert!(user_result.is_ok(), "Failed to create test user");
		let user = users_repo.query_user_by_email(email.clone()).await.unwrap();

		let team_request = TeamsCreateRequestDto {
			name: "Original Team Name".to_string(),
			description: Some("Original description".to_string()),
			is_open: Some(true),
			max_members: Some(10),
			skills_required: None,
			location: None,
			website_url: None,
			github_url: None,
			avatar: None,
			member_emails: vec![],
		};

		// Create team directly for testing
		let team_schema = TeamsSchema::create(team_request, user.id.id.to_raw());
		let create_result = repo.query_create_team(team_schema.clone()).await;
		assert!(create_result.is_ok(), "Failed to create team");

		let team_thing = make_thing_from_enum(ResourceEnum::Teams, &team_schema.id.id.to_raw());
		let original_team = repo.query_team_by_id(&team_thing).await.unwrap();
		let team_id = team_schema.id.id.to_raw();

		// Prepare update request
		let update_request = imphnen_iam::TeamsUpdateRequestDto {
			name: Some("Updated Team Name".to_string()),
			description: Some("Updated description".to_string()),
			is_open: Some(false),
			max_members: Some(15),
			skills_required: Some(vec!["Rust".to_string(), "Testing".to_string()]),
			location: Some("Office".to_string()),
			website_url: Some("https://example.com".to_string()),
			github_url: Some("https://github.com/example".to_string()),
		};

		// Update team through controller
		let response = imphnen_iam::TeamsController::update_team(
			&app_state, user.id.id.to_raw(), update_request, team_id.clone()
		).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);

		let msg: imphnen_entities::MessageResponseDto =
			crate::common::response_helpers::parse_response(response, 2048).await;
		assert!(msg.message.to_lowercase().contains("updated") || msg.message.to_lowercase().contains("success"));

		// Verify team was updated in database
		let updated_team = repo.query_team_by_id(&team_thing).await.unwrap();
		assert_eq!(updated_team.name, "Updated Team Name");
		assert_eq!(updated_team.description, Some("Updated description".to_string()));
		assert_eq!(updated_team.is_open, false);
		assert_eq!(updated_team.max_members, Some(15));
		assert_eq!(updated_team.skills_required, Some(vec!["Rust".to_string(), "Testing".to_string()]));
		assert_eq!(updated_team.location, Some("Office".to_string()));
		assert_eq!(updated_team.website_url, Some("https://example.com".to_string()));
		assert_eq!(updated_team.github_url, Some("https://github.com/example".to_string()));

		// Clean up
		let _ = repo.query_delete_team(team_id).await;
		let _ = users_repo.query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_delete_team_controller() {
		let app_state = crate::get_app_state().await;
		let users_repo = UsersRepository::new(&app_state);
		let repo = TeamsRepository::new(&app_state);

		let email = generate_unique_email("team_deleter");
		let role_id = get_role_id("mentee", &app_state).await;

		let user_data = crate::create_test_user(&email, "password123", true, &role_id);
		let user_result = users_repo.query_create_user(user_data.clone()).await;
		assert!(user_result.is_ok(), "Failed to create test user");
		let user = users_repo.query_user_by_email(email.clone()).await.unwrap();

		let team_request = TeamsCreateRequestDto {
			name: "Team to Delete".to_string(),
			description: Some("Team that will be deleted".to_string()),
			is_open: Some(true),
			max_members: Some(10),
			skills_required: None,
			location: None,
			website_url: None,
			github_url: None,
			avatar: None,
			member_emails: vec![],
		};

		// Create team directly for testing
		let team_schema = TeamsSchema::create(team_request, user.id.id.to_raw());
		let create_result = repo.query_create_team(team_schema.clone()).await;
		assert!(create_result.is_ok(), "Failed to create team");

		let team_thing = make_thing_from_enum(ResourceEnum::Teams, &team_schema.id.id.to_raw());
		let team = repo.query_team_by_id(&team_thing).await.unwrap();
		let team_id = team_schema.id.id.to_raw();

		// Verify team exists before deletion
		let exists_before = repo.query_team_by_id(&team_thing).await.is_ok();
		assert!(exists_before);

		// Delete team through controller
		let response = imphnen_iam::TeamsController::delete_team(
			&app_state, user.id.id.to_raw(), team_id.clone()
		).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);

		let msg: imphnen_entities::MessageResponseDto =
			crate::common::response_helpers::parse_response(response, 1024).await;
		assert!(msg.message.to_lowercase().contains("deleted") || msg.message.to_lowercase().contains("success"));

		// Verify team was deleted from database
		let exists_after = repo.query_team_by_id(&team_thing).await.is_ok();
		assert!(!exists_after);

		// Clean up
		let _ = users_repo.query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_search_teams_controller() {
		let app_state = crate::get_app_state().await;
		let users_repo = UsersRepository::new(&app_state);
		let repo = TeamsRepository::new(&app_state);

		let email = generate_unique_email("team_searcher");
		let role_id = get_role_id("mentee", &app_state).await;

		let user_data = crate::create_test_user(&email, "password123", true, &role_id);
		let user_result = users_repo.query_create_user(user_data.clone()).await;
		assert!(user_result.is_ok(), "Failed to create test user");
		let user = users_repo.query_user_by_email(email.clone()).await.unwrap();

		let team_request = TeamsCreateRequestDto {
			name: "Searchable Test Team".to_string(),
			description: Some("A team for testing search functionality".to_string()),
			is_open: Some(true),
			max_members: Some(8),
			skills_required: Some(vec!["Rust".to_string(), "Testing".to_string()]),
			location: Some("Remote".to_string()),
			website_url: None,
			github_url: None,
			avatar: None,
			member_emails: vec![],
		};

		// Create team directly for testing
		let team_schema = TeamsSchema::create(team_request, user.id.id.to_raw());
		let create_result = repo.query_create_team(team_schema.clone()).await;
		assert!(create_result.is_ok(), "Failed to create team");

		let team_id = team_schema.id.id.to_raw();

		// Prepare search request
		let search_params = TeamsSearchQueryDto {
			query: Some("Searchable".to_string()),
			open: Some(true),
			skills: Some(vec!["Rust".to_string()]),
			location: Some("Remote".to_string()),
			page: Some(1),
			per_page: Some(10),
		};

		// Search teams through controller
		let response = imphnen_iam::TeamsController::search_teams(
			&app_state, search_params
		).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);

		let body_json: serde_json::Value = crate::common::response_helpers::parse_response_value(response, 2048).await;
		// Expect a list wrapper with data -> array
		let list = if let Some(d) = body_json.get("data") { d } else { &body_json };
		assert!(list.is_array(), "search should return array of teams");
		let found = list.as_array().unwrap().iter().any(|item| {
			if let Some(name) = item.get("name") { name == "Searchable Test Team" } else { false }
		});
		assert!(found, "created team should appear in search results");

		// Clean up
		let _ = repo.query_delete_team(team_id).await;
		let _ = users_repo.query_delete_user(user.id.id.to_raw()).await;
	}
}