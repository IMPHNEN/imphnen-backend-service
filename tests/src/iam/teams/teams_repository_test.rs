#[cfg(test)]
mod tests {
    use crate::{generate_unique_email, get_role_id, UsersRepository};
    use imphnen_iam::{
        TeamsCreateRequestDto, TeamsSchema, TeamMembersSchema, TeamInvitationsSchema,
        TeamsRepository, TeamsSearchQueryDto
    };
    use imphnen_utils::{make_thing_from_enum, ResourceEnum};
    use chrono::{Utc, NaiveDateTime};
    use surrealdb::sql::Thing;

    #[tokio::test]
    async fn test_create_and_get_team() {
        let app_state = crate::get_app_state().await;
        let users_repo = UsersRepository::new(&app_state);
        let repo = TeamsRepository::new(&app_state);

        // Create test user
        let email = generate_unique_email("team_owner");
        let role_id = get_role_id("mentee", &app_state).await;
        let user_data = crate::create_test_user(&email, "password123", true, &role_id);
        let user_result = users_repo.query_create_user(user_data.clone()).await;
        assert!(user_result.is_ok(), "Failed to create test user");
        let user = users_repo.query_user_by_email(email.clone()).await.unwrap();
        let user_thing = make_thing_from_enum(ResourceEnum::Users, &user.id.id.to_raw());

        // Create team
        let team_request = TeamsCreateRequestDto {
            name: "Test Team Repository".to_string(),
            description: Some("Team created via repository test".to_string()),
            is_open: Some(true),
            max_members: Some(10),
            skills_required: Some(vec!["Rust".to_string(), "Testing".to_string()]),
            location: Some("Remote".to_string()),
            website_url: None,
            github_url: None,
            avatar: None,
            member_emails: vec![],
        };

        let team_schema = TeamsSchema::create(team_request, &user.id.id.to_raw());
        let create_result = repo.query_create_team(team_schema.clone()).await;
        assert!(create_result.is_ok(), "Failed to create team");
        assert_eq!(create_result.unwrap(), "Success create team");

        // Get team by ID
        let team_thing = make_thing_from_enum(ResourceEnum::Teams, &team_schema.id.id.to_raw());
        let result = repo.query_team_by_id(&team_thing).await;
        assert!(result.is_ok(), "Failed to get team by ID");
        let retrieved_team = result.unwrap();
        
        // Validate team data
        assert_eq!(retrieved_team.name, team_schema.name);
        assert_eq!(retrieved_team.description, team_schema.description);
        assert_eq!(retrieved_team.is_open, team_schema.is_open);
        assert_eq!(retrieved_team.max_members, team_schema.max_members);
        assert_eq!(retrieved_team.skills_required, team_schema.skills_required);
        assert_eq!(retrieved_team.location, team_schema.location);
        assert_eq!(retrieved_team.avatar, team_schema.avatar);
        assert_eq!(retrieved_team.website_url, team_schema.website_url);
        assert_eq!(retrieved_team.github_url, team_schema.github_url);
        assert_eq!(retrieved_team.is_active, true);
        assert_eq!(retrieved_team.is_deleted, false);

        // Clean up
        let _ = repo.query_delete_team(team_schema.id.id.to_raw()).await;
        let _ = users_repo.query_delete_user(user.id.id.to_raw()).await;
    }

