#[cfg(test)]
mod tests {
    use crate::{generate_unique_email, get_role_id, UsersRepository};
    use axum::http::StatusCode;
    
    use imphnen_hackathon::v1::hackathon::hackathon_dto::{
        HackathonCreateRequestDto, HackathonSubmissionCreateRequestDto,
    };
    use imphnen_hackathon::v1::hackathon::HackathonRepository;
    use imphnen_iam::v1::teams::teams_dto::{TeamsCreateRequestDto};
    use imphnen_iam::v1::teams::teams_repository::TeamsRepository;
    use imphnen_iam::v1::teams::teams_schema::TeamMembersSchema;
    
    use chrono::{Utc, Duration};
    use serde_json::json;
    

    #[tokio::test]
    async fn test_create_hackathon() {
    let app = crate::get_full_test_app().await;
    let users_repo = UsersRepository::new(&app.state);

        // Create test organizer
        let email = generate_unique_email("hackathon_organizer_controller");
        let role_id = get_role_id("mentor", &app.state).await;
        let user_data = crate::create_test_user(&email, "password123", true, &role_id);
        let user_result = users_repo.query_create_user(user_data.clone()).await;
        assert!(user_result.is_ok(), "Failed to create test user");
        let user = users_repo.query_user_by_email(email.clone()).await.unwrap();

        // Create hackathon request
        let hackathon_request = HackathonCreateRequestDto {
            name: "Test Hackathon Controller".to_string(),
            description: "Hackathon created via controller test".to_string(),
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

        // Send create request
        let response = app.service.post("/api/v1/hackathons")
            .header("Authorization", format!("Bearer {}", crate::get_test_token(&user.id.id.to_raw()).await))
            .json(&hackathon_request)
            .await
            .unwrap();

        // For debugging: capture status then extract body so we can print server error details
        let status = response.status();
        let body = crate::get_response_body(response).await;
        eprintln!("[DEBUG] create_hackathon status: {}", status);
        eprintln!("[DEBUG] create_hackathon body: {}", body);

        // Verify response
        assert_eq!(status, StatusCode::CREATED);
    assert_eq!(body["message"], "Success create hackathon");
    assert_eq!(body["data"]["name"], "Test Hackathon Controller");
    assert_eq!(body["data"]["description"], "Hackathon created via controller test");

        // Clean up
        let hackathon_id = body["data"]["id"].as_str().unwrap().to_string();
        let repo = HackathonRepository::new(&app.state);
        let _ = repo.delete_hackathon(hackathon_id).await;
        let _ = users_repo.query_delete_user(user.id.id.to_raw()).await;
    }

    #[tokio::test]
    async fn test_get_hackathon_by_id() {
    let app = crate::get_full_test_app().await;
        let users_repo = UsersRepository::new(&app.state);
        let repo = HackathonRepository::new(&app.state);

        // Create test organizer and hackathon
        let email = generate_unique_email("hackathon_get_controller");
        let role_id = get_role_id("mentor", &app.state).await;
        let user_data = crate::create_test_user(&email, "password123", true, &role_id);
        let user_result = users_repo.query_create_user(user_data.clone()).await;
        assert!(user_result.is_ok(), "Failed to create test user");
        let user = users_repo.query_user_by_email(email.clone()).await.unwrap();

        let hackathon_request = HackathonCreateRequestDto {
            name: "Get Hackathon Test".to_string(),
            description: "Test hackathon for get by ID".to_string(),
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
        let hackathon_id = created.id.id.to_raw();

        // Send get request
        let response = app.service.get(format!("/api/v1/hackathons/{}", hackathon_id))
            .await
            .unwrap();

        // Verify response
        assert_eq!(response.status(), StatusCode::OK);
        let body = crate::get_response_body(response).await;
    assert_eq!(body["data"]["name"], "Get Hackathon Test");
    assert_eq!(body["data"]["description"], "Test hackathon for get by ID");

    // Clean up
    let _ = repo.delete_hackathon(hackathon_id).await;
        let _ = users_repo.query_delete_user(user.id.id.to_raw()).await;
    }

    #[tokio::test]
    async fn test_update_hackathon() {
    let app = crate::get_full_test_app().await;
        let users_repo = UsersRepository::new(&app.state);
    let repo = HackathonRepository::new(&app.state);

        // Create test organizer and hackathon
        let email = generate_unique_email("hackathon_update_controller");
        let role_id = get_role_id("mentor", &app.state).await;
        let user_data = crate::create_test_user(&email, "password123", true, &role_id);
        let user_result = users_repo.query_create_user(user_data.clone()).await;
        assert!(user_result.is_ok(), "Failed to create test user");
        let user = users_repo.query_user_by_email(email.clone()).await.unwrap();

        let hackathon_request = HackathonCreateRequestDto {
            name: "Original Hackathon Controller".to_string(),
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

        let create_result = repo.create_hackathon(hackathon_request.clone()).await.expect("Failed to create hackathon");
        let hackathon_id = create_result.id.id.to_raw();

        // Prepare update payload using the public update DTO
        let update_payload = imphnen_hackathon::v1::hackathon::hackathon_dto::HackathonUpdateRequestDto {
            name: Some("Updated Hackathon Controller".to_string()),
            description: Some("Updated description via controller".to_string()),
            start_date: None,
            end_date: None,
            registration_deadline: None,
            max_participants: None,
            theme: None,
            rules: None,
            prizes: None,
            previous_winners: None,
            organizers: None,
        };

        // Send update request
        let response = app.service.put(format!("/api/v1/hackathons/{}", hackathon_id))
            .header("Authorization", format!("Bearer {}", crate::get_test_token(&user.id.id.to_raw()).await))
            .json(&update_payload)
            .await
            .unwrap();

        // Verify response
        assert_eq!(response.status(), StatusCode::OK);
        let body = crate::get_response_body(response).await;
    assert_eq!(body["message"], "Success update hackathon");
    assert_eq!(body["data"]["name"], "Updated Hackathon Controller");

        // Clean up
    let _ = repo.delete_hackathon(hackathon_id).await;
        let _ = users_repo.query_delete_user(user.id.id.to_raw()).await;
    }

    #[tokio::test]
    async fn test_delete_hackathon() {
    let app = crate::get_full_test_app().await;
        let users_repo = UsersRepository::new(&app.state);
    let repo = HackathonRepository::new(&app.state);

        // Create test organizer and hackathon
        let email = generate_unique_email("hackathon_delete_controller");
        let role_id = get_role_id("mentor", &app.state).await;
        let user_data = crate::create_test_user(&email, "password123", true, &role_id);
        let user_result = users_repo.query_create_user(user_data.clone()).await;
        assert!(user_result.is_ok(), "Failed to create test user");
        let user = users_repo.query_user_by_email(email.clone()).await.unwrap();

        let hackathon_request = HackathonCreateRequestDto {
            name: "Hackathon to Delete Controller".to_string(),
            description: "Hackathon for deletion test".to_string(),
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

        let create_result = repo.create_hackathon(hackathon_request.clone()).await.expect("Failed to create hackathon");
        let hackathon_id = create_result.id.id.to_raw();

        // Send delete request
        let response = app.service.delete(format!("/api/v1/hackathons/{}", hackathon_id))
            .header("Authorization", format!("Bearer {}", crate::get_test_token(&user.id.id.to_raw()).await))
            .await
            .unwrap();

        // Verify response
        assert_eq!(response.status(), StatusCode::OK);
        let body = crate::get_response_body(response).await;
        assert_eq!(body["message"], "Success delete hackathon");

        // Verify hackathon is deleted
        let get_response = app.service.get(format!("/api/v1/hackathons/{}", hackathon_id))
            .await
            .unwrap();
        assert_eq!(get_response.status(), StatusCode::NOT_FOUND);

        // Clean up
        let _ = users_repo.query_delete_user(user.id.id.to_raw()).await;
    }

    #[tokio::test]
    async fn test_submit_project_as_user() {
    let app = crate::get_full_test_app().await;
        let users_repo = UsersRepository::new(&app.state);
    let repo = HackathonRepository::new(&app.state);

        // Create test users and hackathon
        let organizer_email = generate_unique_email("hackathon_submit_organizer_controller");
        let participant_email = generate_unique_email("hackathon_submit_participant_controller");
        
        let role_id = get_role_id("mentee", &app.state).await;
        let organizer_data = crate::create_test_user(&organizer_email, "password123", true, &role_id);
        let participant_data = crate::create_test_user(&participant_email, "password123", true, &role_id);
        
        let organizer_result = users_repo.query_create_user(organizer_data.clone()).await;
        let participant_result = users_repo.query_create_user(participant_data.clone()).await;
        
        assert!(organizer_result.is_ok(), "Failed to create organizer");
        assert!(participant_result.is_ok(), "Failed to create participant");
        
        let organizer = users_repo.query_user_by_email(organizer_email.clone()).await.unwrap();
        let participant = users_repo.query_user_by_email(participant_email.clone()).await.unwrap();

        let hackathon_request = HackathonCreateRequestDto {
            name: "User Submission Test Controller".to_string(),
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

        let create_result = repo.create_hackathon(hackathon_request.clone()).await.expect("Failed to create hackathon");
        let hackathon_id = create_result.id.id.to_raw();

        // Submit project request
        let submission_dto = HackathonSubmissionCreateRequestDto {
            project_name: "My Rust Project Controller".to_string(),
            description: "A cool Rust project for the hackathon".to_string(),
            repository_url: Some("https://github.com/user/my-rust-project".to_string()),
            demo_url: Some("https://my-rust-project.com".to_string()),
            slides_url: None,
            technologies: vec![],
        };

        // Send submission request (use team path; for single-user tests we pass participant id as team_id)
        let response = app.service.post(format!("/api/v1/hackathons/{}/teams/{}/submissions", hackathon_id, participant.id.id.to_raw()))
            .header("Authorization", format!("Bearer {}", crate::get_test_token(&participant.id.id.to_raw()).await))
            .json(&submission_dto)
            .await
            .unwrap();

        // Verify response
        assert_eq!(response.status(), StatusCode::CREATED);
        let body = crate::get_response_body(response).await;
        assert_eq!(body["message"], "Success submit project");
    assert_eq!(body["data"]["project_name"], "My Rust Project Controller");
    assert_eq!(body["data"]["status"], "Draft");

        // Clean up
    let _ = repo.delete_hackathon(hackathon_id).await;
        let _ = users_repo.query_delete_user(organizer.id.id.to_raw()).await;
        let _ = users_repo.query_delete_user(participant.id.id.to_raw()).await;
    }

    #[tokio::test]
    async fn test_submit_project_as_team() {
    let app = crate::get_full_test_app().await;
        let users_repo = UsersRepository::new(&app.state);
        let teams_repo = TeamsRepository::new(&app.state);
    let hackathon_repo = HackathonRepository::new(&app.state);

        // Create test users, team, and hackathon
        let organizer_email = generate_unique_email("hackathon_team_submit_organizer_controller");
        let member1_email = generate_unique_email("team_member1_submit_controller");
        let member2_email = generate_unique_email("team_member2_submit_controller");
        
        let role_id = get_role_id("mentee", &app.state).await;
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
            name: "Hackathon Team Controller".to_string(),
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

        let team_schema = imphnen_iam::TeamsSchema::create(team_request, member1.id.id.to_raw());
        let team_create_result = teams_repo.query_create_team(team_schema).await.unwrap();
        let team_id = team_create_result.split_whitespace().last().unwrap().to_string();

        // Add team members
        let member2_schema = TeamMembersSchema::create(
            team_id.clone(), 
            member2.id.id.to_raw(), 
            Some("member".to_string())
        );
        let add_member_result = teams_repo.query_add_team_member(member2_schema).await;
        assert!(add_member_result.is_ok(), "Failed to add team member");

        // Create hackathon
        let hackathon_request = HackathonCreateRequestDto {
            name: "Team Submission Test Controller".to_string(),
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

        let create_result = hackathon_repo.create_hackathon(hackathon_request.clone()).await.expect("Failed to create hackathon");
        let hackathon_id = create_result.id.id.to_raw();

        // Submit team project request
        let team_submission_dto = HackathonSubmissionCreateRequestDto {
            project_name: "Our Team Rust Project Controller".to_string(),
            description: "A collaborative Rust project by our team".to_string(),
            repository_url: Some("https://github.com/team/our-rust-project".to_string()),
            demo_url: Some("https://our-team-project.com".to_string()),
            slides_url: Some("https://docs.google.com/presentation/d/12345".to_string()),
            technologies: vec![],
        };

        // Send team submission request (use teams path)
        let response = app.service.post(format!("/api/v1/hackathons/{}/teams/{}/submissions", hackathon_id, team_id))
            .header("Authorization", format!("Bearer {}", crate::get_test_token(&member1.id.id.to_raw()).await))
            .json(&team_submission_dto)
            .await
            .unwrap();

        // Verify response
        assert_eq!(response.status(), StatusCode::CREATED);
        let body = crate::get_response_body(response).await;
        assert_eq!(body["message"], "Success submit team project");
        assert_eq!(body["data"]["project_name"], "Our Team Rust Project Controller");
    assert_eq!(body["data"]["status"], "Draft");
        assert_eq!(body["data"]["team_id"], team_id);

        // Clean up
    let _ = hackathon_repo.delete_hackathon(hackathon_id).await;
        let _ = teams_repo.query_delete_team(team_id).await;
        let _ = users_repo.query_delete_user(organizer.id.id.to_raw()).await;
        let _ = users_repo.query_delete_user(member1.id.id.to_raw()).await;
        let _ = users_repo.query_delete_user(member2.id.id.to_raw()).await;
    }

    #[tokio::test]
    async fn test_get_hackathon_submissions() {
    let app = crate::get_full_test_app().await;
        let users_repo = UsersRepository::new(&app.state);
    let repo = HackathonRepository::new(&app.state);

        // Create test users and hackathon
        let organizer_email = generate_unique_email("hackathon_submissions_organizer_controller");
        let participant_email = generate_unique_email("hackathon_submissions_participant_controller");
        
        let role_id = get_role_id("mentee", &app.state).await;
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
            name: "Submissions Test Controller".to_string(),
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

        let create_result = repo.create_hackathon(hackathon_request.clone()).await.expect("Failed to create hackathon");
        let hackathon_id = create_result.id.id.to_raw();

        // Submit multiple projects
        let submission_dtos = [
            HackathonSubmissionCreateRequestDto {
                project_name: "Project 1 Controller".to_string(),
                description: "Description 1".to_string(),
                repository_url: Some("https://github.com/user/project1".to_string()),
                demo_url: None,
                slides_url: None,
                technologies: vec![],
            },
            HackathonSubmissionCreateRequestDto {
                project_name: "Project 2 Controller".to_string(),
                description: "Description 2".to_string(),
                repository_url: Some("https://github.com/user/project2".to_string()),
                demo_url: Some("https://project2.com".to_string()),
                slides_url: None,
                technologies: vec![],
            }
        ];

        for submission_dto in submission_dtos.iter() {
            let response = app.service.post(format!("/api/v1/hackathons/{}/teams/{}/submissions", hackathon_id, participant.id.id.to_raw()))
                .header("Authorization", format!("Bearer {}", crate::get_test_token(&participant.id.id.to_raw()).await))
                .json(submission_dto)
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::CREATED);
        }

        // Get hackathon submissions
        let response = app.service.get(format!("/api/v1/hackathons/{}/submissions", hackathon_id))
            .await
            .unwrap();

        // Verify response
        assert_eq!(response.status(), StatusCode::OK);
        let body = crate::get_response_body(response).await;
        assert_eq!(body["data"].as_array().unwrap().len(), 2);
        assert_eq!(body["data"][0]["project_name"], "Project 1 Controller");
        assert_eq!(body["data"][1]["project_name"], "Project 2 Controller");
    assert_eq!(body["data"][0]["status"], "Draft");
    assert_eq!(body["data"][1]["status"], "Draft");

        // Clean up
    let _ = repo.delete_hackathon(hackathon_id).await;
        let _ = users_repo.query_delete_user(organizer.id.id.to_raw()).await;
        let _ = users_repo.query_delete_user(participant.id.id.to_raw()).await;
    }

    #[tokio::test]
    async fn test_get_user_hackathon_submissions() {
    let app = crate::get_full_test_app().await;
        let users_repo = UsersRepository::new(&app.state);
    let repo = HackathonRepository::new(&app.state);

        // Create test users and hackathons
        let organizer_email = generate_unique_email("hackathon_user_submissions_organizer_controller");
        let participant_email = generate_unique_email("hackathon_user_submissions_participant_controller");
        
        let role_id = get_role_id("mentee", &app.state).await;
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
                name: "Hackathon 1 Controller".to_string(),
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
                name: "Hackathon 2 Controller".to_string(),
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
            let create_result = repo.create_hackathon(hackathon_request.clone()).await.unwrap();
            let hackathon_id = create_result.id.id.to_raw();
            hackathon_ids.push(hackathon_id);
        }

        // Submit projects to different hackathons using team endpoint (single-user uses user id as team_id)
        let submission_requests = [
            HackathonSubmissionCreateRequestDto {
                project_name: "Project for Hackathon 1 Controller".to_string(),
                description: "Description for hackathon 1".to_string(),
                repository_url: Some("https://github.com/user/hackathon1-project".to_string()),
                demo_url: None,
                slides_url: None,
                technologies: vec![],
            },
            HackathonSubmissionCreateRequestDto {
                project_name: "Project for Hackathon 2 Controller".to_string(),
                description: "Description for hackathon 2".to_string(),
                repository_url: Some("https://github.com/user/hackathon2-project".to_string()),
                demo_url: Some("https://hackathon2-project.com".to_string()),
                slides_url: None,
                technologies: vec![],
            }
        ];

        for (i, create_req) in submission_requests.iter().enumerate() {
            let response = app.service.post(format!("/api/v1/hackathons/{}/teams/{}/submissions", hackathon_ids[i], participant.id.id.to_raw()))
                .header("Authorization", format!("Bearer {}", crate::get_test_token(&participant.id.id.to_raw()).await))
                .json(create_req)
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::CREATED);
        }

        // Get user's hackathon submissions
        let response = app.service.get(format!("/api/v1/users/{}/hackathon-submissions", participant.id.id.to_raw()))
            .await
            .unwrap();

        // Verify response
        assert_eq!(response.status(), StatusCode::OK);
        let body = crate::get_response_body(response).await;
        assert_eq!(body["data"].as_array().unwrap().len(), 2);
        assert_eq!(body["data"][0]["project_name"], "Project for Hackathon 1 Controller");
        assert_eq!(body["data"][1]["project_name"], "Project for Hackathon 2 Controller");
        assert_eq!(body["data"][0]["status"], "Draft");
        assert_eq!(body["data"][1]["status"], "Draft");

        // Clean up
        for hackathon_id in hackathon_ids {
            let _ = repo.delete_hackathon(hackathon_id).await;
        }
        let _ = users_repo.query_delete_user(organizer.id.id.to_raw()).await;
        let _ = users_repo.query_delete_user(participant.id.id.to_raw()).await;
    }

    #[tokio::test]
    async fn test_search_hackathons() {
    let app = crate::get_full_test_app().await;
        let users_repo = UsersRepository::new(&app.state);
    let repo = HackathonRepository::new(&app.state);

        // Create test organizer and hackathons
        let email = generate_unique_email("hackathon_search_controller");
        let role_id = get_role_id("mentor", &app.state).await;
        let user_data = crate::create_test_user(&email, "password123", true, &role_id);
        let user_result = users_repo.query_create_user(user_data.clone()).await;
        assert!(user_result.is_ok(), "Failed to create test user");
        let user = users_repo.query_user_by_email(email.clone()).await.unwrap();

        // Create test hackathons
        let hackathon_requests = [
            HackathonCreateRequestDto {
                name: "Rust Backend Hackathon Controller".to_string(),
                description: "Build Rust backend projects".to_string(),
                start_date: Utc::now() + Duration::days(7),
                end_date: Utc::now() + Duration::days(14),
                registration_deadline: Utc::now() + Duration::days(10),
                max_participants: None,
                theme: Some("Backend".to_string()),
                rules: None,
                prizes: None,
                previous_winners: None,
                organizers: vec![user.id.id.to_raw()],
            },
            HackathonCreateRequestDto {
                name: "TypeScript Frontend Hackathon Controller".to_string(),
                description: "Build TypeScript frontend projects".to_string(),
                start_date: Utc::now() + Duration::days(14),
                end_date: Utc::now() + Duration::days(21),
                registration_deadline: Utc::now() + Duration::days(17),
                max_participants: None,
                theme: Some("Frontend".to_string()),
                rules: None,
                prizes: None,
                previous_winners: None,
                organizers: vec![user.id.id.to_raw()],
            },
            HackathonCreateRequestDto {
                name: "Rust Fullstack Hackathon Controller".to_string(),
                description: "Build fullstack projects with Rust".to_string(),
                start_date: Utc::now() + Duration::days(21),
                end_date: Utc::now() + Duration::days(28),
                registration_deadline: Utc::now() + Duration::days(24),
                max_participants: None,
                theme: Some("Fullstack".to_string()),
                rules: None,
                prizes: None,
                previous_winners: None,
                organizers: vec![user.id.id.to_raw()],
            }
        ];

        for hackathon_request in hackathon_requests.iter() {
            let create_result = repo.create_hackathon(hackathon_request.clone()).await.unwrap();
            // Store hackathon IDs for cleanup
            let _ = create_result.id.id.to_raw();
        }

        // Test search with multiple parameters
        let search_params = json!({
            "query": "Rust",
            "category": "Backend",
            "location": "Remote",
            "is_featured": true,
            "page": 1,
            "per_page": 10
        });

        let response = app.service.post("/api/v1/hackathons/search")
            .json(&search_params)
            .await
            .unwrap();

        // Verify response
        assert_eq!(response.status(), StatusCode::OK);
        let body = crate::get_response_body(response).await;
        assert_eq!(body["data"].as_array().unwrap().len(), 1);
    assert_eq!(body["data"][0]["name"], "Rust Backend Hackathon Controller");
    assert!(body["data"][0]["description"].as_str().unwrap().contains("Rust"));

        // Clean up - in a real test you would store and delete all created hackathons
        let _ = users_repo.query_delete_user(user.id.id.to_raw()).await;
    }

    #[tokio::test]
    async fn test_update_submission_status() {
        let app = crate::get_full_test_app().await;
        let users_repo = UsersRepository::new(&app.state);
        let repo = HackathonRepository::new(&app.state);

        // Create test users and hackathon
        let organizer_email = generate_unique_email("hackathon_status_organizer_controller");
        let participant_email = generate_unique_email("hackathon_status_participant_controller");
        
        let role_id = get_role_id("mentee", &app.state).await;
        let organizer_data = crate::create_test_user(&organizer_email, "password123", true, &role_id);
        let participant_data = crate::create_test_user(&participant_email, "password123", true, &role_id);
        
        let organizer_result = users_repo.query_create_user(organizer_data.clone()).await;
        let participant_result = users_repo.query_create_user(participant_data.clone()).await;
        
        assert!(organizer_result.is_ok(), "Failed to create organizer");
        assert!(participant_result.is_ok(), "Failed to create participant");
        
        let organizer = users_repo.query_user_by_email(organizer_email.clone()).await.unwrap();
        let participant = users_repo.query_user_by_email(participant_email.clone()).await.unwrap();

        // Create hackathon and submission
        let hackathon_request = HackathonCreateRequestDto {
            name: "Submission Status Test Controller".to_string(),
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

        let create_result = repo.create_hackathon(hackathon_request).await.unwrap();
        let hackathon_id = create_result.id.id.to_raw();

        let submission_dto = HackathonSubmissionCreateRequestDto {
            project_name: "Test Project Controller".to_string(),
            description: "Test description".to_string(),
            repository_url: Some("https://github.com/user/test-project".to_string()),
            demo_url: None,
            slides_url: None,
            technologies: vec![],
        };

        let submit_response = app.service.post(format!("/api/v1/hackathons/{}/teams/{}/submissions", hackathon_id, participant.id.id.to_raw()))        
            .header("Authorization", format!("Bearer {}", crate::get_test_token(&participant.id.id.to_raw()).await))
            .json(&submission_dto)
            .await
            .unwrap();
        assert_eq!(submit_response.status(), StatusCode::CREATED);

        let submission_id = crate::get_response_body(submit_response).await["data"]["id"]
            .as_str()
            .unwrap()
            .to_string();

        // Update submission status to "Accepted"
        let update_status = json!({
            "status": "Accepted",
            "feedback": "Great project!"
        });

        let response = app.service.patch(format!("/api/v1/hackathons/submissions/{}/status", submission_id))
            .header("Authorization", format!("Bearer {}", crate::get_test_token(&organizer.id.id.to_raw()).await))
            .json(&update_status)
            .await
            .unwrap();

        // Verify response
        assert_eq!(response.status(), StatusCode::OK);
        let body = crate::get_response_body(response).await;
        assert_eq!(body["message"], "Success update submission status");
        assert_eq!(body["data"]["status"], "Accepted");
        assert_eq!(body["data"]["judge_feedback"], "Great project!");

        // Update status again to "Rejected"
        let update_status2 = json!({
            "status": "Rejected",
            "feedback": "Does not meet criteria"
        });

        let response2 = app.service.patch(format!("/api/v1/hackathons/submissions/{}/status", submission_id))
            .header("Authorization", format!("Bearer {}", crate::get_test_token(&organizer.id.id.to_raw()).await))
            .json(&update_status2)
            .await
            .unwrap();

        // Verify second update
        assert_eq!(response2.status(), StatusCode::OK);
        let body2 = crate::get_response_body(response2).await;
        assert_eq!(body2["message"], "Success update submission status");
        assert_eq!(body2["data"]["status"], "Rejected");
        assert_eq!(body2["data"]["judge_feedback"], "Does not meet criteria");

    // Clean up
    let _ = repo.delete_hackathon(hackathon_id).await;
        let _ = users_repo.query_delete_user(organizer.id.id.to_raw()).await;
        let _ = users_repo.query_delete_user(participant.id.id.to_raw()).await;
    }
}