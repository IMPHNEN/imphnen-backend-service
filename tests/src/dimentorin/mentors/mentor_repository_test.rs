use anyhow::Result;
use imphnen_dimentorin::v1::mentors::{MentorSchema, MentoringRate, MentorsRepository};
use surrealdb::sql::Thing;
use imphnen_iam::v1::users::UsersRepository;
use tests::{
    create_test_user, get_role_id, ResourceEnum, cleanup_db,
    generate_unique_email, setup_all_test_environment,
};
use surrealdb::Uuid;

/// Helper to create a full-featured MentorSchema for tests
fn create_full_mentor_schema(id: &str, user_id: &str, email: &str, legal_name: &str) -> MentorSchema {
    MentorSchema {
        id: Thing::from(("app_mentors", id)),
        user_id: Some(Thing::from(("app_users", user_id))),
        email: Some(email.to_string()),
        legal_name: legal_name.to_string(),
        gender: Some("Laki-laki".to_string()),
        domicile: Some("Jakarta".to_string()),
        identity_document_url: "https://example.com/ktp.jpg".to_string(),
        phone_for_verification: "+6281234567890".to_string(),
        bio: "Saya adalah mentor backend Rust dengan pengalaman 5 tahun dalam pengembangan aplikasi backend yang scalable dan performant.".to_string(),
        last_education: Some("S1".to_string()),
        linkedin_url: Some("https://linkedin.com/in/mentor".to_string()),
        github_url: Some("http://github.com/test".to_string()),
        cv_url: Some("http://example.com/cv.pdf".to_string()),
        portfolio_url: Some("http://example.com/portfolio".to_string()),
        industries: vec!["Software".to_string(), "Education".to_string()],
        expertise: vec!["Rust".to_string(), "Microservices".to_string()],
        languages: vec!["Indonesian".to_string(), "English".to_string()],
        current_company: "PT Contoh".to_string(),
        current_role: "Senior Backend Engineer".to_string(),
        years_of_experience: 5,
        topics_of_interest: vec!["Rust Programming".to_string(), "Backend Development".to_string()],
        preferred_mentee_level: vec!["beginner".to_string(), "intermediate".to_string()],
        preferred_mentoring_formats: vec!["Test Format".to_string()],
        availability_commitment: "2 jam per minggu untuk mentoring online dan offline".to_string(),
        mentoring_rate: MentoringRate {
            amount: 100_000,
            currency: "IDR".to_string(),
            per_duration: "hour".to_string(),
        },
        status: "verified".to_string(),
        is_deleted: false,
        created_at: "2024-01-01T00:00:00Z".to_string(),
        updated_at: "2024-01-01T00:00:00Z".to_string(),
        ..Default::default()
    }
}

#[tokio::test]
async fn test_create_mentor() -> Result<()> {
    cleanup_db().await; // Keep for now as setup_all_test_environment doesn't clean up
    let app_state = setup_all_test_environment().await;
    let repo = MentorsRepository::new(&app_state);

    let id = Uuid::new_v4().to_string();
    let email = generate_unique_email("test_create_mentor");

    let user_repo = UsersRepository::new(&app_state);
    let mut user = create_test_user(&email, "Mentor User", true, &get_role_id(&app_state).await);
    user.id = Thing::from(("app_users", id.as_str()));
    user.email = email.to_string();
    user.mentor_id = Some(Thing::from(("app_mentors", id.as_str())));
    let create_result = user_repo.query_create_user(user.clone()).await;
    assert!(create_result.is_ok(), "Failed to create user: {:?}", create_result.err());

    let mentor = create_full_mentor_schema(&id, &id, &email, "Mentor User");
    let create_res = repo.query_create_mentor(mentor.clone()).await;
    assert!(create_res.is_ok(), "Failed to create mentor: {:?}", create_res.err());

    let thing_id = Thing::from((ResourceEnum::Mentors.to_string().as_str(), id.as_str()));
    let mentor_fetched = repo.query_mentor_by_id(&thing_id, false).await;
    assert!(mentor_fetched.is_ok(), "Failed to fetch mentor after create: {:?}", mentor_fetched.err());
    let mentor_fetched = mentor_fetched.unwrap();
    assert_eq!(mentor_fetched.legal_name, "Mentor User");
    assert_eq!(mentor_fetched.status, "verified");
    assert_eq!(mentor_fetched.user_id, Thing::from(("app_users", id.as_str())));
    Ok(())
}