    #[tokio::test]
    async fn test_update_team() {
        let app_state = crate::get_app_state().await;
        let users_repo = UsersRepository::new(&app_state);
        let repo = TeamsRepository::new(&app_state);

        // Create test user
        let email = generate_unique_email("team_updater");
        let role_id = get_role_id("mentee", &app_state).await;
        let user_data = crate::create_test_user(&email, "password123", true, &role_id);
        let user_result = users_repo.query_create_user(user_data.clone()).await;
        assert!(user_result.is_ok(), "Failed to create test user");
        let user = users_repo.query_user_by_email(email.clone()).await.unwrap();

        // Create team
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

        let team_schema = TeamsSchema::create(team_request, user.id.id.to_raw());
        let create_result = repo.query_create_team(team_schema.clone()).await;
        assert!(create_result.is_ok(), "Failed to create team");

        // Update team
        let mut updated_team = team_schema.clone();
        updated_team.name = "Updated Team Name".to_string();
        updated_team.description = Some("Updated description".to_string());
        updated_team.is_open = false;
        updated_team.max_members = Some(15);
        updated_team.skills_required = Some(vec!["Rust".to_string(), "Testing".to_string()]);
        updated_team.location = Some("Office".to_string());
        updated_team.website_url = Some("https://example.com".to_string());
        updated_team.github_url = Some("https://github.com/example".to_string());
        
        let update_result = repo.query_update_team(updated_team).await;
        assert!(update_result.is_ok(), "Failed to update team");
        assert_eq!(update_result.unwrap(), "Success update team");

        // Verify update
        let team_thing = make_thing_from_enum(ResourceEnum::Teams, &team_schema.id.id.to_raw());
        let result = repo.query_team_by_id(&team_thing).await;
        assert!(result.is_ok(), "Failed to get updated team");
        let retrieved_team = result.unwrap();
        
        assert_eq!(retrieved_team.name, "Updated Team Name");
        assert_eq!(retrieved_team.description, Some("Updated description".to_string()));
        assert_eq!(retrieved_team.is_open, false);
        assert_eq!(retrieved_team.max_members, Some(15));
        assert_eq!(retrieved_team.skills_required, Some(vec!["Rust".to_string(), "Testing".to_string()]));
        assert_eq!(retrieved_team.location, Some("Office".to_string()));
        assert_eq!(retrieved_team.website_url, Some("https://example.com".to_string()));
        assert_eq!(retrieved_team.github_url, Some("https://github.com/example".to_string()));

        // Clean up
        let _ = repo.query_delete_team(team_schema.id.id.to_raw()).await;
        let _ = users_repo.query_delete_user(user.id.id.to_raw()).await;
    }

    #[tokio::test]
    async fn test_delete_team() {
        let app_state = crate::get_app_state().await;
        let users_repo = UsersRepository::new(&app_state);
        let repo = TeamsRepository::new(&app_state);

        // Create test user
        let email = generate_unique_email("team_deleter");
        let role_id = get_role_id("mentee", &app_state).await;
        let user_data = crate::create_test_user(&email, "password123", true, &role_id);
        let user_result = users_repo.query_create_user(user_data.clone()).await;
        assert!(user_result.is_ok(), "Failed to create test user");
        let user = users_repo.query_user_by_email(email.clone()).await.unwrap();

        // Create team
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

        let team_schema = TeamsSchema::create(team_request, user.id.id.to_raw());
        let create_result = repo.query_create_team(team_schema.clone()).await;
        assert!(create_result.is_ok(), "Failed to create team");

        // Verify team exists before deletion
        let team_thing = make_thing_from_enum(ResourceEnum::Teams, &team_schema.id.id.to_raw());
        let exists_before = repo.query_team_by_id(&team_thing).await.is_ok();
        assert!(exists_before, "Team should exist before deletion");

        // Delete team
        let delete_result = repo.query_delete_team(team_schema.id.id.to_raw()).await;
        assert!(delete_result.is_ok(), "Failed to delete team");
        assert_eq!(delete_result.unwrap(), "Success delete team");

        // Verify team is deleted
        let exists_after = repo.query_team_by_id(&team_thing).await.is_ok();
        assert!(!exists_after, "Team should not exist after deletion");

        // Clean up
        let _ = users_repo.query_delete_user(user.id.id.to_raw()).await;
    }

