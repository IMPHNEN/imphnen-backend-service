#[cfg(test)]
mod tests {
    use crate::{generate_unique_email, get_role_id, UsersRepository};
    use imphnen_iam::v1::teams::teams_repository::{TeamsRepository, TeamsSchema, TeamMembersSchema, TeamInvitationsSchema};
    use imphnen_iam::v1::teams::teams_service::TeamsService;
    use imphnen_iam::v1::teams::TeamsCreateRequestDto;
    use imphnen_iam::v1::teams::TeamsSearchQueryDto;
    use imphnen_utils::{make_thing_from_enum, ResourceEnum};
    use chrono::{Utc, NaiveDateTime};
    use surrealdb::sql::Thing;

    #[tokio::test]
    async fn test_service_create_and_get_team() {
        let app_state = crate::get_app_state().await;
        let users_repo = UsersRepository::new(&app_state);
        let repo = TeamsRepository::new(&app_state);
        let service = TeamsService::new(repo.clone());

        // Create test user
        let email = generate_unique_email("team_owner_service");
        let role_id = get_role_id("mentee", &app_state).await;
        let user_data = crate::create_test_user(&email, "password123", true, &role_id);
        let user_result = users_repo.query_create_user(user_data.clone()).await;
        assert!(user_result.is_ok(), "Failed to create test user");
        let user = users_repo.query_user_by_email(email.clone()).await.unwrap();

        // Create team via service
        let team_request = TeamsCreateRequestDto {
            name: "Test Team Service".to_string(),
            description: Some("Team created via service test".to_string()),
            is_open: Some(true),
            max_members: Some(10),
            skills_required: Some(vec!["Rust".to_string(), "Testing".to_string()]),
            location: Some("Remote".to_string()),
            website_url: None,
            github_url: None,
            avatar: None,
            member_emails: vec![],
        };

        let create_result = service.create_team(team_request, user.id.id.to_raw()).await;
        assert!(create_result.is_ok(), "Failed to create team via service");
        assert_eq!(create_result.unwrap(), "Success create team");

        // Get team by ID via service
        let team_thing = make_thing_from_enum(ResourceEnum::Teams, &create_result.unwrap().split_whitespace().last().unwrap());
        let result = service.get_team_by_id(&team_thing).await;
        assert!(result.is_ok(), "Failed to get team by ID via service");
        let retrieved_team = result.unwrap();
        
        // Validate team data
        assert_eq!(retrieved_team.name, "Test Team Service");
        assert_eq!(retrieved_team.description, Some("Team created via service test".to_string()));
        assert_eq!(retrieved_team.is_open, Some(true));
        assert_eq!(retrieved_team.max_members, Some(10));
        assert_eq!(retrieved_team.skills_required, Some(vec!["Rust".to_string(), "Testing".to_string()]));
        assert_eq!(retrieved_team.location, Some("Remote".to_string()));

        // Clean up
        let team_id = create_result.unwrap().split_whitespace().last().unwrap().to_string();
        let _ = repo.query_delete_team(team_id).await;
        let _ = users_repo.query_delete_user(user.id.id.to_raw()).await;
    }