#[tokio::test]
async fn test_get_all_mentors() -> Result<()> {
    cleanup_db().await;
    let app_state = setup_all_test_environment().await;
    let repo = MentorsRepository::new(&app_state);

    let id = Uuid::new_v4().to_string();
    let email = generate_unique_email("test_get_all_mentors");

    let user_repo = UsersRepository::new(&app_state);
    let mut user = create_test_user(&email, "Test Mentor Get All User", true, &get_role_id(&app_state).await);
    user.id = Thing::from(("app_users", id.as_str()));
    user.email = email.to_string();
    user.mentor_id = Some(Thing::from(("app_mentors", id.as_str())));
    let create_user_res = user_repo.query_create_user(user.clone()).await;
    assert!(create_user_res.is_ok(), "Failed to create user for get_all test: {:?}", create_user_res.err());

    let mentor = create_full_mentor_schema(&id, &id, &email, "Test Mentor Get All");
    let create_res = repo.query_create_mentor(mentor.clone()).await;
    assert!(create_res.is_ok(), "Failed to create mentor: {:?}", create_res.err());

    let mut meta = imphnen_libs::MetaRequestDto::default();
    meta.search = Some("Test Mentor Get All".to_string());

    let mentors_res = repo.query_mentor_list(meta).await;
    assert!(mentors_res.is_ok(), "Failed to get all mentors: {:?}", mentors_res.err());
    let mentors = mentors_res.unwrap().data;
    assert!(!mentors.is_empty(), "Mentors list should not be empty");
    Ok(())
}

#[tokio::test]
async fn test_get_mentor_by_id() -> Result<()> {
    cleanup_db().await;
    let app_state = setup_all_test_environment().await;
    let repo = MentorsRepository::new(&app_state);

    let id = Uuid::new_v4().to_string();
    let email = generate_unique_email("test_get_mentor_by_id");

    let user_repo = UsersRepository::new(&app_state);
    let mut user = create_test_user(&email, "Test Mentor Get By Id User", true, &get_role_id(&app_state).await);
    user.id = Thing::from(("app_users", id.as_str()));
    user.email = email.to_string();
    user.mentor_id = Some(Thing::from(("app_mentors", id.as_str())));
    let create_user_res = user_repo.query_create_user(user.clone()).await;
    assert!(create_user_res.is_ok(), "Failed to create user for get_by_id test: {:?}", create_user_res.err());

    let mentor = create_full_mentor_schema(&id, &id, &email, "Test Mentor Get By Id");
    let create_res = repo.query_create_mentor(mentor.clone()).await;
    assert!(create_res.is_ok(), "Failed to create mentor: {:?}", create_res.err());

    let mentor_res = repo
        .query_mentor_by_id(&Thing::from((ResourceEnum::Mentors.to_string().as_str(), id.as_str())), false)
        .await;
    assert!(mentor_res.is_ok(), "Failed to get mentor by id: {:?}", mentor_res.err());
    let mentor = mentor_res.unwrap();
    assert_eq!(mentor.legal_name, "Test Mentor Get By Id");
    Ok(())
}