    #[tokio::test]
    async fn test_add_and_get_team_member() {
        let app_state = crate::get_app_state().await;
        let users_repo = UsersRepository::new(&app_state);
        let repo = TeamsRepository::new(&app_state);

        // Create test users
        let email1 = generate_unique_email("team_member1");
        let email2 = generate_unique_email("team_member2");
        let role_id = get_role_id("mentee", &app_state).await;
        
        let user_data1 = crate::create_test_user(&email1, "password123", true, &role_id);
        let user_data2 = crate::create_test_user(&email2, "password123", true, &role_id);
        
        let user_result1 = users_repo.query_create_user(user_data1.clone()).await;
        let user_result2 = users_repo.query_create_user(user_data2.clone()).await;
        
        assert!(user_result1.is_ok(), "Failed to create first test user");
        assert!(user_result2.is_ok(), "Failed to create second test user");
        
        let user1 = users_repo.query_user_by_email(email1.clone()).await.unwrap();
        let user2 = users_repo.query_user_by_email(email2.clone()).await.unwrap();

        // Create team
        let team_request = TeamsCreateRequestDto {
            name: "Team with Members".to_string(),
            description: Some("Team for testing members".to_string()),
            is_open: Some(true),
            max_members: Some(10),
            skills_required: None,
            location: None,
            website_url: None,
            github_url: None,
            avatar: None,
            member_emails: vec![],
        };

        let team_schema = TeamsSchema::create(team_request, &user1.id.id.to_raw());
        let create_result = repo.query_create_team(team_schema.clone()).await;
        assert!(create_result.is_ok(), "Failed to create team");

        let team_thing = make_thing_from_enum(ResourceEnum::Teams, &team_schema.id.id.to_raw());
        
        // Add member
        let member_schema = TeamMembersSchema::create(
            team_schema.id.id.to_raw(), 
            user2.id.id.to_raw(), 
            Some("member".to_string())
        );
        
        let add_result = repo.query_add_team_member(member_schema.clone()).await;
        assert!(add_result.is_ok(), "Failed to add team member");
        assert_eq!(add_result.unwrap(), "Success add team member");

        // Get team members
        let members_result = repo.query_team_members(&team_thing).await;
        assert!(members_result.is_ok(), "Failed to get team members");
        let members = members_result.unwrap();
        
        assert!(!members.is_empty(), "Should have at least one member");
        assert_eq!(members.len(), 1, "Should have exactly one member");
        assert_eq!(members[0].user_id.id.to_raw(), user2.id.id.to_raw(), "Member user ID should match");
        assert_eq!(members[0].role, "member", "Member role should be correct");

        // Clean up
        let _ = repo.query_delete_team(team_schema.id.id.to_raw()).await;
        let _ = users_repo.query_delete_user(user1.id.id.to_raw()).await;
        let _ = users_repo.query_delete_user(user2.id.id.to_raw()).await;
    }

    #[tokio::test]
    async fn test_get_teams_by_user() {
        let app_state = crate::get_app_state().await;
        let users_repo = UsersRepository::new(&app_state);
        let repo = TeamsRepository::new(&app_state);

        // Create test user
        let email = generate_unique_email("team_user");
        let role_id = get_role_id("mentee", &app_state).await;
        let user_data = crate::create_test_user(&email, "password123", true, &role_id);
        let user_result = users_repo.query_create_user(user_data.clone()).await;
        assert!(user_result.is_ok(), "Failed to create test user");
        let user = users_repo.query_user_by_email(email.clone()).await.unwrap();
        let user_thing = make_thing_from_enum(ResourceEnum::Users, &user.id.id.to_raw());

        // Create team
        let team_request = TeamsCreateRequestDto {
            name: "User's Team".to_string(),
            description: Some("Team for testing user teams".to_string()),
            is_open: Some(true),
            max_members: Some(10),
            skills_required: None,
            location: None,
            website_url: None,
            github_url: None,
            avatar: None,
            member_emails: vec![],
        };

        let team_schema = TeamsSchema::create(team_request, user.id.id.to_raw());
        let create_result = repo.query_create_team(team_schema.clone()).await;
        assert!(create_result.is_ok(), "Failed to create team");

        // Get teams by user
        let teams_result = repo.query_teams_by_user(&user_thing).await;
        assert!(teams_result.is_ok(), "Failed to get teams by user");
        let teams = teams_result.unwrap();
        
        assert!(!teams.is_empty(), "Should have at least one team");
        assert_eq!(teams.len(), 1, "Should have exactly one team");
        assert_eq!(teams[0].name, "User's Team", "Team name should match");

        // Clean up
        let _ = repo.query_delete_team(team_schema.id.id.to_raw()).await;
        let _ = users_repo.query_delete_user(user.id.id.to_raw()).await;
    }