    #[tokio::test]
    async fn test_service_update_team() {
        let app_state = crate::get_app_state().await;
        let users_repo = UsersRepository::new(&app_state);
        let repo = TeamsRepository::new(&app_state);
        let service = TeamsService::new(repo.clone());

        // Create test user
        let email = generate_unique_email("team_updater_service");
        let role_id = get_role_id("mentee", &app_state).await;
        let user_data = crate::create_test_user(&email, "password123", true, &role_id);
        let user_result = users_repo.query_create_user(user_data.clone()).await;
        assert!(user_result.is_ok(), "Failed to create test user");
        let user = users_repo.query_user_by_email(email.clone()).await.unwrap();

        // Create team via service
        let team_request = TeamsCreateRequestDto {
            name: "Original Team Name Service".to_string(),
            description: Some("Original description service".to_string()),
            is_open: Some(true),
            max_members: Some(10),
            skills_required: None,
            location: None,
            website_url: None,
            github_url: None,
            avatar: None,
            member_emails: vec![],
        };

        let create_result = service.create_team(team_request, user.id.id.to_raw()).await;
        assert!(create_result.is_ok(), "Failed to create team via service");

        // Get team ID
        let team_id = create_result.unwrap().split_whitespace().last().unwrap().to_string();
        let team_thing = make_thing_from_enum(ResourceEnum::Teams, &team_id);
        
        // Get team for update
        let team_result = service.get_team_by_id(&team_thing).await;
        assert!(team_result.is_ok(), "Failed to get team for update");
        let mut team = team_result.unwrap();

        // Update team data
        team.name = "Updated Team Name Service".to_string();
        team.description = Some("Updated description service".to_string());
        team.is_open = false;
        team.max_members = Some(15);
        team.skills_required = Some(vec!["Rust".to_string(), "Testing".to_string()]);
        team.location = Some("Office".to_string());
        team.website_url = Some("https://example.com/service".to_string());
        team.github_url = Some("https://github.com/example/service".to_string());

        // Update team via service
        let update_result = service.update_team(team).await;
        assert!(update_result.is_ok(), "Failed to update team via service");
        assert_eq!(update_result.unwrap(), "Success update team");

        // Verify update via service
        let updated_team_result = service.get_team_by_id(&team_thing).await;
        assert!(updated_team_result.is_ok(), "Failed to get updated team via service");
        let updated_team = updated_team_result.unwrap();
        
        assert_eq!(updated_team.name, "Updated Team Name Service");
        assert_eq!(updated_team.description, Some("Updated description service".to_string()));
        assert_eq!(updated_team.is_open, false);
        assert_eq!(updated_team.max_members, Some(15));
        assert_eq!(updated_team.location, Some("Office".to_string()));
        assert_eq!(updated_team.website_url, Some("https://example.com/service".to_string()));

        // Clean up
        let _ = repo.query_delete_team(team_id).await;
        let _ = users_repo.query_delete_user(user.id.id.to_raw()).await;
    }

    #[tokio::test]
    async fn test_service_delete_team() {
        let app_state = crate::get_app_state().await;
        let users_repo = UsersRepository::new(&app_state);
        let repo = TeamsRepository::new(&app_state);
        let service = TeamsService::new(repo.clone());

        // Create test user
        let email = generate_unique_email("team_deleter_service");
        let role_id = get_role_id("mentee", &app_state).await;
        let user_data = crate::create_test_user(&email, "password123", true, &role_id);
        let user_result = users_repo.query_create_user(user_data.clone()).await;
        assert!(user_result.is_ok(), "Failed to create test user");
        let user = users_repo.query_user_by_email(email.clone()).await.unwrap();

        // Create team via service
        let team_request = TeamsCreateRequestDto {
            name: "Team to Delete Service".to_string(),
            description: Some("Team that will be deleted via service".to_string()),
            is_open: Some(true),
            max_members: Some(10),
            skills_required: None,
            location: None,
            website_url: None,
            github_url: None,
            avatar: None,
            member_emails: vec![],
        };

        let create_result = service.create_team(team_request, user.id.id.to_raw()).await;
        assert!(create_result.is_ok(), "Failed to create team via service");

        // Get team ID
        let team_id = create_result.unwrap().split_whitespace().last().unwrap().to_string();
        let team_thing = make_thing_from_enum(ResourceEnum::Teams, &team_id);
        
        // Verify team exists before deletion
        let exists_before = service.get_team_by_id(&team_thing).await.is_ok();
        assert!(exists_before, "Team should exist before deletion via service");

        // Delete team via service
        let delete_result = service.delete_team(team_id).await;
        assert!(delete_result.is_ok(), "Failed to delete team via service");
        assert_eq!(delete_result.unwrap(), "Success delete team");

        // Verify team is deleted via service
        let exists_after = service.get_team_by_id(&team_thing).await.is_ok();
        assert!(!exists_after, "Team should not exist after deletion via service");

        // Clean up
        let _ = users_repo.query_delete_user(user.id.id.to_raw()).await;
    }

