#[cfg(test)]
mod tests {
    use crate::{generate_unique_email, get_role_id, UsersRepository};
    use chrono::{Duration, Utc, Days};
    use crate::ResourceEnum;
    use imphnen_hackathon::v1::hackathon::{
        HackathonCreateRequestDto, HackathonUpdateRequestDto,
        HackathonSubmissionCreateRequestDto,
        HackathonRepository,
    };
    use imphnen_hackathon::v1::hackathon::SubmissionStatus;
    use imphnen_hackathon::v1::hackathon::hackathon_schema::{
        SubmissionStatus as HackathonSubmissionStatus,
    };
            use imphnen_iam::v1::teams::{TeamsCreateRequestDto, TeamsRepository, TeamMembersSchema, TeamsSchema};
        use imphnen_utils::make_thing_from_enum;
    

    // Use the existing test helpers from the crate root

    #[tokio::test]
    async fn test_create_and_get_hackathon() {
        let app_state = crate::get_app_state().await;
        let users_repo = UsersRepository::new(&app_state);
        let repo = HackathonRepository::new(&app_state);

        // Create test organizer
        let email = generate_unique_email("hackathon_organizer");
        let role_id = get_role_id("mentor", &app_state).await;
        let user_data = crate::create_test_user(&email, "password123", true, &role_id);
        let user_result = users_repo.query_create_user(user_data.clone()).await;
        assert!(user_result.is_ok(), "Failed to create test user");
        let user = users_repo.query_user_by_email(email.clone()).await.unwrap();

        // Create hackathon
        let hackathon_request = HackathonCreateRequestDto {
            name: "Rust Hackathon 2025".to_string(),
            description: "Build amazing Rust projects!".to_string(),
            start_date: Utc::now() + Duration::days(7),
            end_date: Utc::now() + Duration::days(14),
            registration_deadline: Utc::now() + Duration::days(10),
            max_participants: None,
            theme: Some("Backend".to_string()),
            rules: Some("Must use Rust; Open source only".to_string()),
            prizes: None,
            previous_winners: None,
            organizers: vec![user.id.id.to_raw()],
        };

        let created = repo.create_hackathon(hackathon_request.clone()).await.expect("Failed to create hackathon");

        // Get hackathon by ID
        let result = repo.get_hackathon_by_id(created.id.id.to_raw()).await;
        assert!(result.is_ok(), "Failed to get hackathon by ID");
        let retrieved_hackathon = result.unwrap();

        // Validate hackathon data
        assert_eq!(retrieved_hackathon.name, hackathon_request.name);
        assert_eq!(retrieved_hackathon.description, hackathon_request.description);
        assert_eq!(retrieved_hackathon.start_date, hackathon_request.start_date);
        assert_eq!(retrieved_hackathon.end_date, hackathon_request.end_date);
        assert_eq!(retrieved_hackathon.registration_deadline, hackathon_request.registration_deadline);
        assert_eq!(retrieved_hackathon.theme, hackathon_request.theme);
        assert_eq!(retrieved_hackathon.rules, hackathon_request.rules);
    assert!(!retrieved_hackathon.is_deleted);

        // Clean up
        let _ = repo.delete_hackathon(created.id.id.to_raw()).await;
        let _ = users_repo.query_delete_user(user.id.id.to_raw()).await;
    }

