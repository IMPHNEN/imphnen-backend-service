#[cfg(test)]
mod tests {
    use chrono::Utc;
    use imphnen_hackathon::v1::hackathon::{
        hackathon_dto::{
            HackathonCreateRequestDto, HackathonEventCreateRequestDto,
            HackathonEventUpdateRequestDto, HackathonSubmissionCreateRequestDto,
            HackathonSubmissionUpdateRequestDto, HackathonTimelineCreateRequestDto,
            HackathonTimelineUpdateRequestDto, HackathonUpdateRequestDto,
        },
        hackathon_repository::HackathonRepository,
        hackathon_schema::{
            HackathonEventType, HackathonPhase, HackathonStatus,
            SubmissionStatus,
        },
    };

    #[tokio::test]
    async fn test_create_hackathon_repository() {
        let app_state = crate::get_app_state().await;
        let repo = HackathonRepository::new(&app_state);

        let request = HackathonCreateRequestDto {
            name: "Test Hackathon".to_string(),
            description: "A test hackathon".to_string(),
            start_date: Utc::now() + chrono::Duration::days(1),
            end_date: Utc::now() + chrono::Duration::days(2),
            registration_deadline: Utc::now() + chrono::Duration::hours(12),
            max_participants: Some(100),
            theme: Some("AI/ML".to_string()),
            rules: Some("No cheating".to_string()),
            prizes: Some(vec![]),
            organizers: vec!["user-1".to_string()],
        };

        let result = repo.create_hackathon(request).await;
        assert!(result.is_ok());

        let hackathon = result.unwrap();
        assert_eq!(hackathon.name, "Test Hackathon");
        assert_eq!(hackathon.status, HackathonStatus::Draft);

        // Cleanup
        let _ = repo.delete_hackathon(hackathon.id.id.to_raw()).await;
    }

    #[tokio::test]
    async fn test_get_hackathon_by_id_repository() {
        let app_state = crate::get_app_state().await;
        let repo = HackathonRepository::new(&app_state);

        // Create test hackathon
        let request = HackathonCreateRequestDto {
            name: "Test Hackathon Get".to_string(),
            description: "A test hackathon for get".to_string(),
            start_date: Utc::now() + chrono::Duration::days(1),
            end_date: Utc::now() + chrono::Duration::days(2),
            registration_deadline: Utc::now() + chrono::Duration::hours(12),
            max_participants: Some(50),
            theme: None,
            rules: None,
            prizes: None,
            organizers: vec!["user-1".to_string()],
        };

        let created = repo.create_hackathon(request).await.unwrap();
        let hackathon_id = created.id.id.to_raw();

        // Test get by id
        let result = repo.get_hackathon_by_id(hackathon_id.clone()).await;
        assert!(result.is_ok());

        let retrieved = result.unwrap();
        assert_eq!(retrieved.name, "Test Hackathon Get");
        assert_eq!(retrieved.id.id.to_raw(), hackathon_id);

        // Cleanup
        let _ = repo.delete_hackathon(hackathon_id).await;
    }

    #[tokio::test]
    async fn test_get_hackathon_by_id_not_found_repository() {
        let app_state = crate::get_app_state().await;
        let repo = HackathonRepository::new(&app_state);

        let result = repo.get_hackathon_by_id("non-existent-id".to_string()).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Hackathon not found"));
    }

    #[tokio::test]
    async fn test_list_hackathons_repository() {
        let app_state = crate::get_app_state().await;
        let repo = HackathonRepository::new(&app_state);

        // Create test hackathons
        let request1 = HackathonCreateRequestDto {
            name: "Test Hackathon 1".to_string(),
            description: "First test hackathon".to_string(),
            start_date: Utc::now() + chrono::Duration::days(1),
            end_date: Utc::now() + chrono::Duration::days(2),
            registration_deadline: Utc::now() + chrono::Duration::hours(12),
            max_participants: Some(50),
            theme: None,
            rules: None,
            prizes: None,
            organizers: vec!["user-1".to_string()],
        };

        let request2 = HackathonCreateRequestDto {
            name: "Test Hackathon 2".to_string(),
            description: "Second test hackathon".to_string(),
            start_date: Utc::now() + chrono::Duration::days(3),
            end_date: Utc::now() + chrono::Duration::days(4),
            registration_deadline: Utc::now() + chrono::Duration::days(1),
            max_participants: Some(75),
            theme: None,
            rules: None,
            prizes: None,
            organizers: vec!["user-2".to_string()],
        };

        let created1 = repo.create_hackathon(request1).await.unwrap();
        let created2 = repo.create_hackathon(request2).await.unwrap();

        let meta = crate::get_meta_request_dto(1, 10);
        let result = repo.list_hackathons(meta).await;
        assert!(result.is_ok());

        let list_result = result.unwrap();
        assert!(list_result.data.len() >= 2);

        // Verify our test hackathons are in the list
        let names: Vec<String> = list_result.data.iter().map(|h| h.name.clone()).collect();
        assert!(names.contains(&"Test Hackathon 1".to_string()));
        assert!(names.contains(&"Test Hackathon 2".to_string()));

        // Cleanup
        let _ = repo.delete_hackathon(created1.id.id.to_raw()).await;
        let _ = repo.delete_hackathon(created2.id.id.to_raw()).await;
    }