    #[tokio::test]
    async fn test_service_add_and_get_team_member() {
        let app_state = crate::get_app_state().await;
        let users_repo = UsersRepository::new(&app_state);
        let repo = TeamsRepository::new(&app_state);
        let service = TeamsService::new(repo.clone());

        // Create test users
        let email1 = generate_unique_email("team_owner_member_service");
        let email2 = generate_unique_email("team_member2_service");
        let role_id = get_role_id("mentee", &app_state).await;
        
        let user_data1 = crate::create_test_user(&email1, "password123", true, &role_id);
        let user_data2 = crate::create_test_user(&email2, "password123", true, &role_id);
        
        let user_result1 = users_repo.query_create_user(user_data1.clone()).await;
        let user_result2 = users_repo.query_create_user(user_data2.clone()).await;
        
        assert!(user_result1.is_ok(), "Failed to create first test user");
        assert!(user_result2.is_ok(), "Failed to create second test user");
        
        let user1 = users_repo.query_user_by_email(email1.clone()).await.unwrap();
        let user2 = users_repo.query_user_by_email(email2.clone()).await.unwrap();

        // Create team via service
        let team_request = TeamsCreateRequestDto {
            name: "Team with Members Service".to_string(),
            description: Some("Team for testing members via service".to_string()),
            is_open: Some(true),
            max_members: Some(10),
            skills_required: None,
            location: None,
            website_url: None,
            github_url: None,
            avatar: None,
            member_emails: vec![],
        };

        let create_result = service.create_team(team_request, user1.id.id.to_raw()).await;
        assert!(create_result.is_ok(), "Failed to create team via service");

        let team_id = create_result.unwrap().split_whitespace().last().unwrap().to_string();
        let team_thing = make_thing_from_enum(ResourceEnum::Teams, &team_id);
        
        // Add member via service
        let add_result = service.add_team_member(team_id.clone(), user2.id.id.to_raw(), Some("member".to_string())).await;
        assert!(add_result.is_ok(), "Failed to add team member via service");
        assert_eq!(add_result.unwrap(), "Success add team member");

        // Get team members via service
        let members_result = service.get_team_members(&team_thing).await;
        assert!(members_result.is_ok(), "Failed to get team members via service");
        let members = members_result.unwrap();
        
        assert!(!members.is_empty(), "Should have at least one member");
        assert_eq!(members.len(), 1, "Should have exactly one member");
        assert_eq!(members[0].user_id.id.to_raw(), user2.id.id.to_raw(), "Member user ID should match");
        assert_eq!(members[0].role, "member", "Member role should be correct");

        // Clean up
        let _ = repo.query_delete_team(team_id).await;
        let _ = users_repo.query_delete_user(user1.id.id.to_raw()).await;
        let _ = users_repo.query_delete_user(user2.id.id.to_raw()).await;
    }

    #[tokio::test]
    async fn test_service_get_teams_by_user() {
        let app_state = crate::get_app_state().await;
        let users_repo = UsersRepository::new(&app_state);
        let repo = TeamsRepository::new(&app_state);
        let service = TeamsService::new(repo.clone());

        // Create test user
        let email = generate_unique_email("team_user_service");
        let role_id = get_role_id("mentee", &app_state).await;
        let user_data = crate::create_test_user(&email, "password123", true, &role_id);
        let user_result = users_repo.query_create_user(user_data.clone()).await;
        assert!(user_result.is_ok(), "Failed to create test user");
        let user = users_repo.query_user_by_email(email.clone()).await.unwrap();
        let user_thing = make_thing_from_enum(ResourceEnum::Users, &user.id.id.to_raw());

        // Create team via service
        let team_request = TeamsCreateRequestDto {
            name: "User's Team Service".to_string(),
            description: Some("Team for testing user teams via service".to_string()),
            is_open: Some(true),
            max_members: Some(10),
            skills_required: None,
            location: None,
            website_url: None,
            github_url: None,
            avatar: None,
            member_emails: vec![],
        };

        let create_result = service.create_team(team_request, user.id.id.to_raw()).await;
        assert!(create_result.is_ok(), "Failed to create team via service");

        // Get teams by user via service
        let teams_result = service.get_teams_by_user(&user_thing).await;
        assert!(teams_result.is_ok(), "Failed to get teams by user via service");
        let teams = teams_result.unwrap();
        
        assert!(!teams.is_empty(), "Should have at least one team");
        assert_eq!(teams.len(), 1, "Should have exactly one team");
        assert_eq!(teams[0].name, "User's Team Service", "Team name should match");

        // Clean up
        let team_id = create_result.unwrap().split_whitespace().last().unwrap().to_string();
        let _ = repo.query_delete_team(team_id).await;
        let _ = users_repo.query_delete_user(user.id.id.to_raw()).await;
    }