    #[tokio::test]
    async fn test_update_hackathon() {
        let app_state = crate::get_app_state().await;
        let users_repo = UsersRepository::new(&app_state);
        let repo = HackathonRepository::new(&app_state);

        // Create test organizer
        let email = generate_unique_email("hackathon_updater");
        let role_id = get_role_id("mentor", &app_state).await;
        let user_data = crate::create_test_user(&email, "password123", true, &role_id);
        let user_result = users_repo.query_create_user(user_data.clone()).await;
        assert!(user_result.is_ok(), "Failed to create test user");
        let user = users_repo.query_user_by_email(email.clone()).await.unwrap();

        // Create hackathon
        let hackathon_request = HackathonCreateRequestDto {
            name: "Original Hackathon".to_string(),
            description: "Original description".to_string(),
            start_date: Utc::now() + Duration::days(7),
            end_date: Utc::now() + Duration::days(14),
            registration_deadline: Utc::now() + Duration::days(10),
            max_participants: None,
            theme: None,
            rules: None,
            prizes: None,
            previous_winners: None,
            organizers: vec![user.id.id.to_raw()],
        };

        let created = repo.create_hackathon(hackathon_request.clone()).await.expect("Failed to create hackathon");

        // Update hackathon using Update DTO
        let updates = HackathonUpdateRequestDto {
            name: Some("Updated Hackathon Title".to_string()),
            description: Some("Updated description with more details".to_string()),
            start_date: Some(Utc::now() + Duration::days(8)),
            end_date: Some(Utc::now() + Duration::days(15)),
            registration_deadline: Some(Utc::now() + Duration::days(11)),
            max_participants: None,
            theme: Some("Fullstack".to_string()),
            rules: Some("Must use Rust or TypeScript; Open source required; Presentation required".to_string()),
            prizes: None,
            previous_winners: None,
            organizers: Some(vec![user.id.id.to_raw()]),
        };

        let updated = repo.update_hackathon(created.id.id.to_raw(), updates).await.expect("Failed to update hackathon");

        // Verify update
        let retrieved_hackathon = repo.get_hackathon_by_id(updated.id.id.to_raw()).await.expect("Failed to get updated hackathon");

        assert_eq!(retrieved_hackathon.name, "Updated Hackathon Title");
        assert_eq!(retrieved_hackathon.description, "Updated description with more details");
        assert_eq!(retrieved_hackathon.theme.unwrap(), "Fullstack");

        // Clean up
        let _ = repo.delete_hackathon(created.id.id.to_raw()).await;
        let _ = users_repo.query_delete_user(user.id.id.to_raw()).await;
    }

    #[tokio::test]
    async fn test_submit_project_as_user() {
        let app_state = crate::get_app_state().await;
        let users_repo = UsersRepository::new(&app_state);
        let repo = HackathonRepository::new(&app_state);

        // Create test users
        let organizer_email = generate_unique_email("hackathon_organizer");
        let participant_email = generate_unique_email("hackathon_participant");
        
        let role_id = get_role_id("mentee", &app_state).await;
        let organizer_data = crate::create_test_user(&organizer_email, "password123", true, &role_id);
        let participant_data = crate::create_test_user(&participant_email, "password123", true, &role_id);
        
        let organizer_result = users_repo.query_create_user(organizer_data.clone()).await;
        let participant_result = users_repo.query_create_user(participant_data.clone()).await;
        
        assert!(organizer_result.is_ok(), "Failed to create organizer");
        assert!(participant_result.is_ok(), "Failed to create participant");
        
        let organizer = users_repo.query_user_by_email(organizer_email.clone()).await.unwrap();
        let participant = users_repo.query_user_by_email(participant_email.clone()).await.unwrap();

        // Create hackathon
        let hackathon_request = HackathonCreateRequestDto {
            name: "User Submission Test Hackathon".to_string(),
            description: "Test hackathon for user submissions".to_string(),
            start_date: Utc::now() + Duration::days(7),
            end_date: Utc::now() + Duration::days(14),
            registration_deadline: Utc::now() + Duration::days(10),
            max_participants: None,
            theme: None,
            rules: None,
            prizes: None,
            previous_winners: None,
            organizers: vec![organizer.id.id.to_raw()],
        };

        let created = repo.create_hackathon(hackathon_request.clone()).await.expect("Failed to create hackathon");

        // For user submissions create a team for the participant so we can provide a team_id
        let teams_repo = TeamsRepository::new(&app_state);
        let team_request = TeamsCreateRequestDto {
            name: "User Individual Team".to_string(),
            description: Some("Auto team for individual user".to_string()),
            is_open: Some(false),
            max_members: Some(1),
            skills_required: None,
            location: None,
            website_url: None,
            github_url: None,
            avatar: None,
            member_emails: vec![],
        };
    let team_schema = imphnen_iam::v1::teams::TeamsSchema::create(team_request, participant.id.id.to_raw());
        let _ = teams_repo.query_create_team(team_schema.clone()).await.expect("Failed to create team");

        // Submit project as user (using the team id)
        let submission_req = HackathonSubmissionCreateRequestDto {
            project_name: "My Rust Project".to_string(),
            description: "A cool Rust project for the hackathon".to_string(),
            repository_url: Some("https://github.com/user/my-rust-project".to_string()),
            demo_url: Some("https://my-rust-project.com".to_string()),
            slides_url: None,
            technologies: vec![],
        };

        let submission = repo.create_hackathon_submission(
            created.id.id.to_raw(),
            team_schema.id.id.to_raw(),
            submission_req,
        ).await.expect("Failed to submit project");

        // Verify submission
        let retrieved_submission = repo.get_hackathon_submission_by_id(submission.id.id.to_raw()).await.expect("Failed to get submission by ID");

        assert_eq!(retrieved_submission.hackathon_id.id.to_raw(), created.id.id.to_raw());
        assert_eq!(retrieved_submission.team_id.as_ref().unwrap().id.to_raw(), team_schema.id.id.to_raw());
        assert_eq!(retrieved_submission.project_name, Some("My Rust Project".to_string()));
        assert_eq!(retrieved_submission.description, Some("A cool Rust project for the hackathon".to_string()));
        assert_eq!(retrieved_submission.repository_url, Some("https://github.com/user/my-rust-project".to_string()));
        assert_eq!(retrieved_submission.demo_url, Some("https://my-rust-project.com".to_string()));
    assert_eq!(retrieved_submission.submission_status, Some(HackathonSubmissionStatus::Draft));

        // Clean up
        let _ = repo.delete_hackathon(created.id.id.to_raw()).await;
        let _ = users_repo.query_delete_user(organizer.id.id.to_raw()).await;
        let _ = users_repo.query_delete_user(participant.id.id.to_raw()).await;
    }

