use anyhow::Result;
use chrono::{DateTime, Utc};
use imphnen_dimentorin::v1::sessions::{SessionSchema, SessionsRepository};
use imphnen_iam::v1::users::UsersRepository;
use tests::{
    cleanup_db, generate_unique_email, setup_postgres_test_environment,
};
use sea_orm::{EntityTrait, ActiveModelTrait, Set};
use uuid::Uuid;

/// Helper to create a test SessionSchema
fn create_test_session_schema(
    mentor_id: Uuid,
    mentee_id: Uuid,
) -> SessionSchema {
    let now = Utc::now();
    SessionSchema {
        id: Uuid::new_v4(),
        mentor_id,
        mentee_id,
        topic: "Test Session".to_string(),
        description: Some("This is a test session".to_string()),
        scheduled_at: now,
        duration_minutes: 60,
        meeting_link: Some("https://meet.google.com/test-link".to_string()),
        session_type: "video_call".to_string(),
        status: "pending".to_string(),
        feedback: None,
        rating: None,
        feedback_submitted_at: None,
        created_at: now,
        updated_at: now,
    }
}

#[tokio::test]
async fn test_create_session() -> Result<()> {
    cleanup_db().await;
    let app_state = setup_postgres_test_environment().await;
    let repo = SessionsRepository::new(&app_state);

    // Create test user for mentor and mentee
    let mentor_email = generate_unique_email("test_mentor");
    let mentee_email = generate_unique_email("test_mentee");
    
    let user_repo = UsersRepository::new(&app_state);
    
    // Create mentor user
    let mentor_user = create_test_user(&mentor_email, "Test Mentor", true);
    let mentor_id = mentor_user.id;
    let _ = user_repo.query_create_user(mentor_user).await?;
    
    // Create mentee user
    let mentee_user = create_test_user(&mentee_email, "Test Mentee", true);
    let mentee_id = mentee_user.id;
    let _ = user_repo.query_create_user(mentee_user).await?;

    // Create session
    let session = create_test_session_schema(mentor_id, mentee_id);
    let created_session = repo.create_session(session.clone()).await?;

    assert_eq!(created_session.topic, "Test Session");
    assert_eq!(created_session.status, "pending");
    assert_eq!(created_session.mentor_id, mentor_id);
    assert_eq!(created_session.mentee_id, mentee_id);

    Ok(())
}

#[tokio::test]
async fn test_get_session_by_id() -> Result<()> {
    cleanup_db().await;
    let app_state = setup_postgres_test_environment().await;
    let repo = SessionsRepository::new(&app_state);

    // Create test user for mentor and mentee
    let mentor_email = generate_unique_email("test_mentor_get");
    let mentee_email = generate_unique_email("test_mentee_get");
    
    let user_repo = UsersRepository::new(&app_state);
    
    // Create mentor user
    let mentor_user = create_test_user(&mentor_email, "Test Mentor Get", true);
    let mentor_id = mentor_user.id;
    let _ = user_repo.query_create_user(mentor_user).await?;
    
    // Create mentee user
    let mentee_user = create_test_user(&mentee_email, "Test Mentee Get", true);
    let mentee_id = mentee_user.id;
    let _ = user_repo.query_create_user(mentee_user).await?;

    // Create session
    let session = create_test_session_schema(mentor_id, mentee_id);
    let created_session = repo.create_session(session.clone()).await?;
    
    // Get session by ID
    let retrieved_session = repo.query_session_by_id(&created_session.id.to_string()).await?;
    
    assert!(retrieved_session.is_some());
    let retrieved_session = retrieved_session.unwrap();
    
    assert_eq!(retrieved_session.id, created_session.id);
    assert_eq!(retrieved_session.topic, "Test Session");
    assert_eq!(retrieved_session.status, "pending");

    Ok(())
}