    #[tokio::test]
    async fn test_is_team_member() {
        let app_state = crate::get_app_state().await;
        let users_repo = UsersRepository::new(&app_state);
        let repo = TeamsRepository::new(&app_state);

        // Create test users
        let email1 = generate_unique_email("team_owner");
        let email2 = generate_unique_email("team_member");
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

        // Create team
        let team_request = TeamsCreateRequestDto {
            name: "Team for Membership Test".to_string(),
            description: Some("Team to test membership".to_string()),
            is_open: Some(true),
            max_members: Some(10),
            skills_required: None,
            location: None,
            website_url: None,
            github_url: None,
            avatar: None,
            member_emails: vec![],
        };

        let team_schema = TeamsSchema::create(team_request, &user1.id.id.to_raw());
        let create_result = repo.query_create_team(team_schema.clone()).await;
        assert!(create_result.is_ok(), "Failed to create team");

        let team_thing = make_thing_from_enum(ResourceEnum::Teams, &team_schema.id.id.to_raw());
        
        // Check if user1 is member (should be true - owner)
        let is_member1 = repo.query_is_team_member(&team_thing, &user1_thing).await;
        assert!(is_member1.is_ok(), "Failed to check membership for user1");
        assert!(is_member1.unwrap(), "User1 should be a team member (owner)");

        // Check if user2 is member (should be false initially)
        let is_member2 = repo.query_is_team_member(&team_thing, &user2_thing).await;
        assert!(is_member2.is_ok(), "Failed to check membership for user2");
        assert!(!is_member2.unwrap(), "User2 should not be a team member initially");

        // Add user2 as member
        let member_schema = TeamMembersSchema::create(
            team_schema.id.id.to_raw(), 
            user2.id.id.to_raw(), 
            Some("member".to_string())
        );
        
        let add_result = repo.query_add_team_member(member_schema).await;
        assert!(add_result.is_ok(), "Failed to add team member");

        // Check again if user2 is member (should be true now)
        let is_member2_after = repo.query_is_team_member(&team_thing, &user2_thing).await;
        assert!(is_member2_after.is_ok(), "Failed to check membership for user2 after addition");
        assert!(is_member2_after.unwrap(), "User2 should be a team member after being added");

        // Clean up
        let _ = repo.query_delete_team(team_schema.id.id.to_raw()).await;
        let _ = users_repo.query_delete_user(user1.id.id.to_raw()).await;
        let _ = users_repo.query_delete_user(user2.id.id.to_raw()).await;
    }

    #[tokio::test]
    async fn test_create_and_get_invitation() {
        let app_state = crate::get_app_state().await;
        let users_repo = UsersRepository::new(&app_state);
        let repo = TeamsRepository::new(&app_state);

        // Create test users
        let email1 = generate_unique_email("inviter");
        let email2 = generate_unique_email("invitee");
        let role_id = get_role_id("mentee", &app_state).await;
        
        let user_data1 = crate::create_test_user(&email1, "password123", true, &role_id);
        let user_data2 = crate::create_test_user(&email2, "password123", true, &role_id);
        
        let user_result1 = users_repo.query_create_user(user_data1.clone()).await;
        let user_result2 = users_repo.query_create_user(user_data2.clone()).await;
        
        assert!(user_result1.is_ok(), "Failed to create first test user");
        assert!(user_result2.is_ok(), "Failed to create second test user");
        
        let user1 = users_repo.query_user_by_email(email1.clone()).await.unwrap();
        let user2 = users_repo.query_user_by_email(email2.clone()).await.unwrap();

        // Create team
        let team_request = TeamsCreateRequestDto {
            name: "Team with Invitations".to_string(),
            description: Some("Team for testing invitations".to_string()),
            is_open: Some(true),
            max_members: Some(10),
            skills_required: None,
            location: None,
            website_url: None,
            github_url: None,
            avatar: None,
            member_emails: vec![],
        };

        let team_schema = TeamsSchema::create(team_request, &user1.id.id.to_raw());
        let create_result = repo.query_create_team(team_schema.clone()).await;
        assert!(create_result.is_ok(), "Failed to create team");

        // Create invitation
        let invite_code = uuid::Uuid::new_v4().to_string();
        let invitation_schema = TeamInvitationsSchema::create(
            team_schema.id.id.to_raw(), 
            user2.email.clone(), 
            user1.id.id.to_raw(), 
            invite_code.clone()
        );
        
        let create_invite_result = repo.query_create_invitation(invitation_schema.clone()).await;
        assert!(create_invite_result.is_ok(), "Failed to create invitation");
        assert_eq!(create_invite_result.unwrap(), "Success create invitation");

        // Get invitation by token
        let get_invite_result = repo.query_invitation_by_token(&invite_code).await;
        assert!(get_invite_result.is_ok(), "Failed to get invitation by token");
        let invitation = get_invite_result.unwrap();
        
        assert_eq!(invitation.email, user2.email.clone(), "Invitation email should match");
        assert_eq!(invitation.status, "pending", "Invitation status should be pending");
        assert!(invitation.expires_at > chrono::Utc::now().to_string(), "Invitation should not be expired");

        // Clean up
        let _ = repo.query_delete_team(team_schema.id.id.to_raw()).await;
        let _ = users_repo.query_delete_user(user1.id.id.to_raw()).await;
        let _ = users_repo.query_delete_user(user2.id.id.to_raw()).await;
    }