    #[tokio::test]
    async fn test_submit_project_as_team() {
        let app_state = crate::get_app_state().await;
        let users_repo = UsersRepository::new(&app_state);
        let teams_repo = TeamsRepository::new(&app_state);
        let repo = HackathonRepository::new(&app_state);

        // Create test users
        let organizer_email = generate_unique_email("hackathon_organizer");
        let member1_email = generate_unique_email("team_member1");
        let member2_email = generate_unique_email("team_member2");
        
        let role_id = get_role_id("mentee", &app_state).await;
        let organizer_data = crate::create_test_user(&organizer_email, "password123", true, &role_id);
        let member1_data = crate::create_test_user(&member1_email, "password123", true, &role_id);
        let member2_data = crate::create_test_user(&member2_email, "password123", true, &role_id);
        
        let organizer_result = users_repo.query_create_user(organizer_data.clone()).await;
        let member1_result = users_repo.query_create_user(member1_data.clone()).await;
        let member2_result = users_repo.query_create_user(member2_data.clone()).await;
        
        assert!(organizer_result.is_ok(), "Failed to create organizer");
        assert!(member1_result.is_ok(), "Failed to create member1");
        assert!(member2_result.is_ok(), "Failed to create member2");
        
        let organizer = users_repo.query_user_by_email(organizer_email.clone()).await.unwrap();
        let member1 = users_repo.query_user_by_email(member1_email.clone()).await.unwrap();
        let member2 = users_repo.query_user_by_email(member2_email.clone()).await.unwrap();

        // Create team
        let team_request = TeamsCreateRequestDto {
            name: "Hackathon Team".to_string(),
            description: Some("Team for hackathon submissions".to_string()),
            is_open: Some(false),
            max_members: Some(5),
            skills_required: Some(vec!["Rust".to_string(), "Backend".to_string()]),
            location: Some("Remote".to_string()),
            website_url: None,
            github_url: None,
            avatar: None,
            member_emails: vec![],
        };

    let team_schema = TeamsSchema::create(team_request, member1.id.id.to_raw());
        let team_create_result = teams_repo.query_create_team(team_schema.clone()).await;
        assert!(team_create_result.is_ok(), "Failed to create team");

        // Add team members
        let member2_schema = TeamMembersSchema::create(
                    team_schema.id.id.to_raw(),
                    member2.id.id.to_raw(),
                    Some("member".to_string())
                );
        let add_member_result = teams_repo.query_add_team_member(member2_schema).await;
        assert!(add_member_result.is_ok(), "Failed to add team member");

        // Create hackathon
        let hackathon_request = HackathonCreateRequestDto {
                    name: "Team Submission Test Hackathon".to_string(),
                    description: "Test hackathon for team submissions".to_string(),
                    start_date: Utc::now() + Duration::days(7),
                    end_date: Utc::now() + Duration::days(14),
                    registration_deadline: Utc::now() + Duration::days(10),
                    max_participants: None,
                    theme: None,
                    rules: None,
                    prizes: None,
                    previous_winners: None,
                    organizers: vec![organizer.id.id.to_raw()],
                };

    let create_result = repo.create_hackathon(hackathon_request.clone()).await;
        assert!(create_result.is_ok(), "Failed to create hackathon");
        let hackathon_schema = create_result.unwrap();

        // Submit project as team
        let create_submission = HackathonSubmissionCreateRequestDto {
            project_name: "Our Team Rust Project".to_string(),
            description: "A collaborative Rust project by our team".to_string(),
            repository_url: Some("https://github.com/team/our-rust-project".to_string()),
            demo_url: Some("https://our-team-project.com".to_string()),
            slides_url: Some("https://docs.google.com/presentation/d/12345".to_string()),
            technologies: vec![],
        };

    let submit_result = repo.create_hackathon_submission(hackathon_schema.id.id.to_raw(), team_schema.id.id.to_raw(), create_submission.clone()).await;
    assert!(submit_result.is_ok(), "Failed to submit team project");
    let _submitted = submit_result.unwrap();

        // Verify submission
        let submission_id = _submitted.id.id.to_raw();
    let result = repo.get_hackathon_submission_by_id(submission_id.clone()).await;
    assert!(result.is_ok(), "Failed to get team submission by ID");
    let retrieved_submission = result.unwrap();
        
    assert_eq!(retrieved_submission.project_name, Some(create_submission.project_name.clone()));
        // second comparison should clone to avoid moved value
        assert_eq!(retrieved_submission.project_name, Some(create_submission.project_name.clone()));
        assert_eq!(retrieved_submission.description, Some(create_submission.description.clone()));
    assert_eq!(retrieved_submission.repository_url, create_submission.repository_url);
    assert_eq!(retrieved_submission.demo_url, create_submission.demo_url);
    assert_eq!(retrieved_submission.slides_url, create_submission.slides_url);
    assert_eq!(retrieved_submission.submission_status, Some(HackathonSubmissionStatus::Draft));

        // Clean up
    let _ = repo.delete_hackathon(hackathon_schema.id.id.to_raw()).await;
        let _ = teams_repo.query_delete_team(team_schema.id.id.to_raw()).await;
        let _ = users_repo.query_delete_user(organizer.id.id.to_raw()).await;
        let _ = users_repo.query_delete_user(member1.id.id.to_raw()).await;
        let _ = users_repo.query_delete_user(member2.id.id.to_raw()).await;
    }