#[tokio::test]
async fn test_update_session() -> Result<()> {
    cleanup_db().await;
    let app_state = setup_postgres_test_environment().await;
    let repo = SessionsRepository::new(&app_state);

    // Create test user for mentor and mentee
    let mentor_email = generate_unique_email("test_mentor_update");
    let mentee_email = generate_unique_email("test_mentee_update");
    
    let user_repo = UsersRepository::new(&app_state);
    
    // Create mentor user
    let mentor_user = create_test_user(&mentor_email, "Test Mentor Update", true);
    let mentor_id = mentor_user.id;
    let _ = user_repo.query_create_user(mentor_user).await?;
    
    // Create mentee user
    let mentee_user = create_test_user(&mentee_email, "Test Mentee Update", true);
    let mentee_id = mentee_user.id;
    let _ = user_repo.query_create_user(mentee_user).await?;

    // Create session
    let session = create_test_session_schema(mentor_id, mentee_id);
    let created_session = repo.create_session(session.clone()).await?;
    
    // Update session
    let mut updated_session = created_session.clone();
    updated_session.topic = "Updated Session Topic".to_string();
    updated_session.status = "confirmed".to_string();
    updated_session.meeting_link = Some("https://meet.google.com/updated-link".to_string());
    
    let saved_session = repo.update_session(&updated_session.id.to_string(), updated_session).await?;
    
    // Verify update
    assert_eq!(saved_session.topic, "Updated Session Topic");
    assert_eq!(saved_session.status, "confirmed");
    assert_eq!(saved_session.meeting_link.unwrap(), "https://meet.google.com/updated-link");

    Ok(())
}

#[tokio::test]
async fn test_delete_session() -> Result<()> {
    cleanup_db().await;
    let app_state = setup_postgres_test_environment().await;
    let repo = SessionsRepository::new(&app_state);

    // Create test user for mentor and mentee
    let mentor_email = generate_unique_email("test_mentor_delete");
    let mentee_email = generate_unique_email("test_mentee_delete");
    
    let user_repo = UsersRepository::new(&app_state);
    
    // Create mentor user
    let mentor_user = create_test_user(&mentor_email, "Test Mentor Delete", true);
    let mentor_id = mentor_user.id;
    let _ = user_repo.query_create_user(mentor_user).await?;
    
    // Create mentee user
    let mentee_user = create_test_user(&mentee_email, "Test Mentee Delete", true);
    let mentee_id = mentee_user.id;
    let _ = user_repo.query_create_user(mentee_user).await?;

    // Create session
    let session = create_test_session_schema(mentor_id, mentee_id);
    let created_session = repo.create_session(session.clone()).await?;
    
    // Delete session
    let delete_result = repo.delete_session(&created_session.id.to_string()).await?;
    assert!(delete_result.is_ok());
    
    // Verify session is deleted
    let retrieved_session = repo.query_session_by_id(&created_session.id.to_string()).await?;
    assert!(retrieved_session.is_none());

    Ok(())
}

#[tokio::test]
async fn test_query_mentor_sessions() -> Result<()> {
    cleanup_db().await;
    let app_state = setup_postgres_test_environment().await;
    let repo = SessionsRepository::new(&app_state);

    // Create test user for mentor and mentees
    let mentor_email = generate_unique_email("test_mentor_sessions");
    let mentee1_email = generate_unique_email("test_mentee1");
    let mentee2_email = generate_unique_email("test_mentee2");
    
    let user_repo = UsersRepository::new(&app_state);
    
    // Create mentor user
    let mentor_user = create_test_user(&mentor_email, "Test Mentor Sessions", true);
    let mentor_id = mentor_user.id;
    let _ = user_repo.query_create_user(mentor_user).await?;
    
    // Create mentee users
    let mentee1_user = create_test_user(&mentee1_email, "Test Mentee 1", true);
    let mentee1_id = mentee1_user.id;
    let _ = user_repo.query_create_user(mentee1_user).await?;
    
    let mentee2_user = create_test_user(&mentee2_email, "Test Mentee 2", true);
    let mentee2_id = mentee2_user.id;
    let _ = user_repo.query_create_user(mentee2_user).await?;

    // Create sessions
    let session1 = create_test_session_schema(mentor_id, mentee1_id);
    let _ = repo.create_session(session1.clone()).await?;
    
    let session2 = create_test_session_schema(mentor_id, mentee2_id);
    let _ = repo.create_session(session2.clone()).await?;

    // Query mentor sessions
    let sessions = repo.query_mentor_sessions(&mentor_id.to_string(), None).await?;
    
    assert_eq!(sessions.len(), 2);
    assert!(sessions.iter().any(|s| s.mentee_id == mentee1_id.to_string()));
    assert!(sessions.iter().any(|s| s.mentee_id == mentee2_id.to_string()));

    Ok(())
}