    #[tokio::test]
    async fn test_update_invitation() {
        let app_state = crate::get_app_state().await;
        let users_repo = UsersRepository::new(&app_state);
        let repo = TeamsRepository::new(&app_state);

        // Create test users
        let email1 = generate_unique_email("inviter");
        let email2 = generate_unique_email("invitee");
        let role_id = get_role_id("mentee", &app_state).await;
        
        let user_data1 = crate::create_test_user(&email1, "password123", true, &role_id);
        let user_data2 = crate::create_test_user(&email2, "password123", true, &role_id);
        
        let user_result1 = users_repo.query_create_user(user_data1.clone()).await;
        let user_result2 = users_repo.query_create_user(user_data2.clone()).await;
        
        assert!(user_result1.is_ok(), "Failed to create first test user");
        assert!(user_result2.is_ok(), "Failed to create second test user");
        
        let user1 = users_repo.query_user_by_email(email1.clone()).await.unwrap();
        let user2 = users_repo.query_user_by_email(email2.clone()).await.unwrap();

        // Create team
        let team_request = TeamsCreateRequestDto {
            name: "Team for Updating Invitations".to_string(),
            description: Some("Team to test invitation updates".to_string()),
            is_open: Some(true),
            max_members: Some(10),
            skills_required: None,
            location: None,
            website_url: None,
            github_url: None,
            avatar: None,
            member_emails: vec![],
        };

        let team_schema = TeamsSchema::create(team_request, &user1.id.id.to_raw());
        let create_result = repo.query_create_team(team_schema.clone()).await;
        assert!(create_result.is_ok(), "Failed to create team");

        // Create invitation
        let invite_code = uuid::Uuid::new_v4().to_string();
        let invitation_schema = TeamInvitationsSchema::create(
            team_schema.id.id.to_raw(), 
            user2.email.clone(), 
            user1.id.id.to_raw(), 
            invite_code.clone()
        );
        
        let create_invite_result = repo.query_create_invitation(invitation_schema.clone()).await;
        assert!(create_invite_result.is_ok(), "Failed to create invitation");

        // Get invitation
        let get_invite_result = repo.query_invitation_by_token(&invite_code).await;
        assert!(get_invite_result.is_ok(), "Failed to get invitation");
        let mut invitation = get_invite_result.unwrap();
        
        // Convert to schema for update
        let invitation_schema = TeamInvitationsSchema {
            id: invitation.id,
            team_id: invitation.team_id,
            email: invitation.email,
            inviter_id: invitation.inviter_id,
            invite_code: invitation.invite_code,
            expires_at: invitation.expires_at,
            status: "accepted".to_string(),
            invited_at: invitation.invited_at,
            accepted_at: Some(chrono::Utc::now().to_string()),
        };
        
        let update_invite_result = repo.query_update_invitation(invitation_schema).await;
        assert!(update_invite_result.is_ok(), "Failed to update invitation");
        assert_eq!(update_invite_result.unwrap(), "Success update invitation");

        // Verify update
        let get_updated_invite_result = repo.query_invitation_by_token(&invite_code).await;
        assert!(get_updated_invite_result.is_ok(), "Failed to get updated invitation");
        let updated_invitation = get_updated_invite_result.unwrap();
        
        assert_eq!(updated_invitation.status, "accepted", "Invitation status should be accepted");
        assert!(updated_invitation.accepted_at.is_some(), "Invitation should have accepted_at timestamp");

        // Clean up
        let _ = repo.query_delete_team(team_schema.id.id.to_raw()).await;
        let _ = users_repo.query_delete_user(user1.id.id.to_raw()).await;
        let _ = users_repo.query_delete_user(user2.id.id.to_raw()).await;
    }