    #[tokio::test]
    async fn test_service_is_team_member() {
        let app_state = crate::get_app_state().await;
        let users_repo = UsersRepository::new(&app_state);
        let repo = TeamsRepository::new(&app_state);
        let service = TeamsService::new(repo.clone());

        // Create test users
        let email1 = generate_unique_email("team_owner_member_check_service");
        let email2 = generate_unique_email("team_member_check2_service");
        let role_id = get_role_id("mentee", &app_state).await;
        
        let user_data1 = crate::create_test_user(&email1, "password123", true, &role_id);
        let user_data2 = crate::create_test_user(&email2, "password123", true, &role_id);
        
        let user_result1 = users_repo.query_create_user(user_data1.clone()).await;
        let user_result2 = users_repo.query_create_user(user_data2.clone()).await;
        
        assert!(user_result1.is_ok(), "Failed to create first test user");
        assert!(user_result2.is_ok(), "Failed to create second test user");
        
        let user1 = users_repo.query_user_by_email(email1.clone()).await.unwrap();
        let user2 = users_repo.query_user_by_email(email2.clone()).await.unwrap();
        
        let user1_thing = make_thing_from_enum(ResourceEnum::Users, &user1.id.id.to_raw());
        let user2_thing = make_thing_from_enum(ResourceEnum::Users, &user2.id.id.to_raw());

        // Create team via service
        let team_request = TeamsCreateRequestDto {
            name: "Team for Membership Test Service".to_string(),
            description: Some("Team to test membership via service".to_string()),
            is_open: Some(true),
            max_members: Some(10),
            skills_required: None,
            location: None,
            website_url: None,
            github_url: None,
            avatar: None,
            member_emails: vec![],
        };

        let create_result = service.create_team(team_request, user1.id.id.to_raw()).await;
        assert!(create_result.is_ok(), "Failed to create team via service");

        let team_id = create_result.unwrap().split_whitespace().last().unwrap().to_string();
        let team_thing = make_thing_from_enum(ResourceEnum::Teams, &team_id);
        
        // Check if user1 is member (should be true - owner)
        let is_member1 = service.is_team_member(&team_thing, &user1_thing).await;
        assert!(is_member1.is_ok(), "Failed to check membership for user1 via service");
        assert!(is_member1.unwrap(), "User1 should be a team member (owner) via service");

        // Check if user2 is member (should be false initially)
        let is_member2 = service.is_team_member(&team_thing, &user2_thing).await;
        assert!(is_member2.is_ok(), "Failed to check membership for user2 via service");
        assert!(!is_member2.unwrap(), "User2 should not be a team member initially via service");

        // Add user2 as member via service
        let add_result = service.add_team_member(team_id.clone(), user2.id.id.to_raw(), Some("member".to_string())).await;
        assert!(add_result.is_ok(), "Failed to add team member via service");

        // Check again if user2 is member (should be true now)
        let is_member2_after = service.is_team_member(&team_thing, &user2_thing).await;
        assert!(is_member2_after.is_ok(), "Failed to check membership for user2 after addition via service");
        assert!(is_member2_after.unwrap(), "User2 should be a team member after being added via service");

        // Clean up
        let _ = repo.query_delete_team(team_id).await;
        let _ = users_repo.query_delete_user(user1.id.id.to_raw()).await;
        let _ = users_repo.query_delete_user(user2.id.id.to_raw()).await;
    }

