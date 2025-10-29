#[cfg(test)]
mod tests {
    use crate::{generate_unique_email, get_role_id, UsersRepository};
    use axum::{http::StatusCode, response::Response};
    use imphnen_iam::v1::teams::admin_teams_controller;
    use imphnen_iam::v1::teams::teams_dto::{
        TeamsCreateRequestDto, TeamsUpdateRequestDto, TeamsSearchQueryDto,
        TeamsDetailItemDto, TeamsListItemDto
    };
    use imphnen_iam::v1::teams::teams_repository::{
        TeamsSchema, TeamMembersSchema, TeamsRepository
    };
    use imphnen_entities::{ResponseListSuccessDto, MessageResponseDto};
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
        let response = imphnen_iam::v1::teams::teams_controller::create_team(
            &app_state, user.id.id.to_raw(), team_request.clone()
        ).await;

        // Verify response
        assert_eq!(response.status(), StatusCode::CREATED);

        // Parse and verify response contains team data
        let team_response: imphnen_entities::ResponseSuccessDto<TeamsDetailItemDto> =
            crate::common::response_helpers::parse_response(response, 2048).await;
         
        // Validate all required fields in response
        assert!(!team_response.data.id.is_empty(), "Created team must have non-empty id");
        assert_eq!(team_response.data.name, "Test Controller Team");
        assert!(team_response.data.description.is_some(), "Team must have description field");
        assert!(team_response.data.leader.is_some(), "Team must have leader field");
        assert!(team_response.data.is_open, "Team must be open");
        assert!(team_response.data.current_member_count >= 0, "Team must have current_member_count");
        assert!(team_response.data.created_at.is_some(), "Team must have created_at timestamp");

        // Verify team was created in database
        let team_thing = make_thing_from_enum(ResourceEnum::Teams, &user.id.id.to_raw());
        let teams = repo.query_teams_by_user(&team_thing).await.unwrap();
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
        let response = imphnen_iam::v1::teams::teams_controller::get_team(
            &app_state, team_id.clone()
        ).await;

        // Verify response
        assert_eq!(response.status(), StatusCode::OK);

        let team: imphnen_entities::ResponseSuccessDto<TeamsDetailItemDto> =
            crate::common::response_helpers::parse_response(response, 2048).await;
         
        // Validate all required fields
        assert!(!team.data.id.is_empty(), "Team must have non-empty id");
        assert_eq!(team.data.name, "Test Get Team");
        assert!(team.data.description.is_some(), "Team must have description field");
        assert!(team.data.leader.is_some(), "Team must have leader field");
        assert!(team.data.is_open, "Team must be open");
        assert!(team.data.current_member_count >= 0, "Team must have current_member_count");
        assert!(team.data.max_members.is_some(), "Team must have max_members field");
        assert!(team.data.skills_required.is_some(), "Team must have skills_required field");
        assert!(team.data.location.is_some(), "Team must have location field");
        assert!(team.data.avatar.is_some(), "Team must have avatar field");
        assert!(team.data.website_url.is_some(), "Team must have website_url field");
        assert!(team.data.github_url.is_some(), "Team must have github_url field");
        assert!(team.data.members.is_some(), "Team must have members field");
        assert!(team.data.is_active, "Team must be active");
        assert!(team.data.created_at.is_some(), "Team must have created_at timestamp");
        assert!(team.data.updated_at.is_some(), "Team must have updated_at timestamp");

        // Validate leader object
        let leader = team.data.leader.as_ref().unwrap();
        assert!(!leader.id.is_empty(), "Leader must have non-empty id");
        assert!(!leader.user_id.is_empty(), "Leader must have non-empty user_id");
        assert!(!leader.fullname.is_empty(), "Leader must have non-empty fullname");
        assert_eq!(leader.role, "leader", "Leader must have leader role");
        assert!(leader.joined_at.is_some(), "Leader must have joined_at timestamp");

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
        let response = imphnen_iam::v1::teams::teams_controller::get_team(
            &app_state, non_existent_id
        ).await;

        // Verify not found response
        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let err: imphnen_entities::MessageResponseDto =
            crate::common::response_helpers::parse_response(response, 1024).await;
        assert!(err.message.to_lowercase().contains("not found") || err.message.to_lowercase().contains("team not found"));
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
        let update_request = imphnen_iam::v1::teams::teams_dto::TeamsUpdateRequestDto {
            name: Some("Updated Team Name".to_string()),
            description: Some("Updated description".to_string()),
            is_open: Some(false),
            max_members: Some(15),
            skills_required: Some(vec!["Rust".to_string(), "Testing".to_string()]),
            location: Some("Office".to_string()),
            website_url: Some("https://example.com".to_string()),
            github_url: Some("https://github.com/example".to_string()),
            avatar: None,
        };

        // Update team through controller
        let response = imphnen_iam::v1::teams::teams_controller::update_team(
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
        let response = teams_controller::delete_team(
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
        let response = imphnen_iam::v1::teams::teams_controller::search_teams(
            &app_state, search_params
        ).await;

        // Verify response
        assert_eq!(response.status(), StatusCode::OK);

        // Parse and verify search results
        let search_response: imphnen_entities::ResponseListSuccessDto<Vec<TeamsListItemDto>> =
            crate::common::response_helpers::parse_response(response, 2048).await;
         
        assert!(!search_response.data.is_empty(), "Search should return at least one team");
        
        // Verify all results have required fields in TeamsListItemDto
        for team in &search_response.data {
            assert!(!team.id.is_empty(), "Search result team must have non-empty id");
            assert!(!team.name.is_empty(), "Search result team must have non-empty name");
            assert!(team.description.is_some(), "Search result team must have description field");
            assert!(team.leader.is_some(), "Search result team must have leader field");
            assert!(team.is_open, "Search result team must be open");
            assert!(team.current_member_count >= 0, "Search result team must have current_member_count");
            assert!(team.max_members.is_some(), "Search result team must have max_members field");
            assert!(team.skills_required.is_some(), "Search result team must have skills_required field");
            assert!(team.location.is_some(), "Search result team must have location field");
            assert!(team.avatar.is_some(), "Search result team must have avatar field");
            assert!(team.created_at.is_some(), "Search result team must have created_at timestamp");
            
            // Validate leader object in search results
            let leader = team.leader.as_ref().unwrap();
            assert!(!leader.id.is_empty(), "Search result leader must have non-empty id");
            assert!(!leader.user_id.is_empty(), "Search result leader must have non-empty user_id");
            assert!(!leader.fullname.is_empty(), "Search result leader must have non-empty fullname");
            assert_eq!(leader.role, "leader", "Search result leader must have leader role");
            assert!(leader.joined_at.is_some(), "Search result leader must have joined_at timestamp");
        }

        // Verify our team is in results
        let found_team = search_response.data.iter().find(|t| t.name == "Searchable Test Team");
        assert!(found_team.is_some(), "Created team should appear in search results");
        assert_eq!(found_team.unwrap().is_open, true, "Found team should be open");

        // Clean up
        let _ = repo.query_delete_team(team_id).await;
        let _ = users_repo.query_delete_user(user.id.id.to_raw()).await;
    }
}