#[tokio::test]
async fn test_query_user_sessions() -> Result<()> {
    cleanup_db().await;
    let app_state = setup_postgres_test_environment().await;
    let repo = SessionsRepository::new(&app_state);

    // Create test user for mentors and mentee
    let mentor1_email = generate_unique_email("test_mentor1_user");
    let mentor2_email = generate_unique_email("test_mentor2_user");
    let mentee_email = generate_unique_email("test_mentee_user");
    
    let user_repo = UsersRepository::new(&app_state);
    
    // Create mentor users
    let mentor1_user = create_test_user(&mentor1_email, "Test Mentor 1", true);
    let mentor1_id = mentor1_user.id;
    let _ = user_repo.query_create_user(mentor1_user).await?;
    
    let mentor2_user = create_test_user(&mentor2_email, "Test Mentor 2", true);
    let mentor2_id = mentor2_user.id;
    let _ = user_repo.query_create_user(mentor2_user).await?;
    
    // Create mentee user
    let mentee_user = create_test_user(&mentee_email, "Test Mentee User", true);
    let mentee_id = mentee_user.id;
    let _ = user_repo.query_create_user(mentee_user).await?;

    // Create sessions
    let session1 = create_test_session_schema(mentor1_id, mentee_id);
    let _ = repo.create_session(session1.clone()).await?;
    
    let session2 = create_test_session_schema(mentor2_id, mentee_id);
    let _ = repo.create_session(session2.clone()).await?;

    // Query user sessions
    let sessions = repo.query_user_sessions(&mentee_id.to_string(), None).await?;
    
    assert_eq!(sessions.len(), 2);
    assert!(sessions.iter().any(|s| s.mentor_id == mentor1_id.to_string()));
    assert!(sessions.iter().any(|s| s.mentor_id == mentor2_id.to_string()));

    Ok(())
}

#[tokio::test]
async fn test_count_mentor_sessions() -> Result<()> {
    cleanup_db().await;
    let app_state = setup_postgres_test_environment().await;
    let repo = SessionsRepository::new(&app_state);

    // Create test user for mentor and mentees
    let mentor_email = generate_unique_email("test_mentor_count");
    let mentee1_email = generate_unique_email("test_mentee_count1");
    let mentee2_email = generate_unique_email("test_mentee_count2");
    
    let user_repo = UsersRepository::new(&app_state);
    
    // Create mentor user
    let mentor_user = create_test_user(&mentor_email, "Test Mentor Count", true);
    let mentor_id = mentor_user.id;
    let _ = user_repo.query_create_user(mentor_user).await?;
    
    // Create mentee users
    let mentee1_user = create_test_user(&mentee1_email, "Test Mentee Count 1", true);
    let mentee1_id = mentee1_user.id;
    let _ = user_repo.query_create_user(mentee1_user).await?;
    
    let mentee2_user = create_test_user(&mentee2_email, "Test Mentee Count 2", true);
    let mentee2_id = mentee2_user.id;
    let _ = user_repo.query_create_user(mentee2_user).await?;

    // Create sessions
    let session1 = create_test_session_schema(mentor_id, mentee1_id);
    let _ = repo.create_session(session1.clone()).await?;
    
    let session2 = create_test_session_schema(mentor_id, mentee2_id);
    let _ = repo.create_session(session2.clone()).await?;

    // Count mentor sessions
    let count = repo.count_mentor_sessions(&mentor_id.to_string(), None).await?;
    
    assert_eq!(count, 2);

    Ok(())
}