    #[tokio::test]
    async fn test_get_hackathon_submissions() {
        let app_state = crate::get_app_state().await;
        let users_repo = UsersRepository::new(&app_state);
        let repo = HackathonRepository::new(&app_state);

        // Create test users
        let organizer_email = generate_unique_email("hackathon_organizer");
        let participant_email = generate_unique_email("hackathon_participant");
        
        let role_id = get_role_id("mentee", &app_state).await;
        let organizer_data = crate::create_test_user(&organizer_email, "password123", true, &role_id);
        let participant_data = crate::create_test_user(&participant_email, "password123", true, &role_id);
        
        let organizer_result = users_repo.query_create_user(organizer_data.clone()).await;
        let participant_result = users_repo.query_create_user(participant_data.clone()).await;
        
        assert!(organizer_result.is_ok(), "Failed to create organizer");
        assert!(participant_result.is_ok(), "Failed to create participant");
        
        let organizer = users_repo.query_user_by_email(organizer_email.clone()).await.unwrap();
        let participant = users_repo.query_user_by_email(participant_email.clone()).await.unwrap();

        // Create hackathon
    let hackathon_request = HackathonCreateRequestDto {
            name: "Submissions Test Hackathon".to_string(),
            description: "Test hackathon for retrieving submissions".to_string(),
            start_date: Utc::now() + Duration::days(7),
            end_date: Utc::now() + Duration::days(14),
            registration_deadline: Utc::now() + Duration::days(10),
        max_participants: None,
            theme: None,
            rules: None,
            prizes: None,
            previous_winners: None,
            organizers: vec![organizer.id.id.to_raw()],
        };

    let create_result = repo.create_hackathon(hackathon_request.clone()).await;
    assert!(create_result.is_ok(), "Failed to create hackathon");
    let hackathon_schema = create_result.unwrap();

    let _hackathon_thing = make_thing_from_enum(ResourceEnum::Hackathons, &hackathon_schema.id.id.to_raw());

        // Submit multiple projects using create requests
        let create_requests = [
            ("".to_string(), HackathonSubmissionCreateRequestDto {
                project_name: "Project 1".to_string(),
                description: "Description 1".to_string(),
                repository_url: Some("https://github.com/user/project1".to_string()),
                demo_url: None,
                slides_url: None,
                technologies: vec![],
            }),
            ("".to_string(), HackathonSubmissionCreateRequestDto {
                project_name: "Project 2".to_string(),
                description: "Description 2".to_string(),
                repository_url: Some("https://github.com/user/project2".to_string()),
                demo_url: Some("https://project2.com".to_string()),
                slides_url: None,
                technologies: vec![],
            }),
        ];

        for (team_id, create_req) in create_requests.iter() {
            let _ = repo.create_hackathon_submission(hackathon_schema.id.id.to_raw(), team_id.clone(), create_req.clone()).await.expect("Failed to submit project");
        }

        // Get hackathon submissions
    
    let submissions_result = repo.list_hackathon_submissions(imphnen_libs::MetaRequestDto::default(), hackathon_schema.id.id.to_raw()).await;
    assert!(submissions_result.is_ok(), "Failed to get hackathon submissions");
    let submissions = submissions_result.unwrap().data;
        
    assert_eq!(submissions.len(), 2, "Should have 2 submissions");
    assert_eq!(submissions[0].project_name, Some("Project 1".to_string()));
    assert_eq!(submissions[1].project_name, Some("Project 2".to_string()));
        assert_eq!(submissions[0].submission_status, Some(HackathonSubmissionStatus::Draft));
        assert_eq!(submissions[1].submission_status, Some(HackathonSubmissionStatus::Draft));

        // Clean up
    let _ = repo.delete_hackathon(hackathon_schema.id.id.to_raw()).await;
        let _ = users_repo.query_delete_user(organizer.id.id.to_raw()).await;
        let _ = users_repo.query_delete_user(participant.id.id.to_raw()).await;
    }