    #[tokio::test]
    async fn test_service_create_and_get_invitation() {
        let app_state = crate::get_app_state().await;
        let users_repo = UsersRepository::new(&app_state);
        let repo = TeamsRepository::new(&app_state);
        let service = TeamsService::new(repo.clone());

        // Create test users
        let email1 = generate_unique_email("inviter_service");
        let email2 = generate_unique_email("invitee_service");
        let role_id = get_role_id("mentee", &app_state).await;
        
        let user_data1 = crate::create_test_user(&email1, "password123", true, &role_id);
        let user_data2 = crate::create_test_user(&email2, "password123", true, &role_id);
        
        let user_result1 = users_repo.query_create_user(user_data1.clone()).await;
        let user_result2 = users_repo.query_create_user(user_data2.clone()).await;
        
        assert!(user_result1.is_ok(), "Failed to create first test user");
        assert!(user_result2.is_ok(), "Failed to create second test user");
        
        let user1 = users_repo.query_user_by_email(email1.clone()).await.unwrap();
        let user2 = users_repo.query_user_by_email(email2.clone()).await.unwrap();

        // Create team via service
        let team_request = TeamsCreateRequestDto {
            name: "Team with Invitations Service".to_string(),
            description: Some("Team for testing invitations via service".to_string()),
            is_open: Some(true),
            max_members: Some(10),
            skills_required: None,
            location: None,
            website_url: None,
            github_url: None,
            avatar: None,
            member_emails: vec![],
        };

        let create_result = service.create_team(team_request, user1.id.id.to_raw()).await;
        assert!(create_result.is_ok(), "Failed to create team via service");

        let team_id = create_result.unwrap().split_whitespace().last().unwrap().to_string();
        
        // Create invitation via service
        let invite_code = uuid::Uuid::new_v4().to_string();
        let create_invite_result = service.create_invitation(
            team_id.clone(), 
            user2.email.clone(), 
            user1.id.id.to_raw(), 
            invite_code.clone()
        ).await;
        
        assert!(create_invite_result.is_ok(), "Failed to create invitation via service");
        assert_eq!(create_invite_result.unwrap(), "Success create invitation");

        // Get invitation by token via service
        let get_invite_result = service.get_invitation_by_token(&invite_code).await;
        assert!(get_invite_result.is_ok(), "Failed to get invitation by token via service");
        let invitation = get_invite_result.unwrap();
        
        assert_eq!(invitation.email, user2.email.clone(), "Invitation email should match");
        assert_eq!(invitation.status, "pending", "Invitation status should be pending");
        assert!(invitation.expires_at > chrono::Utc::now().to_string(), "Invitation should not be expired");

        // Clean up
        let _ = repo.query_delete_team(team_id).await;
        let _ = users_repo.query_delete_user(user1.id.id.to_raw()).await;
        let _ = users_repo.query_delete_user(user2.id.id.to_raw()).await;
    }