#[tokio::test]
async fn test_count_user_sessions() -> Result<()> {
    cleanup_db().await;
    let app_state = setup_postgres_test_environment().await;
    let repo = SessionsRepository::new(&app_state);

    // Create test user for mentors and mentee
    let mentor1_email = generate_unique_email("test_mentor_count_user");
    let mentor2_email = generate_unique_email("test_mentor_count_user2");
    let mentee_email = generate_unique_email("test_mentee_count_user");
    
    let user_repo = UsersRepository::new(&app_state);
    
    // Create mentor users
    let mentor1_user = create_test_user(&mentor1_email, "Test Mentor Count User 1", true);
    let mentor1_id = mentor1_user.id;
    let _ = user_repo.query_create_user(mentor1_user).await?;
    
    let mentor2_user = create_test_user(&mentor2_email, "Test Mentor Count User 2", true);
    let mentor2_id = mentor2_user.id;
    let _ = user_repo.query_create_user(mentor2_user).await?;
    
    // Create mentee user
    let mentee_user = create_test_user(&mentee_email, "Test Mentee Count User", true);
    let mentee_id = mentee_user.id;
    let _ = user_repo.query_create_user(mentee_user).await?;

    // Create sessions
    let session1 = create_test_session_schema(mentor1_id, mentee_id);
    let _ = repo.create_session(session1.clone()).await?;
    
    let session2 = create_test_session_schema(mentor2_id, mentee_id);
    let _ = repo.create_session(session2.clone()).await?;

    // Count user sessions
    let count = repo.count_user_sessions(&mentee_id.to_string(), None).await?;
    
    assert_eq!(count, 2);

    Ok(())
}

#[tokio::test]
async fn test_query_booked_dates() -> Result<()> {
    cleanup_db().await;
    let app_state = setup_postgres_test_environment().await;
    let repo = SessionsRepository::new(&app_state);

    // Create test user for mentor and mentee
    let mentor_email = generate_unique_email("test_mentor_booked");
    let mentee_email = generate_unique_email("test_mentee_booked");
    
    let user_repo = UsersRepository::new(&app_state);
    
    // Create mentor user
    let mentor_user = create_test_user(&mentor_email, "Test Mentor Booked", true);
    let mentor_id = mentor_user.id;
    let _ = user_repo.query_create_user(mentor_user).await?;
    
    // Create mentee user
    let mentee_user = create_test_user(&mentee_email, "Test Mentee Booked", true);
    let mentee_id = mentee_user.id;
    let _ = user_repo.query_create_user(mentee_user).await?;

    // Create sessions with different statuses
    let mut session1 = create_test_session_schema(mentor_id, mentee_id);
    session1.status = "pending".to_string();
    let _ = repo.create_session(session1.clone()).await?;
    
    let mut session2 = create_test_session_schema(mentor_id, mentee_id);
    session2.status = "confirmed".to_string();
    let _ = repo.create_session(session2.clone()).await?;
    
    let mut session3 = create_test_session_schema(mentor_id, mentee_id);
    session3.status = "completed".to_string(); // This should not be included in booked dates
    let _ = repo.create_session(session3.clone()).await?;

    // Query booked dates
    let booked_dates = repo.query_booked_dates(&mentor_id.to_string()).await?;
    
    // Should only have 2 dates (pending and confirmed)
    assert_eq!(booked_dates.len(), 2);

    Ok(())
}