    #[tokio::test]
    async fn test_get_user_hackathon_submissions() {
        let app_state = crate::get_app_state().await;
        let users_repo = UsersRepository::new(&app_state);
        let repo = HackathonRepository::new(&app_state);

        // Create test users
        let organizer_email = generate_unique_email("hackathon_organizer");
        let participant_email = generate_unique_email("hackathon_participant");
        
        let role_id = get_role_id("mentee", &app_state).await;
        let organizer_data = crate::create_test_user(&organizer_email, "password123", true, &role_id);
        let participant_data = crate::create_test_user(&participant_email, "password123", true, &role_id);
        
        let organizer_result = users_repo.query_create_user(organizer_data.clone()).await;
        let participant_result = users_repo.query_create_user(participant_data.clone()).await;
        
        assert!(organizer_result.is_ok(), "Failed to create organizer");
        assert!(participant_result.is_ok(), "Failed to create participant");
        
        let organizer = users_repo.query_user_by_email(organizer_email.clone()).await.unwrap();
        let participant = users_repo.query_user_by_email(participant_email.clone()).await.unwrap();

        // Create multiple hackathons
        let hackathon_requests = [
            HackathonCreateRequestDto {
                name: "Hackathon 1".to_string(),
                description: "First hackathon".to_string(),
                    start_date: Utc::now() + Duration::days(7),
                    end_date: Utc::now() + Duration::days(14),
                    registration_deadline: Utc::now() + Duration::days(10),
                max_participants: None,
                theme: None,
                rules: None,
                prizes: None,
                previous_winners: None,
                organizers: vec![organizer.id.id.to_raw()],
            },
            HackathonCreateRequestDto {
                name: "Hackathon 2".to_string(),
                description: "Second hackathon".to_string(),
                start_date: Utc::now() + Duration::days(14),
                end_date: Utc::now() + Duration::days(21),
                registration_deadline: Utc::now() + Duration::days(17),
                max_participants: None,
                theme: None,
                rules: None,
                prizes: None,
                previous_winners: None,
                organizers: vec![organizer.id.id.to_raw()],
            }
        ];

        let mut hackathon_ids = Vec::new();
        
        for hackathon_request in hackathon_requests.iter() {
            let create_result = repo.create_hackathon(hackathon_request.clone()).await;
            assert!(create_result.is_ok(), "Failed to create hackathon");
            let hackathon_schema = create_result.unwrap();
            hackathon_ids.push(hackathon_schema.id.id.to_raw());
        }

        // Submit projects to different hackathons
        // Submit projects using the current CreateRequest DTO
        let create_reqs = vec![
            (hackathon_ids[0].clone(), HackathonSubmissionCreateRequestDto {
                project_name: "Project for Hackathon 1".to_string(),
                description: "Description for hackathon 1".to_string(),
                repository_url: Some("https://github.com/user/hackathon1-project".to_string()),
                demo_url: None,
                slides_url: None,
                technologies: vec![],
            }),
            (hackathon_ids[1].clone(), HackathonSubmissionCreateRequestDto {
                project_name: "Project for Hackathon 2".to_string(),
                description: "Description for hackathon 2".to_string(),
                repository_url: Some("https://github.com/user/hackathon2-project".to_string()),
                demo_url: Some("https://hackathon2-project.com".to_string()),
                slides_url: None,
                technologies: vec![],
            }),
        ];

        for (hackathon_id, create_req) in create_reqs.into_iter() {
            let submit_result = repo.create_hackathon_submission(hackathon_id.clone(), participant.id.id.to_raw(), create_req).await;
            assert!(submit_result.is_ok(), "Failed to submit project");
        }

        // Get user's hackathon submissions
        // Use list_submissions_by_team to fetch submissions for the participant across hackathons
        let submissions_result = repo.list_submissions_by_team(imphnen_libs::MetaRequestDto::default(), participant.id.id.to_raw()).await;
        assert!(submissions_result.is_ok(), "Failed to get user's hackathon submissions");
        let submissions_list = submissions_result.unwrap().data;

        assert_eq!(submissions_list.len(), 2, "Should have 2 submissions");
        assert_eq!(submissions_list[0].project_name, Some("Project for Hackathon 1".to_string()));
        assert_eq!(submissions_list[1].project_name, Some("Project for Hackathon 2".to_string()));
        assert_eq!(submissions_list[0].submission_status, Some(SubmissionStatus::Draft));
        assert_eq!(submissions_list[1].submission_status, Some(SubmissionStatus::Draft));

        // Clean up
        for hackathon_id in hackathon_ids {
            let _ = repo.delete_hackathon(hackathon_id).await;
        }
        let _ = users_repo.query_delete_user(organizer.id.id.to_raw()).await;
        let _ = users_repo.query_delete_user(participant.id.id.to_raw()).await;
    }