    #[tokio::test]
    async fn test_service_update_invitation() {
        let app_state = crate::get_app_state().await;
        let users_repo = UsersRepository::new(&app_state);
        let repo = TeamsRepository::new(&app_state);
        let service = TeamsService::new(repo.clone());

        // Create test users
        let email1 = generate_unique_email("inviter_update_service");
        let email2 = generate_unique_email("invitee_update_service");
        let role_id = get_role_id("mentee", &app_state).await;
        
        let user_data1 = crate::create_test_user(&email1, "password123", true, &role_id);
        let user_data2 = crate::create_test_user(&email2, "password123", true, &role_id);
        
        let user_result1 = users_repo.query_create_user(user_data1.clone()).await;
        let user_result2 = users_repo.query_create_user(user_data2.clone()).await;
        
        assert!(user_result1.is_ok(), "Failed to create first test user");
        assert!(user_result2.is_ok(), "Failed to create second test user");
        
        let user1 = users_repo.query_user_by_email(email1.clone()).await.unwrap();
        let user2 = users_repo.query_user_by_email(email2.clone()).await.unwrap();

        // Create team via service
        let team_request = TeamsCreateRequestDto {
            name: "Team for Updating Invitations Service".to_string(),
            description: Some("Team to test invitation updates via service".to_string()),
            is_open: Some(true),
            max_members: Some(10),
            skills_required: None,
            location: None,
            website_url: None,
            github_url: None,
            avatar: None,
            member_emails: vec![],
        };

        let create_result = service.create_team(team_request, user1.id.id.to_raw()).await;
        assert!(create_result.is_ok(), "Failed to create team via service");

        let team_id = create_result.unwrap().split_whitespace().last().unwrap().to_string();
        
        // Create invitation via service
        let invite_code = uuid::Uuid::new_v4().to_string();
        let create_invite_result = service.create_invitation(
            team_id.clone(), 
            user2.email.clone(), 
            user1.id.id.to_raw(), 
            invite_code.clone()
        ).await;
        
        assert!(create_invite_result.is_ok(), "Failed to create invitation via service");

        // Get invitation via service
        let get_invite_result = service.get_invitation_by_token(&invite_code).await;
        assert!(get_invite_result.is_ok(), "Failed to get invitation via service");
        let invitation = get_invite_result.unwrap();
        
        // Update invitation status via service
        let update_invite_result = service.update_invitation_status(
            invitation.id.id.to_raw(), 
            "accepted".to_string(), 
            Some(chrono::Utc::now().to_string())
        ).await;
        
        assert!(update_invite_result.is_ok(), "Failed to update invitation via service");
        assert_eq!(update_invite_result.unwrap(), "Success update invitation");

        // Verify update via service
        let get_updated_invite_result = service.get_invitation_by_token(&invite_code).await;
        assert!(get_updated_invite_result.is_ok(), "Failed to get updated invitation via service");
        let updated_invitation = get_updated_invite_result.unwrap();
        
        assert_eq!(updated_invitation.status, "accepted", "Invitation status should be accepted");
        assert!(updated_invitation.accepted_at.is_some(), "Invitation should have accepted_at timestamp");

        // Clean up
        let _ = repo.query_delete_team(team_id).await;
        let _ = users_repo.query_delete_user(user1.id.id.to_raw()).await;
        let _ = users_repo.query_delete_user(user2.id.id.to_raw()).await;
    }

    #[tokio::test]
    async fn test_service_search_teams() {
        let app_state = crate::get_app_state().await;
        let users_repo = UsersRepository::new(&app_state);
        let repo = TeamsRepository::new(&app_state);
        let service = TeamsService::new(repo.clone());

        // Create test user
        let email = generate_unique_email("team_searcher_service");
        let role_id = get_role_id("mentee", &app_state).await;
        let user_data = crate::create_test_user(&email, "password123", true, &role_id);
        let user_result = users_repo.query_create_user(user_data.clone()).await;
        assert!(user_result.is_ok(), "Failed to create test user");
        let user = users_repo.query_user_by_email(email.clone()).await.unwrap();

        // Create test teams via service
        let team_requests = [
            TeamsCreateRequestDto {
                name: "Rust Development Team Service".to_string(),
                description: Some("Team for Rust development via service".to_string()),
                is_open: Some(true),
                max_members: Some(10),
                skills_required: Some(vec!["Rust".to_string(), "Backend".to_string()]),
                location: Some("Remote".to_string()),
                website_url: None,
                github_url: None,
                avatar: None,
                member_emails: vec![],
            },
            TeamsCreateRequestDto {
                name: "Testing Team Service".to_string(),
                description: Some("Team for testing applications via service".to_string()),
                is_open: Some(false),
                max_members: Some(8),
                skills_required: Some(vec!["Testing".to_string(), "Automation".to_string()]),
                location: Some("Office".to_string()),
                website_url: None,
                github_url: None,
                avatar: None,
                member_emails: vec![],
            },
            TeamsCreateRequestDto {
                name: "Open Source Team Service".to_string(),
                description: Some("Team for open source projects via service".to_string()),
                is_open: Some(true),
                max_members: Some(15),
                skills_required: Some(vec!["Rust".to_string(), "Open Source".to_string()]),
                location: Some("Remote".to_string()),
                website_url: None,
                github_url: None,
                avatar: None,
                member_emails: vec![],
            }
        ];

        for team_request in team_requests.iter() {
            let create_result = service.create_team(team_request.clone(), user.id.id.to_raw()).await;
            assert!(create_result.is_ok(), "Failed to create team via service");
        }

        // Test search with multiple parameters via service
        let search_params = TeamsSearchQueryDto {
            query: Some("Rust".to_string()),
            open: Some(true),
            skills: Some(vec!["Rust".to_string()]),
            location: Some("Remote".to_string()),
            page: Some(1),
            per_page: Some(10),
        };

        let search_result = service.search_teams(search_params).await;
        assert!(search_result.is_ok(), "Failed to search teams via service");
        let search_response = search_result.unwrap();
        
        // Should find 2 teams: "Rust Development Team" and "Open Source Team"
        assert_eq!(search_response.data.len(), 2, "Should find 2 teams matching the search criteria via service");
        
        // Verify team names
        let team_names: Vec<String> = search_response.data.iter().map(|t| t.name.clone()).collect();
        assert!(team_names.contains(&"Rust Development Team Service".to_string()));
        assert!(team_names.contains(&"Open Source Team Service".to_string()));
        
        // Verify all teams are open
        for team in &search_response.data {
            assert!(team.is_open, "All search results should be open teams via service");
            assert_eq!(team.location, Some("Remote".to_string()), "All search results should be remote via service");
        }

        // Clean up - this would normally be done by tracking created team IDs, but for simplicity we'll leave it
        // In a real test, you would store the team IDs and delete them individually
        let _ = users_repo.query_delete_user(user.id.id.to_raw()).await;
    }