    #[tokio::test]
    async fn test_update_hackathon_repository() {
        let app_state = crate::get_app_state().await;
        let repo = HackathonRepository::new(&app_state);

        // Create test hackathon
        let request = HackathonCreateRequestDto {
            name: "Original Name".to_string(),
            description: "Original description".to_string(),
            start_date: Utc::now() + chrono::Duration::days(1),
            end_date: Utc::now() + chrono::Duration::days(2),
            registration_deadline: Utc::now() + chrono::Duration::hours(12),
            max_participants: Some(50),
            theme: None,
            rules: None,
            prizes: None,
            organizers: vec!["user-1".to_string()],
        };

        let created = repo.create_hackathon(request).await.unwrap();
        let hackathon_id = created.id.id.to_raw();

        // Update hackathon
        let update_request = HackathonUpdateRequestDto {
            name: Some("Updated Name".to_string()),
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

        let result = repo.update_hackathon(hackathon_id.clone(), update_request).await;
        assert!(result.is_ok());

        let updated = result.unwrap();
        assert_eq!(updated.name, "Updated Name");
        assert_eq!(updated.description, "Updated description");
        assert_eq!(updated.max_participants, Some(100));
        assert_eq!(updated.theme, Some("Updated Theme".to_string()));

        // Cleanup
        let _ = repo.delete_hackathon(hackathon_id).await;
    }

    #[tokio::test]
    async fn test_delete_hackathon_repository() {
        let app_state = crate::get_app_state().await;
        let repo = HackathonRepository::new(&app_state);

        // Create test hackathon
        let request = HackathonCreateRequestDto {
            name: "Hackathon to Delete".to_string(),
            description: "This will be deleted".to_string(),
            start_date: Utc::now() + chrono::Duration::days(1),
            end_date: Utc::now() + chrono::Duration::days(2),
            registration_deadline: Utc::now() + chrono::Duration::hours(12),
            max_participants: Some(50),
            theme: None,
            rules: None,
            prizes: None,
            organizers: vec!["user-1".to_string()],
        };

        let created = repo.create_hackathon(request).await.unwrap();
        let hackathon_id = created.id.id.to_raw();

        // Delete hackathon
        let result = repo.delete_hackathon(hackathon_id.clone()).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hackathon deleted successfully".to_string());

        // Verify it's deleted (soft delete)
        let get_result = repo.get_hackathon_by_id(hackathon_id).await;
        assert!(get_result.is_err());
    }

    #[tokio::test]
    async fn test_create_hackathon_event_repository() {
        let app_state = crate::get_app_state().await;
        let repo = HackathonRepository::new(&app_state);

        // Create test hackathon first
        let hackathon_request = HackathonCreateRequestDto {
            name: "Event Test Hackathon".to_string(),
            description: "Hackathon for event testing".to_string(),
            start_date: Utc::now() + chrono::Duration::days(1),
            end_date: Utc::now() + chrono::Duration::days(2),
            registration_deadline: Utc::now() + chrono::Duration::hours(12),
            max_participants: Some(50),
            theme: None,
            rules: None,
            prizes: None,
            organizers: vec!["user-1".to_string()],
        };

        let hackathon = repo.create_hackathon(hackathon_request).await.unwrap();
        let hackathon_id = hackathon.id.id.to_raw();

        // Create event
        let event_request = HackathonEventCreateRequestDto {
            title: "Test Event".to_string(),
            description: Some("A test event".to_string()),
            event_type: HackathonEventType::Workshop,
            start_time: Utc::now() + chrono::Duration::days(1),
            end_time: Utc::now() + chrono::Duration::days(1) + chrono::Duration::hours(2),
            location: Some("Room 101".to_string()),
            virtual_link: None,
            max_attendees: Some(30),
            is_mandatory: false,
        };

        let result = repo.create_hackathon_event(hackathon_id.clone(), event_request).await;
        assert!(result.is_ok());

        let event = result.unwrap();
        assert_eq!(event.title, "Test Event");
        assert_eq!(event.event_type, HackathonEventType::Workshop);

        // Cleanup
        let _ = repo.delete_hackathon_event(event.id.id.to_raw()).await;
        let _ = repo.delete_hackathon(hackathon_id).await;
    }

    #[tokio::test]
    async fn test_list_hackathon_events_repository() {
        let app_state = crate::get_app_state().await;
        let repo = HackathonRepository::new(&app_state);

        // Create test hackathon
        let hackathon_request = HackathonCreateRequestDto {
            name: "Events List Test".to_string(),
            description: "Hackathon for events listing".to_string(),
            start_date: Utc::now() + chrono::Duration::days(1),
            end_date: Utc::now() + chrono::Duration::days(2),
            registration_deadline: Utc::now() + chrono::Duration::hours(12),
            max_participants: Some(50),
            theme: None,
            rules: None,
            prizes: None,
            organizers: vec!["user-1".to_string()],
        };

        let hackathon = repo.create_hackathon(hackathon_request).await.unwrap();
        let hackathon_id = hackathon.id.id.to_raw();

        // Create events
        let event1_request = HackathonEventCreateRequestDto {
            title: "Event 1".to_string(),
            description: Some("First event".to_string()),
            event_type: HackathonEventType::Workshop,
            start_time: Utc::now() + chrono::Duration::days(1),
            end_time: Utc::now() + chrono::Duration::days(1) + chrono::Duration::hours(1),
            location: Some("Room 101".to_string()),
            virtual_link: None,
            max_attendees: Some(20),
            is_mandatory: false,
        };

        let event2_request = HackathonEventCreateRequestDto {
            title: "Event 2".to_string(),
            description: Some("Second event".to_string()),
            event_type: HackathonEventType::Keynote,
            start_time: Utc::now() + chrono::Duration::days(1) + chrono::Duration::hours(2),
            end_time: Utc::now() + chrono::Duration::days(1) + chrono::Duration::hours(3),
            location: Some("Auditorium".to_string()),
            virtual_link: None,
            max_attendees: Some(100),
            is_mandatory: true,
        };

        let event1 = repo.create_hackathon_event(hackathon_id.clone(), event1_request).await.unwrap();
        let event2 = repo.create_hackathon_event(hackathon_id.clone(), event2_request).await.unwrap();

        let meta = crate::get_meta_request_dto(1, 10);
        let result = repo.list_hackathon_events(meta, hackathon_id.clone()).await;
        assert!(result.is_ok());

        let list_result = result.unwrap();
        assert!(list_result.data.len() >= 2);

        // Cleanup
        let _ = repo.delete_hackathon_event(event1.id.id.to_raw()).await;
        let _ = repo.delete_hackathon_event(event2.id.id.to_raw()).await;
        let _ = repo.delete_hackathon(hackathon_id).await;
    }

    #[tokio::test]
    async fn test_update_hackathon_event_repository() {
        let app_state = crate::get_app_state().await;
        let repo = HackathonRepository::new(&app_state);

        // Create test hackathon and event
        let hackathon_request = HackathonCreateRequestDto {
            name: "Event Update Test".to_string(),
            description: "Hackathon for event update testing".to_string(),
            start_date: Utc::now() + chrono::Duration::days(1),
            end_date: Utc::now() + chrono::Duration::days(2),
            registration_deadline: Utc::now() + chrono::Duration::hours(12),
            max_participants: Some(50),
            theme: None,
            rules: None,
            prizes: None,
            organizers: vec!["user-1".to_string()],
        };

        let hackathon = repo.create_hackathon(hackathon_request).await.unwrap();
        let hackathon_id = hackathon.id.id.to_raw();

        let event_request = HackathonEventCreateRequestDto {
            title: "Original Event".to_string(),
            description: Some("Original description".to_string()),
            event_type: HackathonEventType::Workshop,
            start_time: Utc::now() + chrono::Duration::days(1),
            end_time: Utc::now() + chrono::Duration::days(1) + chrono::Duration::hours(1),
            location: Some("Room 101".to_string()),
            virtual_link: None,
            max_attendees: Some(20),
            is_mandatory: false,
        };

        let event = repo.create_hackathon_event(hackathon_id.clone(), event_request).await.unwrap();
        let event_id = event.id.id.to_raw();

        // Update event
        let update_request = HackathonEventUpdateRequestDto {
            title: Some("Updated Event".to_string()),
            description: Some("Updated description".to_string()),
            event_type: Some(HackathonEventType::Keynote),
            start_time: None,
            end_time: None,
            location: Some("Auditorium".to_string()),
            virtual_link: None,
            max_attendees: Some(50),
            is_mandatory: Some(true),
        };

        let result = repo.update_hackathon_event(event_id.clone(), update_request).await;
        assert!(result.is_ok());

        let updated = result.unwrap();
        assert_eq!(updated.title, "Updated Event");
        assert_eq!(updated.event_type, HackathonEventType::Keynote);
        assert_eq!(updated.max_attendees, Some(50));
        assert_eq!(updated.is_mandatory, true);

        // Cleanup
        let _ = repo.delete_hackathon_event(event_id).await;
        let _ = repo.delete_hackathon(hackathon_id).await;
    }

    #[tokio::test]
    async fn test_delete_hackathon_event_repository() {
        let app_state = crate::get_app_state().await;
        let repo = HackathonRepository::new(&app_state);

        // Create test hackathon and event
        let hackathon_request = HackathonCreateRequestDto {
            name: "Event Delete Test".to_string(),
            description: "Hackathon for event delete testing".to_string(),
            start_date: Utc::now() + chrono::Duration::days(1),
            end_date: Utc::now() + chrono::Duration::days(2),
            registration_deadline: Utc::now() + chrono::Duration::hours(12),
            max_participants: Some(50),
            theme: None,
            rules: None,
            prizes: None,
            organizers: vec!["user-1".to_string()],
        };

        let hackathon = repo.create_hackathon(hackathon_request).await.unwrap();
        let hackathon_id = hackathon.id.id.to_raw();

        let event_request = HackathonEventCreateRequestDto {
            title: "Event to Delete".to_string(),
            description: Some("This event will be deleted".to_string()),
            event_type: HackathonEventType::Workshop,
            start_time: Utc::now() + chrono::Duration::days(1),
            end_time: Utc::now() + chrono::Duration::days(1) + chrono::Duration::hours(1),
            location: Some("Room 101".to_string()),
            virtual_link: None,
            max_attendees: Some(20),
            is_mandatory: false,
        };

        let event = repo.create_hackathon_event(hackathon_id.clone(), event_request).await.unwrap();
        let event_id = event.id.id.to_raw();

        // Delete event
        let result = repo.delete_hackathon_event(event_id.clone()).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Event deleted successfully".to_string());

        // Cleanup hackathon
        let _ = repo.delete_hackathon(hackathon_id).await;
    }

    #[tokio::test]
    async fn test_create_hackathon_timeline_repository() {
        let app_state = crate::get_app_state().await;
        let repo = HackathonRepository::new(&app_state);

        // Create test hackathon first
        let hackathon_request = HackathonCreateRequestDto {
            name: "Timeline Test Hackathon".to_string(),
            description: "Hackathon for timeline testing".to_string(),
            start_date: Utc::now() + chrono::Duration::days(1),
            end_date: Utc::now() + chrono::Duration::days(5),
            registration_deadline: Utc::now() + chrono::Duration::hours(12),
            max_participants: Some(50),
            theme: None,
            rules: None,
            prizes: None,
            organizers: vec!["user-1".to_string()],
        };

        let hackathon = repo.create_hackathon(hackathon_request).await.unwrap();
        let hackathon_id = hackathon.id.id.to_raw();

        // Create timeline
        let timeline_request = HackathonTimelineCreateRequestDto {
            phase: HackathonPhase::Registration,
            title: "Registration Phase".to_string(),
            description: Some("Register for the hackathon".to_string()),
            start_date: Utc::now() + chrono::Duration::days(1),
            end_date: Utc::now() + chrono::Duration::days(2),
            is_active: true,
            order: 1,
        };

        let result = repo.create_hackathon_timeline(hackathon_id.clone(), timeline_request).await;
        assert!(result.is_ok());

        let timeline = result.unwrap();
        assert_eq!(timeline.title, "Registration Phase");
        assert_eq!(timeline.phase, HackathonPhase::Registration);
        assert_eq!(timeline.is_active, true);

        // Cleanup
        let _ = repo.delete_hackathon_timeline(timeline.id.id.to_raw()).await;
        let _ = repo.delete_hackathon(hackathon_id).await;
    }

    #[tokio::test]
    async fn test_list_hackathon_timeline_repository() {
        let app_state = crate::get_app_state().await;
        let repo = HackathonRepository::new(&app_state);

        // Create test hackathon
        let hackathon_request = HackathonCreateRequestDto {
            name: "Timeline List Test".to_string(),
            description: "Hackathon for timeline listing".to_string(),
            start_date: Utc::now() + chrono::Duration::days(1),
            end_date: Utc::now() + chrono::Duration::days(5),
            registration_deadline: Utc::now() + chrono::Duration::hours(12),
            max_participants: Some(50),
            theme: None,
            rules: None,
            prizes: None,
            organizers: vec!["user-1".to_string()],
        };

        let hackathon = repo.create_hackathon(hackathon_request).await.unwrap();
        let hackathon_id = hackathon.id.id.to_raw();

        // Create timeline entries
        let timeline1_request = HackathonTimelineCreateRequestDto {
            phase: HackathonPhase::Registration,
            title: "Registration".to_string(),
            description: Some("Register now".to_string()),
            start_date: Utc::now() + chrono::Duration::days(1),
            end_date: Utc::now() + chrono::Duration::days(2),
            is_active: true,
            order: 1,
        };

        let timeline2_request = HackathonTimelineCreateRequestDto {
            phase: HackathonPhase::Ideation,
            title: "Ideation".to_string(),
            description: Some("Brainstorm ideas".to_string()),
            start_date: Utc::now() + chrono::Duration::days(2),
            end_date: Utc::now() + chrono::Duration::days(3),
            is_active: false,
            order: 2,
        };

        let timeline1 = repo.create_hackathon_timeline(hackathon_id.clone(), timeline1_request).await.unwrap();
        let timeline2 = repo.create_hackathon_timeline(hackathon_id.clone(), timeline2_request).await.unwrap();

        let meta = crate::get_meta_request_dto(1, 10);
        let result = repo.list_hackathon_timeline(meta, hackathon_id.clone()).await;
        assert!(result.is_ok());

        let list_result = result.unwrap();
        assert!(list_result.data.len() >= 2);

        // Cleanup
        let _ = repo.delete_hackathon_timeline(timeline1.id.id.to_raw()).await;
        let _ = repo.delete_hackathon_timeline(timeline2.id.id.to_raw()).await;
        let _ = repo.delete_hackathon(hackathon_id).await;
    }

    #[tokio::test]
    async fn test_update_hackathon_timeline_repository() {
        let app_state = crate::get_app_state().await;
        let repo = HackathonRepository::new(&app_state);

        // Create test hackathon and timeline
        let hackathon_request = HackathonCreateRequestDto {
            name: "Timeline Update Test".to_string(),
            description: "Hackathon for timeline update testing".to_string(),
            start_date: Utc::now() + chrono::Duration::days(1),
            end_date: Utc::now() + chrono::Duration::days(5),
            registration_deadline: Utc::now() + chrono::Duration::hours(12),
            max_participants: Some(50),
            theme: None,
            rules: None,
            prizes: None,
            organizers: vec!["user-1".to_string()],
        };

        let hackathon = repo.create_hackathon(hackathon_request).await.unwrap();
        let hackathon_id = hackathon.id.id.to_raw();

        let timeline_request = HackathonTimelineCreateRequestDto {
            phase: HackathonPhase::Registration,
            title: "Original Timeline".to_string(),
            description: Some("Original description".to_string()),
            start_date: Utc::now() + chrono::Duration::days(1),
            end_date: Utc::now() + chrono::Duration::days(2),
            is_active: true,
            order: 1,
        };

        let timeline = repo.create_hackathon_timeline(hackathon_id.clone(), timeline_request).await.unwrap();
        let timeline_id = timeline.id.id.to_raw();

        // Update timeline
        let update_request = HackathonTimelineUpdateRequestDto {
            phase: Some(HackathonPhase::Ideation),
            title: Some("Updated Timeline".to_string()),
            description: Some("Updated description".to_string()),
            start_date: None,
            end_date: None,
            is_active: Some(false),
            order: Some(2),
        };

        let result = repo.update_hackathon_timeline(timeline_id.clone(), update_request).await;
        assert!(result.is_ok());

        let updated = result.unwrap();
        assert_eq!(updated.title, "Updated Timeline");
        assert_eq!(updated.phase, HackathonPhase::Ideation);
        assert_eq!(updated.is_active, false);
        assert_eq!(updated.order, 2);

        // Cleanup
        let _ = repo.delete_hackathon_timeline(timeline_id).await;
        let _ = repo.delete_hackathon(hackathon_id).await;
    }

    #[tokio::test]
    async fn test_delete_hackathon_timeline_repository() {
        let app_state = crate::get_app_state().await;
        let repo = HackathonRepository::new(&app_state);

        // Create test hackathon and timeline
        let hackathon_request = HackathonCreateRequestDto {
            name: "Timeline Delete Test".to_string(),
            description: "Hackathon for timeline delete testing".to_string(),
            start_date: Utc::now() + chrono::Duration::days(1),
            end_date: Utc::now() + chrono::Duration::days(5),
            registration_deadline: Utc::now() + chrono::Duration::hours(12),
            max_participants: Some(50),
            theme: None,
            rules: None,
            prizes: None,
            organizers: vec!["user-1".to_string()],
        };

        let hackathon = repo.create_hackathon(hackathon_request).await.unwrap();
        let hackathon_id = hackathon.id.id.to_raw();

        let timeline_request = HackathonTimelineCreateRequestDto {
            phase: HackathonPhase::Registration,
            title: "Timeline to Delete".to_string(),
            description: Some("This timeline will be deleted".to_string()),
            start_date: Utc::now() + chrono::Duration::days(1),
            end_date: Utc::now() + chrono::Duration::days(2),
            is_active: true,
            order: 1,
        };

        let timeline = repo.create_hackathon_timeline(hackathon_id.clone(), timeline_request).await.unwrap();
        let timeline_id = timeline.id.id.to_raw();

        // Delete timeline
        let result = repo.delete_hackathon_timeline(timeline_id.clone()).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Timeline deleted successfully".to_string());

        // Cleanup hackathon
        let _ = repo.delete_hackathon(hackathon_id).await;
    }

    #[tokio::test]
    async fn test_create_hackathon_submission_repository() {
        let app_state = crate::get_app_state().await;
        let repo = HackathonRepository::new(&app_state);

        // Create test hackathon first
        let hackathon_request = HackathonCreateRequestDto {
            name: "Submission Test Hackathon".to_string(),
            description: "Hackathon for submission testing".to_string(),
            start_date: Utc::now() + chrono::Duration::days(1),
            end_date: Utc::now() + chrono::Duration::days(2),
            registration_deadline: Utc::now() + chrono::Duration::hours(12),
            max_participants: Some(50),
            theme: None,
            rules: None,
            prizes: None,
            organizers: vec!["user-1".to_string()],
        };

        let hackathon = repo.create_hackathon(hackathon_request).await.unwrap();
        let hackathon_id = hackathon.id.id.to_raw();

        // Create submission
        let submission_request = HackathonSubmissionCreateRequestDto {
            project_name: "Test Project".to_string(),
            description: "A test project submission".to_string(),
            repository_url: Some("https://github.com/test/repo".to_string()),
            demo_url: Some("https://demo.example.com".to_string()),
            slides_url: Some("https://slides.example.com".to_string()),
            technologies: vec!["Rust".to_string(), "React".to_string()],
        };

        let result = repo.create_hackathon_submission(hackathon_id.clone(), "team-1".to_string(), submission_request).await;
        assert!(result.is_ok());

        let submission = result.unwrap();
        assert_eq!(submission.project_name, "Test Project");
        assert_eq!(submission.submission_status, SubmissionStatus::Draft);
        assert_eq!(submission.technologies, vec!["Rust".to_string(), "React".to_string()]);

        // Cleanup
        let _ = repo.delete_hackathon_submission(submission.id.id.to_raw()).await;
        let _ = repo.delete_hackathon(hackathon_id).await;
    }

    #[tokio::test]
    async fn test_list_hackathon_submissions_repository() {
        let app_state = crate::get_app_state().await;
        let repo = HackathonRepository::new(&app_state);

        // Create test hackathon
        let hackathon_request = HackathonCreateRequestDto {
            name: "Submissions List Test".to_string(),
            description: "Hackathon for submissions listing".to_string(),
            start_date: Utc::now() + chrono::Duration::days(1),
            end_date: Utc::now() + chrono::Duration::days(2),
            registration_deadline: Utc::now() + chrono::Duration::hours(12),
            max_participants: Some(50),
            theme: None,
            rules: None,
            prizes: None,
            organizers: vec!["user-1".to_string()],
        };

        let hackathon = repo.create_hackathon(hackathon_request).await.unwrap();
        let hackathon_id = hackathon.id.id.to_raw();

        // Create submissions
        let submission1_request = HackathonSubmissionCreateRequestDto {
            project_name: "Project 1".to_string(),
            description: "First project".to_string(),
            repository_url: Some("https://github.com/test/repo1".to_string()),
            demo_url: None,
            slides_url: None,
            technologies: vec!["Rust".to_string()],
        };

        let submission2_request = HackathonSubmissionCreateRequestDto {
            project_name: "Project 2".to_string(),
            description: "Second project".to_string(),
            repository_url: Some("https://github.com/test/repo2".to_string()),
            demo_url: Some("https://demo2.example.com".to_string()),
            slides_url: None,
            technologies: vec!["Python".to_string(), "Django".to_string()],
        };

        let submission1 = repo.create_hackathon_submission(hackathon_id.clone(), "team-1".to_string(), submission1_request).await.unwrap();
        let submission2 = repo.create_hackathon_submission(hackathon_id.clone(), "team-2".to_string(), submission2_request).await.unwrap();

        let meta = crate::get_meta_request_dto(1, 10);
        let result = repo.list_hackathon_submissions(meta, hackathon_id.clone()).await;
        assert!(result.is_ok());

        let list_result = result.unwrap();
        assert!(list_result.data.len() >= 2);

        // Cleanup
        let _ = repo.delete_hackathon_submission(submission1.id.id.to_raw()).await;
        let _ = repo.delete_hackathon_submission(submission2.id.id.to_raw()).await;
        let _ = repo.delete_hackathon(hackathon_id).await;
    }

    #[tokio::test]
    async fn test_update_hackathon_submission_repository() {
        let app_state = crate::get_app_state().await;
        let repo = HackathonRepository::new(&app_state);

        // Create test hackathon and submission
        let hackathon_request = HackathonCreateRequestDto {
            name: "Submission Update Test".to_string(),
            description: "Hackathon for submission update testing".to_string(),
            start_date: Utc::now() + chrono::Duration::days(1),
            end_date: Utc::now() + chrono::Duration::days(2),
            registration_deadline: Utc::now() + chrono::Duration::hours(12),
            max_participants: Some(50),
            theme: None,
            rules: None,
            prizes: None,
            organizers: vec!["user-1".to_string()],
        };

        let hackathon = repo.create_hackathon(hackathon_request).await.unwrap();
        let hackathon_id = hackathon.id.id.to_raw();

        let submission_request = HackathonSubmissionCreateRequestDto {
            project_name: "Original Project".to_string(),
            description: "Original description".to_string(),
            repository_url: Some("https://github.com/test/original".to_string()),
            demo_url: None,
            slides_url: None,
            technologies: vec!["Rust".to_string()],
        };

        let submission = repo.create_hackathon_submission(hackathon_id.clone(), "team-1".to_string(), submission_request).await.unwrap();
        let submission_id = submission.id.id.to_raw();

        // Update submission
        let update_request = HackathonSubmissionUpdateRequestDto {
            project_name: Some("Updated Project".to_string()),
            description: Some("Updated description".to_string()),
            repository_url: Some("https://github.com/test/updated".to_string()),
            demo_url: Some("https://demo-updated.example.com".to_string()),
            slides_url: Some("https://slides-updated.example.com".to_string()),
            technologies: Some(vec!["Rust".to_string(), "TypeScript".to_string()]),
        };

        let result = repo.update_hackathon_submission(submission_id.clone(), update_request).await;
        assert!(result.is_ok());

        let updated = result.unwrap();
        assert_eq!(updated.project_name, "Updated Project");
        assert_eq!(updated.description, "Updated description");
        assert_eq!(updated.repository_url, Some("https://github.com/test/updated".to_string()));
        assert_eq!(updated.demo_url, Some("https://demo-updated.example.com".to_string()));
        assert_eq!(updated.slides_url, Some("https://slides-updated.example.com".to_string()));
        assert_eq!(updated.technologies, vec!["Rust".to_string(), "TypeScript".to_string()]);

        // Cleanup
        let _ = repo.delete_hackathon_submission(submission_id).await;
        let _ = repo.delete_hackathon(hackathon_id).await;
    }

    #[tokio::test]
    async fn test_submit_hackathon_submission_repository() {
        let app_state = crate::get_app_state().await;
        let repo = HackathonRepository::new(&app_state);

        // Create test hackathon and submission
        let hackathon_request = HackathonCreateRequestDto {
            name: "Submission Submit Test".to_string(),
            description: "Hackathon for submission submit testing".to_string(),
            start_date: Utc::now() + chrono::Duration::days(1),
            end_date: Utc::now() + chrono::Duration::days(2),
            registration_deadline: Utc::now() + chrono::Duration::hours(12),
            max_participants: Some(50),
            theme: None,
            rules: None,
            prizes: None,
            organizers: vec!["user-1".to_string()],
        };

        let hackathon = repo.create_hackathon(hackathon_request).await.unwrap();
        let hackathon_id = hackathon.id.id.to_raw();

        let submission_request = HackathonSubmissionCreateRequestDto {
            project_name: "Project to Submit".to_string(),
            description: "This project will be submitted".to_string(),
            repository_url: Some("https://github.com/test/submit".to_string()),
            demo_url: None,
            slides_url: None,
            technologies: vec!["Rust".to_string()],
        };

        let submission = repo.create_hackathon_submission(hackathon_id.clone(), "team-1".to_string(), submission_request).await.unwrap();
        let submission_id = submission.id.id.to_raw();

        // Submit submission
        let result = repo.submit_hackathon_submission(submission_id.clone()).await;
        assert!(result.is_ok());

        let submitted = result.unwrap();
        assert_eq!(submitted.submission_status, SubmissionStatus::Submitted);

        // Cleanup
        let _ = repo.delete_hackathon_submission(submission_id).await;
        let _ = repo.delete_hackathon(hackathon_id).await;
    }

    #[tokio::test]
    async fn test_delete_hackathon_submission_repository() {
        let app_state = crate::get_app_state().await;
        let repo = HackathonRepository::new(&app_state);

        // Create test hackathon and submission
        let hackathon_request = HackathonCreateRequestDto {
            name: "Submission Delete Test".to_string(),
            description: "Hackathon for submission delete testing".to_string(),
            start_date: Utc::now() + chrono::Duration::days(1),
            end_date: Utc::now() + chrono::Duration::days(2),
            registration_deadline: Utc::now() + chrono::Duration::hours(12),
            max_participants: Some(50),
            theme: None,
            rules: None,
            prizes: None,
            organizers: vec!["user-1".to_string()],
        };

        let hackathon = repo.create_hackathon(hackathon_request).await.unwrap();
        let hackathon_id = hackathon.id.id.to_raw();

        let submission_request = HackathonSubmissionCreateRequestDto {
            project_name: "Submission to Delete".to_string(),
            description: "This submission will be deleted".to_string(),
            repository_url: Some("https://github.com/test/delete".to_string()),
            demo_url: None,
            slides_url: None,
            technologies: vec!["Rust".to_string()],
        };

        let submission = repo.create_hackathon_submission(hackathon_id.clone(), "team-1".to_string(), submission_request).await.unwrap();
        let submission_id = submission.id.id.to_raw();

        // Delete submission
        let result = repo.delete_hackathon_submission(submission_id.clone()).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Submission deleted successfully".to_string());

        // Cleanup hackathon
        let _ = repo.delete_hackathon(hackathon_id).await;
    }

    #[tokio::test]
    async fn test_update_hackathon_not_found_repository() {
        let app_state = crate::get_app_state().await;
        let repo = HackathonRepository::new(&app_state);

        let update_request = HackathonUpdateRequestDto {
            name: Some("Updated Name".to_string()),
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

        let result = repo.update_hackathon("non-existent-id".to_string(), update_request).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Hackathon not found"));
    }

    #[tokio::test]
    async fn test_delete_hackathon_not_found_repository() {
        let app_state = crate::get_app_state().await;
        let repo = HackathonRepository::new(&app_state);

        let result = repo.delete_hackathon("non-existent-id".to_string()).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Failed to delete hackathon"));
    }

    #[tokio::test]
    async fn test_update_hackathon_event_not_found_repository() {
        let app_state = crate::get_app_state().await;
        let repo = HackathonRepository::new(&app_state);

        let update_request = HackathonEventUpdateRequestDto {
            title: Some("Updated Event".to_string()),
            description: None,
            event_type: None,
            start_time: None,
            end_time: None,
            location: None,
            virtual_link: None,
            max_attendees: None,
            is_mandatory: None,
        };

        let result = repo.update_hackathon_event("non-existent-id".to_string(), update_request).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Event not found"));
    }

    #[tokio::test]
    async fn test_delete_hackathon_event_not_found_repository() {
        let app_state = crate::get_app_state().await;
        let repo = HackathonRepository::new(&app_state);

        let result = repo.delete_hackathon_event("non-existent-id".to_string()).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Failed to delete event"));
    }

    #[tokio::test]
    async fn test_update_hackathon_timeline_not_found_repository() {
        let app_state = crate::get_app_state().await;
        let repo = HackathonRepository::new(&app_state);

        let update_request = HackathonTimelineUpdateRequestDto {
            phase: Some(HackathonPhase::Ideation),
            title: Some("Updated Timeline".to_string()),
            description: None,
            start_date: None,
            end_date: None,
            is_active: None,
            order: None,
        };

        let result = repo.update_hackathon_timeline("non-existent-id".to_string(), update_request).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Timeline not found"));
    }

    #[tokio::test]
    async fn test_delete_hackathon_timeline_not_found_repository() {
        let app_state = crate::get_app_state().await;
        let repo = HackathonRepository::new(&app_state);

        let result = repo.delete_hackathon_timeline("non-existent-id".to_string()).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Failed to delete timeline"));
    }

    #[tokio::test]
    async fn test_update_hackathon_submission_not_found_repository() {
        let app_state = crate::get_app_state().await;
        let repo = HackathonRepository::new(&app_state);

        let update_request = HackathonSubmissionUpdateRequestDto {
            project_name: Some("Updated Project".to_string()),
            description: None,
            repository_url: None,
            demo_url: None,
            slides_url: None,
            technologies: None,
        };

        let result = repo.update_hackathon_submission("non-existent-id".to_string(), update_request).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Submission not found"));
    }

    #[tokio::test]
    async fn test_submit_hackathon_submission_not_found_repository() {
        let app_state = crate::get_app_state().await;
        let repo = HackathonRepository::new(&app_state);

        let result = repo.submit_hackathon_submission("non-existent-id".to_string()).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Submission not found"));
    }

    #[tokio::test]
    async fn test_delete_hackathon_submission_not_found_repository() {
        let app_state = crate::get_app_state().await;
        let repo = HackathonRepository::new(&app_state);

        let result = repo.delete_hackathon_submission("non-existent-id".to_string()).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Failed to delete submission"));
    }

    #[tokio::test]
    async fn test_get_hackathon_submission_by_id_not_found_repository() {
        let app_state = crate::get_app_state().await;
        let repo = HackathonRepository::new(&app_state);

        let result = repo.get_hackathon_submission_by_id("non-existent-id".to_string()).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Submission not found"));
    }

    #[tokio::test]
    async fn test_get_submission_timeline_phase_repository() {
        let app_state = crate::get_app_state().await;
        let repo = HackathonRepository::new(&app_state);

        // Create test hackathon
        let hackathon_request = HackathonCreateRequestDto {
            name: "Timeline Phase Test".to_string(),
            description: "Hackathon for timeline phase testing".to_string(),
            start_date: Utc::now() + chrono::Duration::days(1),
            end_date: Utc::now() + chrono::Duration::days(5),
            registration_deadline: Utc::now() + chrono::Duration::hours(12),
            max_participants: Some(50),
            theme: None,
            rules: None,
            prizes: None,
            organizers: vec!["user-1".to_string()],
        };

        let hackathon = repo.create_hackathon(hackathon_request).await.unwrap();
        let hackathon_id = hackathon.id.id.to_raw();

        // Create submission timeline
        let timeline_request = HackathonTimelineCreateRequestDto {
            phase: HackathonPhase::Submission,
            title: "Submission Phase".to_string(),
            description: Some("Submit your projects".to_string()),
            start_date: Utc::now() + chrono::Duration::days(3),
            end_date: Utc::now() + chrono::Duration::days(4),
            is_active: true,
            order: 3,
        };

        let _timeline = repo.create_hackathon_timeline(hackathon_id.clone(), timeline_request).await.unwrap();

        // Test get submission timeline phase
        let result = repo.get_submission_timeline_phase(hackathon_id.clone()).await;
        assert!(result.is_ok());
        let timeline_option = result.unwrap();
        assert!(timeline_option.is_some());
        let timeline = timeline_option.unwrap();
        assert_eq!(timeline.phase, HackathonPhase::Submission);

        // Cleanup
        let _ = repo.delete_hackathon(hackathon_id).await;
    }

    #[tokio::test]
    async fn test_get_submission_timeline_phase_not_found_repository() {
        let app_state = crate::get_app_state().await;
        let repo = HackathonRepository::new(&app_state);

        // Create test hackathon without submission timeline
        let hackathon_request = HackathonCreateRequestDto {
            name: "No Submission Timeline Test".to_string(),
            description: "Hackathon without submission timeline".to_string(),
            start_date: Utc::now() + chrono::Duration::days(1),
            end_date: Utc::now() + chrono::Duration::days(5),
            registration_deadline: Utc::now() + chrono::Duration::hours(12),
            max_participants: Some(50),
            theme: None,
            rules: None,
            prizes: None,
            organizers: vec!["user-1".to_string()],
        };

        let hackathon = repo.create_hackathon(hackathon_request).await.unwrap();
        let hackathon_id = hackathon.id.id.to_raw();

        // Test get submission timeline phase when none exists
        let result = repo.get_submission_timeline_phase(hackathon_id.clone()).await;
        assert!(result.is_ok());
        let timeline_option = result.unwrap();
        assert!(timeline_option.is_none());

        // Cleanup
        let _ = repo.delete_hackathon(hackathon_id).await;
    }
}