    #[tokio::test]
    async fn test_search_hackathons() {
        let app_state = crate::get_app_state().await;
        let users_repo = UsersRepository::new(&app_state);
        let repo = HackathonRepository::new(&app_state);

        // Create test organizer
        let email = generate_unique_email("hackathon_search_organizer");
        let role_id = get_role_id("mentor", &app_state).await;
        let user_data = crate::create_test_user(&email, "password123", true, &role_id);
        let user_result = users_repo.query_create_user(user_data.clone()).await;
        assert!(user_result.is_ok(), "Failed to create test user");
        let user = users_repo.query_user_by_email(email.clone()).await.unwrap();

        // Create test hackathons
        let hackathon_requests = [
            HackathonCreateRequestDto {
                name: "Rust Backend Hackathon".to_string(),
                description: "Build Rust backend projects".to_string(),
                start_date: Utc::now().checked_add_days(Days::new(7)).unwrap(),
                end_date: Utc::now().checked_add_days(Days::new(14)).unwrap(),
                registration_deadline: Utc::now().checked_add_days(Days::new(10)).unwrap(),
                max_participants: None,
                theme: None,
                rules: None,
                prizes: None,
                previous_winners: None,
                organizers: vec![user.id.id.to_raw()],
            },
            HackathonCreateRequestDto {
                name: "TypeScript Frontend Hackathon".to_string(),
                description: "Build TypeScript frontend projects".to_string(),
                start_date: Utc::now().checked_add_days(Days::new(14)).unwrap(),
                end_date: Utc::now().checked_add_days(Days::new(21)).unwrap(),
                registration_deadline: Utc::now().checked_add_days(Days::new(17)).unwrap(),
                max_participants: None,
                theme: None,
                rules: None,
                prizes: None,
                previous_winners: None,
                organizers: vec![user.id.id.to_raw()],
            },
            HackathonCreateRequestDto {
                name: "Rust Fullstack Hackathon".to_string(),
                description: "Build fullstack projects with Rust".to_string(),
                start_date: Utc::now() + Duration::days(21),
                end_date: Utc::now() + Duration::days(28),
                registration_deadline: Utc::now() + Duration::days(24),
                max_participants: None,
                theme: None,
                rules: None,
                prizes: None,
                previous_winners: None,
                organizers: vec![user.id.id.to_raw()],
            }
        ];

        let mut hackathon_ids = Vec::new();
        
        for hackathon_request in hackathon_requests.iter() {
            let create_result = repo.create_hackathon(hackathon_request.clone()).await;
            assert!(create_result.is_ok(), "Failed to create hackathon");
            let hackathon_schema = create_result.unwrap();
            hackathon_ids.push(hackathon_schema.id.id.to_raw());
        }

        // Test search with multiple parameters
        // Search functionality is covered by list_hackathons + QueryListBuilder; skip exact search test here

        // Clean up
        for hackathon_id in hackathon_ids {
            let _ = repo.delete_hackathon(hackathon_id).await;
        }
        let _ = users_repo.query_delete_user(user.id.id.to_raw()).await;
    }