    #[tokio::test]
    async fn test_service_remove_team_member() {
        let app_state = crate::get_app_state().await;
        let users_repo = UsersRepository::new(&app_state);
        let repo = TeamsRepository::new(&app_state);
        let service = TeamsService::new(repo.clone());

        // Create test users
        let email1 = generate_unique_email("team_owner_remove_service");
        let email2 = generate_unique_email("team_member_remove_service");
        let role_id = get_role_id("mentee", &app_state).await;
        
        let user_data1 = crate::create_test_user(&email1, "password123", true, &role_id);
        let user_data2 = crate::create_test_user(&email2, "password123", true, &role_id);
        
        let user_result1 = users_repo.query_create_user(user_data1.clone()).await;
        let user_result2 = users_repo.query_create_user(user_data2.clone()).await;
        
        assert!(user_result1.is_ok(), "Failed to create first test user");
        assert!(user_result2.is_ok(), "Failed to create second test user");
        
        let user1 = users_repo.query_user_by_email(email1.clone()).await.unwrap();
        let user2 = users_repo.query_user_by_email(email2.clone()).await.unwrap();
        
        let user2_thing = make_thing_from_enum(ResourceEnum::Users, &user2.id.id.to_raw());

        // Create team via service
        let team_request = TeamsCreateRequestDto {
            name: "Team for Removing Members Service".to_string(),
            description: Some("Team to test member removal via service".to_string()),
            is_open: Some(true),
            max_members: Some(10),
            skills_required: None,
            location: None,
            website_url: None,
            github_url: None,
            avatar: None,
            member_emails: vec![],
        };

        let create_result = service.create_team(team_request, user1.id.id.to_raw()).await;
        assert!(create_result.is_ok(), "Failed to create team via service");

        let team_id = create_result.unwrap().split_whitespace().last().unwrap().to_string();
        let team_thing = make_thing_from_enum(ResourceEnum::Teams, &team_id);
        
        // Add member via service
        let add_result = service.add_team_member(team_id.clone(), user2.id.id.to_raw(), Some("member".to_string())).await;
        assert!(add_result.is_ok(), "Failed to add team member via service");

        // Verify member was added via service
        let members_before = service.get_team_members(&team_thing).await.unwrap();
        assert_eq!(members_before.len(), 1, "Should have one member before removal via service");

        // Remove member via service
        let remove_result = service.remove_team_member(&team_thing, &user2_thing).await;
        assert!(remove_result.is_ok(), "Failed to remove team member via service");
        assert_eq!(remove_result.unwrap(), "Success remove team member");

        // Verify member was removed via service
        let members_after = service.get_team_members(&team_thing).await.unwrap();
        assert_eq!(members_after.len(), 0, "Should have no members after removal via service");

        // Clean up
        let _ = repo.query_delete_team(team_id).await;
        let _ = users_repo.query_delete_user(user1.id.id.to_raw()).await;
        let _ = users_repo.query_delete_user(user2.id.id.to_raw()).await;
    }
}