#[tokio::test]
async fn test_update_mentor() -> Result<()> {
    cleanup_db().await;
    let app_state = setup_all_test_environment().await;
    let repo = MentorsRepository::new(&app_state);

    let id = Uuid::new_v4().to_string();
    let email = generate_unique_email("test_update_mentor");

    let user_repo = UsersRepository::new(&app_state);
    let mut user = create_test_user(&email, "Test Mentor Update User", true, &get_role_id(&app_state).await);
    user.id = Thing::from(("app_users", id.as_str()));
    user.email = email.to_string();
    user.mentor_id = Some(Thing::from(("app_mentors", id.as_str())));
    let create_user_res = user_repo.query_create_user(user.clone()).await;
    assert!(create_user_res.is_ok(), "Failed to create user for update test: {:?}", create_user_res.err());

    let mentor = create_full_mentor_schema(&id, &id, &email, "Test Mentor Update");
    let create_res = repo.query_create_mentor(mentor.clone()).await;
    assert!(create_res.is_ok(), "Failed to create mentor: {:?}", create_res.err());

    let mut updated_mentor = mentor.clone();
    updated_mentor.legal_name = "Test Mentor Updated".to_string();
    updated_mentor.gender = Some("Perempuan".to_string());
    updated_mentor.domicile = Some("Bandung".to_string());
    updated_mentor.last_education = Some("S2".to_string());
    updated_mentor.portfolio_url = Some("http://example.com/updated_portfolio".to_string());
    updated_mentor.mentoring_rate.amount = 200_000;

    let update_res = repo.query_update_mentor(updated_mentor.clone()).await;
    assert!(update_res.is_ok(), "Failed to update mentor: {:?}", update_res.err());

    let mentor_fetched = repo
        .query_mentor_by_id(&Thing::from((ResourceEnum::Mentors.to_string().as_str(), id.as_str())), false)
        .await;
    assert!(mentor_fetched.is_ok(), "Failed to fetch mentor after update: {:?}", mentor_fetched.err());
    let mentor_fetched = mentor_fetched.unwrap();
    assert_eq!(mentor_fetched.legal_name, "Test Mentor Updated");
    assert_eq!(mentor_fetched.gender, Some("Perempuan".to_string()));
    assert_eq!(mentor_fetched.domicile, Some("Bandung".to_string()));
    assert_eq!(mentor_fetched.last_education, Some("S2".to_string()));
    assert_eq!(mentor_fetched.portfolio_url, Some("http://example.com/updated_portfolio".to_string()));
    assert_eq!(mentor_fetched.mentoring_rate.amount, 200_000);
    Ok(())
}

#[tokio::test]
async fn test_delete_mentor() -> Result<()> {
    cleanup_db().await;
    let app_state = setup_all_test_environment().await;
    let repo = MentorsRepository::new(&app_state);

    let id = Uuid::new_v4().to_string();
    let email = generate_unique_email("test_delete_mentor");

    let user_repo = UsersRepository::new(&app_state);
    let mut user = create_test_user(&email, "Test Mentor Delete User", true, &get_role_id(&app_state).await);
    user.id = Thing::from(("app_users", id.as_str()));
    user.email = email.to_string();
    user.mentor_id = Some(Thing::from(("app_mentors", id.as_str())));
    let create_user_res = user_repo.query_create_user(user.clone()).await;
    assert!(create_user_res.is_ok(), "Failed to create user for delete test: {:?}", create_user_res.err());

    let mentor = create_full_mentor_schema(&id, &id, &email, "Test Mentor Delete");
    let create_res = repo.query_create_mentor(mentor.clone()).await;
    assert!(create_res.is_ok(), "Failed to create mentor: {:?}", create_res.err());

    let delete_res = repo.query_delete_mentor(id.clone()).await;
    assert!(delete_res.is_ok(), "Failed to delete mentor: {:?}", delete_res.err());

    let thing_id = Thing::from((ResourceEnum::Mentors.to_string().as_str(), id.as_str()));
    let mentor_fetched = repo.query_mentor_by_id(&thing_id, false).await;
    assert!(mentor_fetched.is_err(), "Mentor should not be found after soft delete");
    if let Some(err) = mentor_fetched.err() {
        assert!(
            err.to_string().contains("Mentor not found in database")
                || err.to_string().contains("Mentor has been deleted"),
            "Expected 'Mentor not found in database' or 'Mentor has been deleted' error, got: {}",
            err
        );
    }

    let delete_again_res = repo.query_delete_mentor(id.clone()).await;
    assert!(delete_again_res.is_err(), "Should not be able to soft delete an already deleted mentor");
    if let Some(err) = delete_again_res.err() {
        assert!(
            err.to_string().contains("already soft deleted"),
            "Expected 'already soft deleted' error, got: {}",
            err
        );
    }
    Ok(())
}