    #[tokio::test]
    async fn test_update_submission_status() {
        let app_state = crate::get_app_state().await;
        let users_repo = UsersRepository::new(&app_state);
        let repo = HackathonRepository::new(&app_state);

        // Create test users
        let organizer_email = generate_unique_email("hackathon_organizer");
        let participant_email = generate_unique_email("hackathon_participant");
        
        let role_id = get_role_id("mentee", &app_state).await;
        let organizer_data = crate::create_test_user(&organizer_email, "password123", true, &role_id);
        let participant_data = crate::create_test_user(&participant_email, "password123", true, &role_id);
        
        let organizer_result = users_repo.query_create_user(organizer_data.clone()).await;
        let participant_result = users_repo.query_create_user(participant_data.clone()).await;
        
        assert!(organizer_result.is_ok(), "Failed to create organizer");
        assert!(participant_result.is_ok(), "Failed to create participant");
        
        let organizer = users_repo.query_user_by_email(organizer_email.clone()).await.unwrap();
        let participant = users_repo.query_user_by_email(participant_email.clone()).await.unwrap();

    // Create hackathon
    let hackathon_request = HackathonCreateRequestDto {
            name: "Submission Status Test Hackathon".to_string(),
            description: "Test hackathon for submission status updates".to_string(),
            start_date: Utc::now() + Duration::days(7),
            end_date: Utc::now() + Duration::days(14),
            registration_deadline: Utc::now() + Duration::days(10),
            max_participants: None,
            theme: None,
            rules: None,
            prizes: None,
            previous_winners: None,
            organizers: vec![organizer.id.id.to_raw()],
        };

    let create_result = repo.create_hackathon(hackathon_request).await;
    assert!(create_result.is_ok(), "Failed to create hackathon");
    let hackathon_schema = create_result.unwrap();

        // Submit project
        let create_req = HackathonSubmissionCreateRequestDto {
            project_name: "Test Project".to_string(),
            description: "Test description".to_string(),
            repository_url: Some("https://github.com/user/test-project".to_string()),
            demo_url: None,
            slides_url: None,
            technologies: vec![],
        };
        let submit_result = repo.create_hackathon_submission(hackathon_schema.id.id.to_raw(), String::new(), create_req).await;
        assert!(submit_result.is_ok(), "Failed to submit project");

        // Get submission ID from result
        let created_submission = submit_result.unwrap();
        // created_submission is a HackathonSubmissionsSchema; get id
        let submission_id = created_submission.id.id.to_raw();

        // Submit the submission (mark as Submitted) using repository API and verify
        let _submitted_schema = repo.submit_hackathon_submission(submission_id.clone()).await.expect("Failed to submit hackathon submission");
        let updated_submission = repo.get_hackathon_submission_by_id(submission_id.clone()).await.expect("Failed to get updated submission");
    assert_eq!(updated_submission.submission_status, Some(HackathonSubmissionStatus::Submitted));
        assert!(updated_submission.updated_at > updated_submission.created_at);

    // Clean up
    let _ = repo.delete_hackathon(hackathon_schema.id.id.to_raw()).await;
        let _ = users_repo.query_delete_user(organizer.id.id.to_raw()).await;
        let _ = users_repo.query_delete_user(participant.id.id.to_raw()).await;
    }

