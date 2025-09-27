#[cfg(test)]
mod tests {
    use chrono::Utc;
    use imphnen_hackathon::v1::hackathon::{
        hackathon_dto::{
            HackathonCreateRequestDto, HackathonEventCreateRequestDto,
            HackathonSubmissionCreateRequestDto, HackathonTimelineCreateRequestDto,
            HackathonUpdateRequestDto,
        },
        hackathon_service::{HackathonService, HackathonServiceTrait},
        hackathon_schema::{HackathonEventType, HackathonPhase, HackathonStatus},
    };
    use imphnen_libs::MetaRequestDto;

    #[tokio::test]
    async fn test_create_hackathon_service_success() {
        let app_state = crate::get_app_state().await;

        let request = HackathonCreateRequestDto {
            name: "Service Test Hackathon".to_string(),
            description: "Testing service layer".to_string(),
            start_date: Utc::now() + chrono::Duration::days(2),
            end_date: Utc::now() + chrono::Duration::days(3),
            registration_deadline: Utc::now() + chrono::Duration::days(1),
            max_participants: Some(100),
            theme: Some("AI/ML".to_string()),
            rules: Some("Be nice".to_string()),
            prizes: Some(vec![]),
            organizers: vec!["user-1".to_string()],
        };

        let result = HackathonService::create_hackathon(request, &app_state).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.data.name, "Service Test Hackathon");
        assert_eq!(response.data.status, HackathonStatus::Draft);
    }

    #[tokio::test]
    async fn test_create_hackathon_service_validation_error_end_date_before_start() {
        let app_state = crate::get_app_state().await;

        let request = HackathonCreateRequestDto {
            name: "Invalid Hackathon".to_string(),
            description: "End date before start date".to_string(),
            start_date: Utc::now() + chrono::Duration::days(3),
            end_date: Utc::now() + chrono::Duration::days(2), // Before start
            registration_deadline: Utc::now() + chrono::Duration::days(1),
            max_participants: Some(100),
            theme: None,
            rules: None,
            prizes: None,
            organizers: vec!["user-1".to_string()],
        };

        let result = HackathonService::create_hackathon(request, &app_state).await;
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.status, 400);
        assert!(error.message.contains("End date must be after start date"));
    }

    #[tokio::test]
    async fn test_create_hackathon_service_validation_error_registration_after_start() {
        let app_state = crate::get_app_state().await;

        let request = HackathonCreateRequestDto {
            name: "Invalid Hackathon".to_string(),
            description: "Registration after start".to_string(),
            start_date: Utc::now() + chrono::Duration::days(2),
            end_date: Utc::now() + chrono::Duration::days(3),
            registration_deadline: Utc::now() + chrono::Duration::days(3), // After start
            max_participants: Some(100),
            theme: None,
            rules: None,
            prizes: None,
            organizers: vec!["user-1".to_string()],
        };

        let result = HackathonService::create_hackathon(request, &app_state).await;
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.status, 400);
        assert!(error.message.contains("Registration deadline must be before start date"));
    }

    #[tokio::test]
    async fn test_create_hackathon_service_validation_error_no_organizers() {
        let app_state = crate::get_app_state().await;

        let request = HackathonCreateRequestDto {
            name: "Invalid Hackathon".to_string(),
            description: "No organizers".to_string(),
            start_date: Utc::now() + chrono::Duration::days(2),
            end_date: Utc::now() + chrono::Duration::days(3),
            registration_deadline: Utc::now() + chrono::Duration::days(1),
            max_participants: Some(100),
            theme: None,
            rules: None,
            prizes: None,
            organizers: vec![], // Empty organizers
        };

        let result = HackathonService::create_hackathon(request, &app_state).await;
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.status, 400);
        assert!(error.message.contains("At least one organizer is required"));
    }

    #[tokio::test]
    async fn test_create_hackathon_service_validation_error_name_too_long() {
        let app_state = crate::get_app_state().await;

        let request = HackathonCreateRequestDto {
            name: "a".repeat(101), // 101 characters, exceeds limit
            description: "Valid description".to_string(),
            start_date: Utc::now() + chrono::Duration::days(2),
            end_date: Utc::now() + chrono::Duration::days(3),
            registration_deadline: Utc::now() + chrono::Duration::days(1),
            max_participants: Some(100),
            theme: None,
            rules: None,
            prizes: None,
            organizers: vec!["user-1".to_string()],
        };

        let result = HackathonService::create_hackathon(request, &app_state).await;
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.status, 400);
        assert!(error.message.contains("Validation failed"));
    }

    #[tokio::test]
    async fn test_get_hackathon_service_success() {
        let app_state = crate::get_app_state().await;

        // Create a hackathon first
        let create_request = HackathonCreateRequestDto {
            name: "Get Test Hackathon".to_string(),
            description: "For get testing".to_string(),
            start_date: Utc::now() + chrono::Duration::days(2),
            end_date: Utc::now() + chrono::Duration::days(3),
            registration_deadline: Utc::now() + chrono::Duration::days(1),
            max_participants: Some(50),
            theme: None,
            rules: None,
            prizes: None,
            organizers: vec!["user-1".to_string()],
        };

        let create_result = HackathonService::create_hackathon(create_request, &app_state).await;
        assert!(create_result.is_ok());
        let hackathon_id = create_result.unwrap().data.id;

        // Get the hackathon
        let get_result = HackathonService::get_hackathon(hackathon_id.clone(), &app_state).await;
        assert!(get_result.is_ok());

        let response = get_result.unwrap();
        assert_eq!(response.data.name, "Get Test Hackathon");
        assert_eq!(response.data.id, hackathon_id);
    }

    #[tokio::test]
    async fn test_get_hackathon_service_not_found() {
        let app_state = crate::get_app_state().await;

        let result = HackathonService::get_hackathon("non-existent-id".to_string(), &app_state).await;
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.status, 404);
        assert!(error.message.contains("Hackathon not found"));
    }

    #[tokio::test]
    async fn test_list_hackathons_service() {
        let app_state = crate::get_app_state().await;

        // Create test hackathons
        let request1 = HackathonCreateRequestDto {
            name: "List Test 1".to_string(),
            description: "First hackathon".to_string(),
            start_date: Utc::now() + chrono::Duration::days(2),
            end_date: Utc::now() + chrono::Duration::days(3),
            registration_deadline: Utc::now() + chrono::Duration::days(1),
            max_participants: Some(50),
            theme: None,
            rules: None,
            prizes: None,
            organizers: vec!["user-1".to_string()],
        };

        let request2 = HackathonCreateRequestDto {
            name: "List Test 2".to_string(),
            description: "Second hackathon".to_string(),
            start_date: Utc::now() + chrono::Duration::days(4),
            end_date: Utc::now() + chrono::Duration::days(5),
            registration_deadline: Utc::now() + chrono::Duration::days(3),
            max_participants: Some(75),
            theme: None,
            rules: None,
            prizes: None,
            organizers: vec!["user-2".to_string()],
        };

        let _ = HackathonService::create_hackathon(request1, &app_state).await;
        let _ = HackathonService::create_hackathon(request2, &app_state).await;

        let meta = MetaRequestDto {
            page: Some(1),
            per_page: Some(10),
            search: None,
            sort_by: None,
            order: None,
            filter: None,
            filter_by: None,
        };

        let result = HackathonService::list_hackathons(meta, &app_state).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.data.len() >= 2);
    }

    #[tokio::test]
    async fn test_update_hackathon_service_success() {
        let app_state = crate::get_app_state().await;

        // Create a hackathon first
        let create_request = HackathonCreateRequestDto {
            name: "Update Test Hackathon".to_string(),
            description: "For update testing".to_string(),
            start_date: Utc::now() + chrono::Duration::days(2),
            end_date: Utc::now() + chrono::Duration::days(3),
            registration_deadline: Utc::now() + chrono::Duration::days(1),
            max_participants: Some(50),
            theme: None,
            rules: None,
            prizes: None,
            organizers: vec!["user-1".to_string()],
        };

        let create_result = HackathonService::create_hackathon(create_request, &app_state).await;
        assert!(create_result.is_ok());
        let hackathon_id = create_result.unwrap().data.id;

        // Update the hackathon
        let update_request = HackathonUpdateRequestDto {
            name: Some("Updated Hackathon".to_string()),
            description: Some("Updated description".to_string()),
            start_date: None,
            end_date: None,
            registration_deadline: None,
            max_participants: Some(100),
            theme: Some("Updated Theme".to_string()),
            rules: None,
            prizes: None,
            organizers: None,
        };

        let update_result = HackathonService::update_hackathon(hackathon_id.clone(), update_request, &app_state).await;
        assert!(update_result.is_ok());

        let response = update_result.unwrap();
        assert_eq!(response.data.name, "Updated Hackathon");
        assert_eq!(response.data.max_participants, Some(100));
        assert_eq!(response.data.theme, Some("Updated Theme".to_string()));
    }

    #[tokio::test]
    async fn test_update_hackathon_service_validation_error() {
        let app_state = crate::get_app_state().await;

        // Create a hackathon first
        let create_request = HackathonCreateRequestDto {
            name: "Update Validation Test".to_string(),
            description: "For update validation testing".to_string(),
            start_date: Utc::now() + chrono::Duration::days(2),
            end_date: Utc::now() + chrono::Duration::days(3),
            registration_deadline: Utc::now() + chrono::Duration::days(1),
            max_participants: Some(50),
            theme: None,
            rules: None,
            prizes: None,
            organizers: vec!["user-1".to_string()],
        };

        let create_result = HackathonService::create_hackathon(create_request, &app_state).await;
        assert!(create_result.is_ok());
        let hackathon_id = create_result.unwrap().data.id;

        // Try to update with invalid data
        let update_request = HackathonUpdateRequestDto {
            name: Some("a".repeat(101)), // Too long
            description: None,
            start_date: None,
            end_date: None,
            registration_deadline: None,
            max_participants: None,
            theme: None,
            rules: None,
            prizes: None,
            organizers: None,
        };

        let update_result = HackathonService::update_hackathon(hackathon_id, update_request, &app_state).await;
        assert!(update_result.is_err());

        let error = update_result.unwrap_err();
        assert_eq!(error.status, 400);
        assert!(error.message.contains("Validation failed"));
    }

    #[tokio::test]
    async fn test_delete_hackathon_service_success() {
        let app_state = crate::get_app_state().await;

        // Create a hackathon first
        let create_request = HackathonCreateRequestDto {
            name: "Delete Test Hackathon".to_string(),
            description: "For delete testing".to_string(),
            start_date: Utc::now() + chrono::Duration::days(2),
            end_date: Utc::now() + chrono::Duration::days(3),
            registration_deadline: Utc::now() + chrono::Duration::days(1),
            max_participants: Some(50),
            theme: None,
            rules: None,
            prizes: None,
            organizers: vec!["user-1".to_string()],
        };

        let create_result = HackathonService::create_hackathon(create_request, &app_state).await;
        assert!(create_result.is_ok());
        let hackathon_id = create_result.unwrap().data.id;

        // Delete the hackathon
        let delete_result = HackathonService::delete_hackathon(hackathon_id.clone(), &app_state).await;
        assert!(delete_result.is_ok());

        let response = delete_result.unwrap();
        assert_eq!(response.data, "Hackathon deleted successfully".to_string());

        // Verify it's deleted
        let get_result = HackathonService::get_hackathon(hackathon_id, &app_state).await;
        assert!(get_result.is_err());
    }

    #[tokio::test]
    async fn test_create_hackathon_event_service_success() {
        let app_state = crate::get_app_state().await;

        // Create a hackathon first
        let hackathon_request = HackathonCreateRequestDto {
            name: "Event Service Test".to_string(),
            description: "For event service testing".to_string(),
            start_date: Utc::now() + chrono::Duration::days(2),
            end_date: Utc::now() + chrono::Duration::days(3),
            registration_deadline: Utc::now() + chrono::Duration::days(1),
            max_participants: Some(50),
            theme: None,
            rules: None,
            prizes: None,
            organizers: vec!["user-1".to_string()],
        };

        let hackathon_result = HackathonService::create_hackathon(hackathon_request, &app_state).await;
        assert!(hackathon_result.is_ok());
        let hackathon_id = hackathon_result.unwrap().data.id;

        // Create event
        let event_request = HackathonEventCreateRequestDto {
            title: "Test Event".to_string(),
            description: Some("A test event".to_string()),
            event_type: HackathonEventType::Workshop,
            start_time: Utc::now() + chrono::Duration::days(2),
            end_time: Utc::now() + chrono::Duration::days(2) + chrono::Duration::hours(2),
            location: Some("Room 101".to_string()),
            virtual_link: None,
            max_attendees: Some(30),
            is_mandatory: false,
        };

        let event_result = HackathonService::create_hackathon_event(hackathon_id, event_request, &app_state).await;
        assert!(event_result.is_ok());

        let response = event_result.unwrap();
        assert_eq!(response.data.title, "Test Event");
        assert_eq!(response.data.event_type, HackathonEventType::Workshop);
    }

    #[tokio::test]
    async fn test_create_hackathon_event_service_validation_error_end_before_start() {
        let app_state = crate::get_app_state().await;

        // Create a hackathon first
        let hackathon_request = HackathonCreateRequestDto {
            name: "Event Validation Test".to_string(),
            description: "For event validation testing".to_string(),
            start_date: Utc::now() + chrono::Duration::days(2),
            end_date: Utc::now() + chrono::Duration::days(3),
            registration_deadline: Utc::now() + chrono::Duration::days(1),
            max_participants: Some(50),
            theme: None,
            rules: None,
            prizes: None,
            organizers: vec!["user-1".to_string()],
        };

        let hackathon_result = HackathonService::create_hackathon(hackathon_request, &app_state).await;
        assert!(hackathon_result.is_ok());
        let hackathon_id = hackathon_result.unwrap().data.id;

        // Create event with invalid times
        let event_request = HackathonEventCreateRequestDto {
            title: "Invalid Event".to_string(),
            description: Some("End before start".to_string()),
            event_type: HackathonEventType::Workshop,
            start_time: Utc::now() + chrono::Duration::days(2) + chrono::Duration::hours(2),
            end_time: Utc::now() + chrono::Duration::days(2) + chrono::Duration::hours(1), // Before start
            location: Some("Room 101".to_string()),
            virtual_link: None,
            max_attendees: Some(30),
            is_mandatory: false,
        };

        let event_result = HackathonService::create_hackathon_event(hackathon_id, event_request, &app_state).await;
        assert!(event_result.is_err());

        let error = event_result.unwrap_err();
        assert_eq!(error.status, 400);
        assert!(error.message.contains("End time must be after start time"));
    }

    #[tokio::test]
    async fn test_create_hackathon_event_service_hackathon_not_found() {
        let app_state = crate::get_app_state().await;

        let event_request = HackathonEventCreateRequestDto {
            title: "Event for Non-existent Hackathon".to_string(),
            description: Some("Should fail".to_string()),
            event_type: HackathonEventType::Workshop,
            start_time: Utc::now() + chrono::Duration::days(2),
            end_time: Utc::now() + chrono::Duration::days(2) + chrono::Duration::hours(2),
            location: Some("Room 101".to_string()),
            virtual_link: None,
            max_attendees: Some(30),
            is_mandatory: false,
        };

        let result = HackathonService::create_hackathon_event("non-existent-hackathon".to_string(), event_request, &app_state).await;
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.status, 404);
        assert!(error.message.contains("Hackathon not found"));
    }

    #[tokio::test]
    async fn test_create_hackathon_timeline_service_success() {
        let app_state = crate::get_app_state().await;

        // Create a hackathon first
        let hackathon_request = HackathonCreateRequestDto {
            name: "Timeline Service Test".to_string(),
            description: "For timeline service testing".to_string(),
            start_date: Utc::now() + chrono::Duration::days(2),
            end_date: Utc::now() + chrono::Duration::days(6),
            registration_deadline: Utc::now() + chrono::Duration::days(1),
            max_participants: Some(50),
            theme: None,
            rules: None,
            prizes: None,
            organizers: vec!["user-1".to_string()],
        };

        let hackathon_result = HackathonService::create_hackathon(hackathon_request, &app_state).await;
        assert!(hackathon_result.is_ok());
        let hackathon_id = hackathon_result.unwrap().data.id;

        // Create timeline
        let timeline_request = HackathonTimelineCreateRequestDto {
            phase: HackathonPhase::Registration,
            title: "Registration Phase".to_string(),
            description: Some("Register for the hackathon".to_string()),
            start_date: Utc::now() + chrono::Duration::days(2),
            end_date: Utc::now() + chrono::Duration::days(3),
            is_active: true,
            order: 1,
        };

        let timeline_result = HackathonService::create_hackathon_timeline(hackathon_id, timeline_request, &app_state).await;
        assert!(timeline_result.is_ok());

        let response = timeline_result.unwrap();
        assert_eq!(response.data.title, "Registration Phase");
        assert_eq!(response.data.phase, HackathonPhase::Registration);
        assert_eq!(response.data.is_active, true);
    }

    #[tokio::test]
    async fn test_create_hackathon_timeline_service_validation_error_end_before_start() {
        let app_state = crate::get_app_state().await;

        // Create a hackathon first
        let hackathon_request = HackathonCreateRequestDto {
            name: "Timeline Validation Test".to_string(),
            description: "For timeline validation testing".to_string(),
            start_date: Utc::now() + chrono::Duration::days(2),
            end_date: Utc::now() + chrono::Duration::days(6),
            registration_deadline: Utc::now() + chrono::Duration::days(1),
            max_participants: Some(50),
            theme: None,
            rules: None,
            prizes: None,
            organizers: vec!["user-1".to_string()],
        };

        let hackathon_result = HackathonService::create_hackathon(hackathon_request, &app_state).await;
        assert!(hackathon_result.is_ok());
        let hackathon_id = hackathon_result.unwrap().data.id;

        // Create timeline with invalid dates
        let timeline_request = HackathonTimelineCreateRequestDto {
            phase: HackathonPhase::Ideation,
            title: "Invalid Timeline".to_string(),
            description: Some("End before start".to_string()),
            start_date: Utc::now() + chrono::Duration::days(4),
            end_date: Utc::now() + chrono::Duration::days(3), // Before start
            is_active: false,
            order: 2,
        };

        let timeline_result = HackathonService::create_hackathon_timeline(hackathon_id, timeline_request, &app_state).await;
        assert!(timeline_result.is_err());

        let error = timeline_result.unwrap_err();
        assert_eq!(error.status, 400);
        assert!(error.message.contains("End date must be after start date"));
    }

    #[tokio::test]
    async fn test_create_hackathon_submission_service_success() {
        let app_state = crate::get_app_state().await;

        // Create a hackathon first
        let hackathon_request = HackathonCreateRequestDto {
            name: "Submission Service Test".to_string(),
            description: "For submission service testing".to_string(),
            start_date: Utc::now() + chrono::Duration::days(2),
            end_date: Utc::now() + chrono::Duration::days(3),
            registration_deadline: Utc::now() + chrono::Duration::days(1),
            max_participants: Some(50),
            theme: None,
            rules: None,
            prizes: None,
            organizers: vec!["user-1".to_string()],
        };

        let hackathon_result = HackathonService::create_hackathon(hackathon_request, &app_state).await;
        assert!(hackathon_result.is_ok());
        let hackathon_id = hackathon_result.unwrap().data.id;

        // Create submission
        let submission_request = HackathonSubmissionCreateRequestDto {
            project_name: "Test Project".to_string(),
            description: "A test project submission".to_string(),
            repository_url: Some("https://github.com/test/repo".to_string()),
            demo_url: Some("https://demo.example.com".to_string()),
            slides_url: Some("https://slides.example.com".to_string()),
            technologies: vec!["Rust".to_string(), "React".to_string()],
        };

        let submission_result = HackathonService::create_hackathon_submission(hackathon_id, "team-1".to_string(), submission_request, &app_state).await;
        assert!(submission_result.is_ok());

        let response = submission_result.unwrap();
        assert_eq!(response.data.project_name, "Test Project");
        assert_eq!(response.data.technologies, vec!["Rust".to_string(), "React".to_string()]);
    }

    #[tokio::test]
    async fn test_create_hackathon_submission_service_hackathon_not_found() {
        let app_state = crate::get_app_state().await;

        let submission_request = HackathonSubmissionCreateRequestDto {
            project_name: "Project for Non-existent Hackathon".to_string(),
            description: "Should fail".to_string(),
            repository_url: Some("https://github.com/test/repo".to_string()),
            demo_url: None,
            slides_url: None,
            technologies: vec!["Rust".to_string()],
        };

        let result = HackathonService::create_hackathon_submission("non-existent-hackathon".to_string(), "team-1".to_string(), submission_request, &app_state).await;
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.status, 404);
        assert!(error.message.contains("Hackathon not found"));
    }

    #[tokio::test]
    async fn test_submit_hackathon_submission_service_success() {
        let app_state = crate::get_app_state().await;

        // Create hackathon and submission first
        let hackathon_request = HackathonCreateRequestDto {
            name: "Submit Test Hackathon".to_string(),
            description: "For submit testing".to_string(),
            start_date: Utc::now() + chrono::Duration::days(2),
            end_date: Utc::now() + chrono::Duration::days(3),
            registration_deadline: Utc::now() + chrono::Duration::days(1),
            max_participants: Some(50),
            theme: None,
            rules: None,
            prizes: None,
            organizers: vec!["user-1".to_string()],
        };

        let hackathon_result = HackathonService::create_hackathon(hackathon_request, &app_state).await;
        assert!(hackathon_result.is_ok());
        let hackathon_id = hackathon_result.unwrap().data.id;

        let submission_request = HackathonSubmissionCreateRequestDto {
            project_name: "Project to Submit".to_string(),
            description: "This will be submitted".to_string(),
            repository_url: Some("https://github.com/test/submit".to_string()),
            demo_url: None,
            slides_url: None,
            technologies: vec!["Rust".to_string()],
        };

        let submission_result = HackathonService::create_hackathon_submission(hackathon_id, "team-1".to_string(), submission_request, &app_state).await;
        assert!(submission_result.is_ok());
        let submission_id = submission_result.unwrap().data.id;

        // Submit the submission
        let submit_result = HackathonService::submit_hackathon_submission(submission_id, &app_state).await;
        assert!(submit_result.is_ok());

        let response = submit_result.unwrap();
        assert_eq!(response.data.submission_status, imphnen_hackathon::v1::hackathon::hackathon_schema::SubmissionStatus::Submitted);
    }
}