    #[tokio::test]
    async fn test_search_teams() {
        let app_state = crate::get_app_state().await;
        let users_repo = UsersRepository::new(&app_state);
        let repo = TeamsRepository::new(&app_state);

        // Create test user
        let email = generate_unique_email("team_searcher");
        let role_id = get_role_id("mentee", &app_state).await;
        let user_data = crate::create_test_user(&email, "password123", true, &role_id);
        let user_result = users_repo.query_create_user(user_data.clone()).await;
        assert!(user_result.is_ok(), "Failed to create test user");
        let user = users_repo.query_user_by_email(email.clone()).await.unwrap();

        // Create test teams
        let team_requests = [
            TeamsCreateRequestDto {
                name: "Rust Development Team".to_string(),
                description: Some("Team for Rust development".to_string()),
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
                name: "Testing Team".to_string(),
                description: Some("Team for testing applications".to_string()),
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
                name: "Open Source Team".to_string(),
                description: Some("Team for open source projects".to_string()),
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

        let mut team_ids = Vec::new();
        
        for team_request in team_requests.iter() {
            let team_schema = TeamsSchema::create(team_request.clone(), user.id.id.to_raw());
            let create_result = repo.query_create_team(team_schema.clone()).await;
            assert!(create_result.is_ok(), "Failed to create team");
            team_ids.push(team_schema.id.id.to_raw());
        }

        // Test search with multiple parameters
        let search_params = TeamsSearchQueryDto {
            query: Some("Rust".to_string()),
            open: Some(true),
            skills: Some(vec!["Rust".to_string()]),
            location: Some("Remote".to_string()),
            page: Some(1),
            per_page: Some(10),
        };

        let search_result = repo.query_search_teams(search_params).await;
        assert!(search_result.is_ok(), "Failed to search teams");
        let search_response = search_result.unwrap();
        
        // Should find 2 teams: "Rust Development Team" and "Open Source Team"
        assert_eq!(search_response.data.len(), 2, "Should find 2 teams matching the search criteria");
        
        // Verify team names
        let team_names: Vec<String> = search_response.data.iter().map(|t| t.name.clone()).collect();
        assert!(team_names.contains(&"Rust Development Team".to_string()));
        assert!(team_names.contains(&"Open Source Team".to_string()));
        
        // Verify all teams are open
        for team in &search_response.data {
            assert!(team.is_open, "All search results should be open teams");
            assert_eq!(team.location, Some("Remote".to_string()), "All search results should be remote");
            assert!(team.skills_required.iter().any(|skills| skills.contains(&"Rust".to_string())), "All search results should require Rust");
        }

        // Clean up
        for team_id in team_ids {
            let _ = repo.query_delete_team(team_id).await;
        }
        let _ = users_repo.query_delete_user(user.id.id.to_raw()).await;
    }

    #[tokio::test]
    async fn test_remove_team_member() {
        let app_state = crate::get_app_state().await;
        let users_repo = UsersRepository::new(&app_state);
        let repo = TeamsRepository::new(&app_state);

        // Create test users
        let email1 = generate_unique_email("team_owner");
        let email2 = generate_unique_email("team_member_to_remove");
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

        // Create team
        let team_request = TeamsCreateRequestDto {
            name: "Team for Removing Members".to_string(),
            description: Some("Team to test member removal".to_string()),
            is_open: Some(true),
            max_members: Some(10),
            skills_required: None,
            location: None,
            website_url: None,
            github_url: None,
            avatar: None,
            member_emails: vec![],
        };

        let team_schema = TeamsSchema::create(team_request, &user1.id.id.to_raw());
        let create_result = repo.query_create_team(team_schema.clone()).await;
        assert!(create_result.is_ok(), "Failed to create team");

        let team_thing = make_thing_from_enum(ResourceEnum::Teams, &team_schema.id.id.to_raw());
        
        // Add member first
        let member_schema = TeamMembersSchema::create(
            team_schema.id.id.to_raw(), 
            user2.id.id.to_raw(), 
            Some("member".to_string())
        );
        
        let add_result = repo.query_add_team_member(member_schema).await;
        assert!(add_result.is_ok(), "Failed to add team member");

        // Verify member was added
        let members_before = repo.query_team_members(&team_thing).await.unwrap();
        assert_eq!(members_before.len(), 1, "Should have one member before removal");

        // Remove member
        let remove_result = repo.query_remove_team_member(&team_thing, &user2_thing).await;
        assert!(remove_result.is_ok(), "Failed to remove team member");
        assert_eq!(remove_result.unwrap(), "Success remove team member");

        // Verify member was removed
        let members_after = repo.query_team_members(&team_thing).await.unwrap();
        assert_eq!(members_after.len(), 0, "Should have no members after removal");

        // Clean up
        let _ = repo.query_delete_team(team_schema.id.id.to_raw()).await;
        let _ = users_repo.query_delete_user(user1.id.id.to_raw()).await;
        let _ = users_repo.query_delete_user(user2.id.id.to_raw()).await;
    }
}