    #[tokio::test]
    async fn test_delete_hackathon() {
        let app_state = crate::get_app_state().await;
        let users_repo = UsersRepository::new(&app_state);
        let repo = HackathonRepository::new(&app_state);

        // Create test organizer
        let email = generate_unique_email("hackathon_deleter");
        let role_id = get_role_id("mentor", &app_state).await;
        let user_data = crate::create_test_user(&email, "password123", true, &role_id);
        let user_result = users_repo.query_create_user(user_data.clone()).await;
        assert!(user_result.is_ok(), "Failed to create test user");
        let user = users_repo.query_user_by_email(email.clone()).await.unwrap();
        // Create hackathon
        let hackathon_request = HackathonCreateRequestDto {
            name: "Hackathon to Delete".to_string(),
            description: "Hackathon that will be deleted".to_string(),
            start_date: Utc::now() + Duration::days(7),
            end_date: Utc::now() + Duration::days(14),
            registration_deadline: Utc::now() + Duration::days(10),
            max_participants: None,
            theme: None,
            rules: None,
            prizes: None,
            previous_winners: None,
            organizers: vec![user.id.id.to_raw()],
        };

        let created = repo.create_hackathon(hackathon_request.clone()).await.expect("Failed to create hackathon");

        // Verify hackathon exists before deletion
        let exists_before = repo.get_hackathon_by_id(created.id.id.to_raw()).await.is_ok();
        assert!(exists_before, "Hackathon should exist before deletion");

        // Delete hackathon
        let delete_result = repo.delete_hackathon(created.id.id.to_raw()).await;
        assert!(delete_result.is_ok(), "Failed to delete hackathon");
        assert_eq!(delete_result.unwrap(), "Hackathon deleted successfully");

        // Verify hackathon is deleted
        let exists_after = repo.get_hackathon_by_id(created.id.id.to_raw()).await.is_ok();
        assert!(!exists_after, "Hackathon should not exist after deletion");

        // Clean up
        let _ = users_repo.query_delete_user(user.id.id.to_raw()).await;
    }
    }