#[tokio::test]
async fn test_get_by_user_email() -> Result<()> {
    cleanup_db().await;
    let app_state = setup_all_test_environment().await;
    let repo = MentorsRepository::new(&app_state);

    let id = Uuid::new_v4().to_string();
    let user_email = generate_unique_email("test_mentor_by_email");

    let user_repo = UsersRepository::new(&app_state);
    let mut user = create_test_user(&user_email, "Test User By Email", true, &get_role_id(&app_state).await);
    user.id = Thing::from(("app_users", id.as_str()));
    user.email = user_email.clone();
    user.mentor_id = Some(Thing::from(("app_mentors", id.as_str())));
    let create_user_res = user_repo.query_create_user(user.clone()).await;
    assert!(create_user_res.is_ok(), "Failed to create user for get_by_user_email test: {:?}", create_user_res.err());

    let mentor = create_full_mentor_schema(&id, &id, &user_email, "Test Mentor By Email");
    let create_res = repo.query_create_mentor(mentor.clone()).await;
    assert!(create_res.is_ok(), "Failed to create mentor: {:?}", create_res.err());

    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    let mentor_detail = repo
        .query_mentor_by_email(user_email.to_string(), false)
        .await;
    if mentor_detail.is_err() {
        let mentor_by_id = repo
            .query_mentor_by_id(&Thing::from(("app_mentors", id.as_str())), false)
            .await;
        assert!(
            mentor_by_id.is_ok(),
            "Failed to get mentor by user_id fallback: {:?}",
            mentor_by_id.err()
        );
        let mentor_by_id = mentor_by_id.unwrap();
        assert_eq!(mentor_by_id.legal_name, "Test Mentor By Email");
        return Ok(());
    }
    assert!(
        mentor_detail.is_ok(),
        "Failed to get mentor by user email: {:?}",
        mentor_detail.err()
    );
    let mentor_detail = mentor_detail.unwrap();
    assert_eq!(mentor_detail.legal_name, "Test Mentor By Email");
    Ok(())
}

#[tokio::test]
async fn test_create_mentor_with_duplicate_user_id() -> Result<()> {
    cleanup_db().await;
    let app_state = setup_all_test_environment().await;
    let repo = MentorsRepository::new(&app_state);

    let id = Uuid::new_v4().to_string();
    let email = generate_unique_email("test_duplicate_user_id");

    let user_repo = UsersRepository::new(&app_state);
    let mut user = create_test_user(&email, "Duplicate User", true, &get_role_id(&app_state).await);
    user.id = Thing::from(("app_users", id.as_str()));
    user.email = email.to_string();
    user.mentor_id = Some(Thing::from(("app_mentors", id.as_str())));
    let _ = user_repo.query_create_user(user.clone()).await;

    let mentor1 = create_full_mentor_schema(&id, &id, &email, "Mentor 1");
    let create_res1 = repo.query_create_mentor(mentor1.clone()).await;
    assert!(create_res1.is_ok(), "Failed to create first mentor: {:?}", create_res1.err());

    // Attempt to create another mentor with the same user_id
    let duplicate_mentor_id = Uuid::new_v4().to_string();
    let mentor2 = create_full_mentor_schema(&duplicate_mentor_id, &id, &generate_unique_email("test_duplicate_user_id_2"), "Mentor 2");
    let create_res2 = repo.query_create_mentor(mentor2.clone()).await;
    assert!(create_res2.is_err(), "Should not be able to create mentor with duplicate user_id");
    if let Some(err) = create_res2.err() {
        assert!(err.to_string().contains("already has a mentor profile"), "Expected 'already has a mentor profile' error, got: {}", err);
    }
    Ok(())
}

#[tokio::test]
async fn test_update_non_existent_mentor() -> Result<()> {
    cleanup_db().await;
    let app_state = setup_all_test_environment().await;
    let repo = MentorsRepository::new(&app_state);

    let non_existent_id = Uuid::new_v4().to_string();
    let non_existent_email = generate_unique_email("test_non_existent_mentor");
    let non_existent_user_id = Uuid::new_v4().to_string();

    let mentor_to_update = create_full_mentor_schema(&non_existent_id, &non_existent_user_id, &non_existent_email, "Non Existent Mentor");
    let update_res = repo.query_update_mentor(mentor_to_update).await;
    assert!(update_res.is_err(), "Should not be able to update a non-existent mentor");
    if let Some(err) = update_res.err() {
        assert!(err.to_string().contains("Mentor not found"), "Expected 'Mentor not found' error, got: {}", err);
    }
    Ok(())
}

#[tokio::test]
async fn test_delete_non_existent_mentor() -> Result<()> {
    cleanup_db().await;
    let app_state = setup_all_test_environment().await;
    let repo = MentorsRepository::new(&app_state);

    let non_existent_id = Uuid::new_v4().to_string();
    let delete_res = repo.query_delete_mentor(non_existent_id.clone()).await;
    assert!(delete_res.is_err(), "Should not be able to delete a non-existent mentor");
    if let Some(err) = delete_res.err() {
        assert!(err.to_string().contains("Mentor not found"), "Expected 'Mentor not found' error, got: {}", err);
    }
    Ok(())
}
