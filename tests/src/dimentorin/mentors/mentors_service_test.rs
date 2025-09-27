#[cfg(test)]
mod tests {
	use crate::{generate_unique_email, get_role_id, UsersRepository, setup_all_test_environment};
	use axum::{http::StatusCode, response::Response};
	use imphnen_dimentorin::{
		mentors_service::MentorsService,
		mentors_dto::{
			MentorUserRegisterRequestDto, MentorUpdateRequestDto, MentorVerifyRequestDto,
			IdentityAndVerification, ProfessionalProfile, MentoringLogistics
		},
		MentorsRepository
	};
	use imphnen_entities::{AppState, MetaRequestDto};
	use imphnen_iam::{RolesEnum};
	use imphnen_utils::{generate_otp, hash_password, make_thing_from_enum, get_iso_date};
	use surrealdb::Uuid;

	#[tokio::test]
	async fn test_register_mentor_service() {
		let app_state = setup_all_test_environment().await;
		let user_repo = UsersRepository::new(&app_state);
		let mentor_repo = MentorsRepository::new(&app_state);
		let role_repo = imphnen_iam::RolesRepository::new(&app_state);
		let auth_repo = imphnen_iam::AuthRepository::new(&app_state);

		// Test data
		let email = generate_unique_email("test_register_mentor");
		let password = "Password123!".to_string();

		let mentor_dto = MentorUserRegisterRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test Mentor Service".to_string(),
			phone_number: "1234567890".to_string(),
			identity_and_verification: IdentityAndVerification {
				legal_name: "Legal Test Name".to_string(),
				gender: Some("Laki-laki".to_string()),
				domicile: Some("Jakarta Selatan".to_string()),
				identity_document_url: "http://example.com/id.pdf".to_string(),
				phone_for_verification: "0987654321".to_string(),
			},
			professional_profile: ProfessionalProfile {
				bio: "Experienced professional with 5+ years of experience in software development.".to_string(),
				last_education: Some("S1".to_string()),
				linkedin_url: Some("http://linkedin.com/in/test".to_string()),
				github_url: None,
				cv_url: None,
				portfolio_url: Some("http://example.com/portfolio".to_string()),
				industries: vec!["Technology".to_string()],
				expertise: vec!["Rust".to_string(), "Backend Development".to_string()],
				languages: vec!["English".to_string()],
				current_company: "Tech Corp".to_string(),
				current_role: "Senior Engineer".to_string(),
				years_of_experience: 5,
			},
			mentoring_logistics: MentoringLogistics {
				topics_of_interest: vec!["Career Development".to_string()],
				preferred_mentee_level: vec!["Beginner".to_string()],
				preferred_mentoring_formats: vec!["Online".to_string()],
				availability_commitment: "5 hours/week".to_string(),
				mentoring_rate_amount: 100,
			},
		};

		// Register mentor
		let response = MentorsService::register_mentor(&app_state, mentor_dto.clone()).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);

		// Verify mentor was created in database
		let mentor = mentor_repo.query_mentor_by_email(email.clone(), false).await;
		assert!(mentor.is_ok());
		assert_eq!(mentor.unwrap().status, "pending".to_string());

		// Verify user was updated in database
		let user = user_repo.query_user_by_email(email.clone()).await;
		assert!(user.is_ok());
		assert_eq!(user.unwrap().is_active, false);

		// Clean up
		let user = user.unwrap();
		let _ = user_repo.query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_get_mentor_list_service() {
		let app_state = setup_all_test_environment().await;
		let mentor_repo = MentorsRepository::new(&app_state);
		let user_repo = UsersRepository::new(&app_state);

		// Create test mentor first
		let email = generate_unique_email("test_mentor_list");
		let password = "Password123!".to_string();

		let mentor_dto = MentorUserRegisterRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test Mentor List".to_string(),
			phone_number: "1234567890".to_string(),
			identity_and_verification: IdentityAndVerification {
				legal_name: "Legal Test Name".to_string(),
				gender: Some("Laki-laki".to_string()),
				domicile: Some("Jakarta Selatan".to_string()),
				identity_document_url: "http://example.com/id.pdf".to_string(),
				phone_for_verification: "0987654321".to_string(),
			},
			professional_profile: ProfessionalProfile {
				bio: "Experienced professional with 5+ years of experience in software development.".to_string(),
				last_education: Some("S1".to_string()),
				linkedin_url: Some("http://linkedin.com/in/test".to_string()),
				github_url: None,
				cv_url: None,
				portfolio_url: Some("http://example.com/portfolio".to_string()),
				industries: vec!["Technology".to_string()],
				expertise: vec!["Rust".to_string(), "Backend Development".to_string()],
				languages: vec!["English".to_string()],
				current_company: "Tech Corp".to_string(),
				current_role: "Senior Engineer".to_string(),
				years_of_experience: 5,
			},
			mentoring_logistics: MentoringLogistics {
				topics_of_interest: vec!["Career Development".to_string()],
				preferred_mentee_level: vec!["Beginner".to_string()],
				preferred_mentoring_formats: vec!["Online".to_string()],
				availability_commitment: "5 hours/week".to_string(),
				mentoring_rate_amount: 100,
			},
		};

		let _ = MentorsService::register_mentor(&app_state, mentor_dto).await;

		// Get mentor list
		let meta = MetaRequestDto {
			limit: 10,
			page: 1,
			search: None,
			sort: None,
			filter: None,
		};

		let response = MentorsService::get_mentor_list(&app_state, meta).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);

		// Clean up
		let user = user_repo.query_user_by_email(email.clone()).await.unwrap();
		let _ = user_repo.query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_get_mentor_by_id_service() {
		let app_state = setup_all_test_environment().await;
		let mentor_repo = MentorsRepository::new(&app_state);
		let user_repo = UsersRepository::new(&app_state);

		// Create test mentor first
		let email = generate_unique_email("test_mentor_by_id");
		let password = "Password123!".to_string();

		let mentor_dto = MentorUserRegisterRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test Mentor By ID".to_string(),
			phone_number: "1234567890".to_string(),
			identity_and_verification: IdentityAndVerification {
				legal_name: "Legal Test Name".to_string(),
				gender: Some("Laki-laki".to_string()),
				domicile: Some("Jakarta Selatan".to_string()),
				identity_document_url: "http://example.com/id.pdf".to_string(),
				phone_for_verification: "0987654321".to_string(),
			},
			professional_profile: ProfessionalProfile {
				bio: "Experienced professional with 5+ years of experience in software development.".to_string(),
				last_education: Some("S1".to_string()),
				linkedin_url: Some("http://linkedin.com/in/test".to_string()),
				github_url: None,
				cv_url: None,
				portfolio_url: Some("http://example.com/portfolio".to_string()),
				industries: vec!["Technology".to_string()],
				expertise: vec!["Rust".to_string(), "Backend Development".to_string()],
				languages: vec!["English".to_string()],
				current_company: "Tech Corp".to_string(),
				current_role: "Senior Engineer".to_string(),
				years_of_experience: 5,
			},
			mentoring_logistics: MentoringLogistics {
				topics_of_interest: vec!["Career Development".to_string()],
				preferred_mentee_level: vec!["Beginner".to_string()],
				preferred_mentoring_formats: vec!["Online".to_string()],
				availability_commitment: "5 hours/week".to_string(),
				mentoring_rate_amount: 100,
			},
		};

		let register_response = MentorsService::register_mentor(&app_state, mentor_dto).await;
		assert_eq!(register_response.status(), StatusCode::OK);

		let mentor = mentor_repo.query_mentor_by_email(email.clone(), false).await.unwrap();
		let mentor_id = mentor.id.id.to_raw();

		// Get mentor by ID
		let response = MentorsService::get_mentor_by_id(&app_state, mentor_id).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);

		// Clean up
		let user = user_repo.query_user_by_email(email.clone()).await.unwrap();
		let _ = user_repo.query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_update_mentor_service() {
		let app_state = setup_all_test_environment().await;
		let mentor_repo = MentorsRepository::new(&app_state);
		let user_repo = UsersRepository::new(&app_state);

		// Create test mentor first
		let email = generate_unique_email("test_update_mentor");
		let password = "Password123!".to_string();

		let mentor_dto = MentorUserRegisterRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test Mentor Update".to_string(),
			phone_number: "1234567890".to_string(),
			identity_and_verification: IdentityAndVerification {
				legal_name: "Legal Test Name".to_string(),
				gender: Some("Laki-laki".to_string()),
				domicile: Some("Jakarta Selatan".to_string()),
				identity_document_url: "http://example.com/id.pdf".to_string(),
				phone_for_verification: "0987654321".to_string(),
			},
			professional_profile: ProfessionalProfile {
				bio: "Experienced professional with 5+ years of experience in software development.".to_string(),
				last_education: Some("S1".to_string()),
				linkedin_url: Some("http://linkedin.com/in/test".to_string()),
				github_url: None,
				cv_url: None,
				portfolio_url: Some("http://example.com/portfolio".to_string()),
				industries: vec!["Technology".to_string()],
				expertise: vec!["Rust".to_string(), "Backend Development".to_string()],
				languages: vec!["English".to_string()],
				current_company: "Tech Corp".to_string(),
				current_role: "Senior Engineer".to_string(),
				years_of_experience: 5,
			},
			mentoring_logistics: MentoringLogistics {
				topics_of_interest: vec!["Career Development".to_string()],
				preferred_mentee_level: vec!["Beginner".to_string()],
				preferred_mentoring_formats: vec!["Online".to_string()],
				availability_commitment: "5 hours/week".to_string(),
				mentoring_rate_amount: 100,
			},
		};

		let _ = MentorsService::register_mentor(&app_state, mentor_dto).await;

		let mentor = mentor_repo.query_mentor_by_email(email.clone(), false).await.unwrap();
		let mentor_id = mentor.id.id.to_raw();

		// Prepare update request
		let update_dto = MentorUpdateRequestDto {
			legal_name: Some("Updated Legal Name".to_string()),
			gender: Some("Perempuan".to_string()),
			domicile: Some("Bandung".to_string()),
			phone_for_verification: Some("0876543210".to_string()),
			bio: Some("Updated bio with more experience.".to_string()),
			last_education: Some("S2".to_string()),
			linkedin_url: Some("http://linkedin.com/in/updated".to_string()),
			github_url: Some("http://github.com/updated".to_string()),
			cv_url: Some("http://example.com/updated_cv.pdf".to_string()),
			portfolio_url: Some("http://example.com/updated_portfolio".to_string()),
			industries: Some(vec!["Technology".to_string(), "Education".to_string()]),
			expertise: Some(vec!["Rust".to_string(), "AI".to_string()]),
			languages: Some(vec!["English".to_string(), "Spanish".to_string()]),
			current_company: Some("New Tech Corp".to_string()),
			current_role: Some("Lead Engineer".to_string()),
			years_of_experience: Some(7),
			topics_of_interest: Some(vec!["Career Development".to_string(), "Tech Trends".to_string()]),
			preferred_mentee_level: Some(vec!["Beginner".to_string(), "Intermediate".to_string()]),
			preferred_mentoring_formats: Some(vec!["Online".to_string(), "Offline".to_string()]),
			availability_commitment: Some("10 hours/week".to_string()),
			mentoring_rate_amount: Some(200),
		};

		// Update mentor
		let response = MentorsService::update_mentor(&app_state, mentor_id, update_dto).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);

		// Verify mentor was updated
		let updated_mentor = mentor_repo.query_mentor_by_id(&mentor.id, false).await.unwrap();
		assert_eq!(updated_mentor.legal_name, Some("Updated Legal Name".to_string()));
		assert_eq!(updated_mentor.current_role, "Lead Engineer".to_string());

		// Clean up
		let user = user_repo.query_user_by_email(email.clone()).await.unwrap();
		let _ = user_repo.query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_delete_mentor_service() {
		let app_state = setup_all_test_environment().await;
		let mentor_repo = MentorsRepository::new(&app_state);
		let user_repo = UsersRepository::new(&app_state);

		// Create test mentor first
		let email = generate_unique_email("test_delete_mentor");
		let password = "Password123!".to_string();

		let mentor_dto = MentorUserRegisterRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test Mentor Delete".to_string(),
			phone_number: "1234567890".to_string(),
			identity_and_verification: IdentityAndVerification {
				legal_name: "Legal Test Name".to_string(),
				gender: Some("Laki-laki".to_string()),
				domicile: Some("Jakarta Selatan".to_string()),
				identity_document_url: "http://example.com/id.pdf".to_string(),
				phone_for_verification: "0987654321".to_string(),
			},
			professional_profile: ProfessionalProfile {
				bio: "Experienced professional with 5+ years of experience in software development.".to_string(),
				last_education: Some("S1".to_string()),
				linkedin_url: Some("http://linkedin.com/in/test".to_string()),
				github_url: None,
				cv_url: None,
				portfolio_url: Some("http://example.com/portfolio".to_string()),
				industries: vec!["Technology".to_string()],
				expertise: vec!["Rust".to_string(), "Backend Development".to_string()],
				languages: vec!["English".to_string()],
				current_company: "Tech Corp".to_string(),
				current_role: "Senior Engineer".to_string(),
				years_of_experience: 5,
			},
			mentoring_logistics: MentoringLogistics {
				topics_of_interest: vec!["Career Development".to_string()],
				preferred_mentee_level: vec!["Beginner".to_string()],
				preferred_mentoring_formats: vec!["Online".to_string()],
				availability_commitment: "5 hours/week".to_string(),
				mentoring_rate_amount: 100,
			},
		};

		let _ = MentorsService::register_mentor(&app_state, mentor_dto).await;

		let mentor = mentor_repo.query_mentor_by_email(email.clone(), false).await.unwrap();
		let mentor_id = mentor.id.id.to_raw();

		// Delete mentor
		let response = MentorsService::delete_mentor(&app_state, mentor_id).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);

		// Clean up
		let user = user_repo.query_user_by_email(email.clone()).await.unwrap();
		let _ = user_repo.query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_verify_mentor_service() {
		let app_state = setup_all_test_environment().await;
		let mentor_repo = MentorsRepository::new(&app_state);
		let user_repo = UsersRepository::new(&app_state);

		// Create test mentor first
		let email = generate_unique_email("test_verify_mentor");
		let password = "Password123!".to_string();

		let mentor_dto = MentorUserRegisterRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test Mentor Verify".to_string(),
			phone_number: "1234567890".to_string(),
			identity_and_verification: IdentityAndVerification {
				legal_name: "Legal Test Name".to_string(),
				gender: Some("Laki-laki".to_string()),
				domicile: Some("Jakarta Selatan".to_string()),
				identity_document_url: "http://example.com/id.pdf".to_string(),
				phone_for_verification: "0987654321".to_string(),
			},
			professional_profile: ProfessionalProfile {
				bio: "Experienced professional with 5+ years of experience in software development.".to_string(),
				last_education: Some("S1".to_string()),
				linkedin_url: Some("http://linkedin.com/in/test".to_string()),
				github_url: None,
				cv_url: None,
				portfolio_url: Some("http://example.com/portfolio".to_string()),
				industries: vec!["Technology".to_string()],
				expertise: vec!["Rust".to_string(), "Backend Development".to_string()],
				languages: vec!["English".to_string()],
				current_company: "Tech Corp".to_string(),
				current_role: "Senior Engineer".to_string(),
				years_of_experience: 5,
			},
			mentoring_logistics: MentoringLogistics {
				topics_of_interest: vec!["Career Development".to_string()],
				preferred_mentee_level: vec!["Beginner".to_string()],
				preferred_mentoring_formats: vec!["Online".to_string()],
				availability_commitment: "5 hours/week".to_string(),
				mentoring_rate_amount: 100,
			},
		};

		let _ = MentorsService::register_mentor(&app_state, mentor_dto).await;

		let mentor = mentor_repo.query_mentor_by_email(email.clone(), false).await.unwrap();
		let mentor_id = mentor.id.id.to_raw();

		// Prepare verification request
		let verify_dto = MentorVerifyRequestDto {
			status: "verified".to_string(),
		};

		// Verify mentor
		let response = MentorsService::verify_mentor(&app_state, mentor_id, verify_dto).await;

		// Verify response
		assert_eq!(response.status(), StatusCode::OK);

		// Verify mentor was verified
		let updated_mentor = mentor_repo.query_mentor_by_id(&mentor.id, false).await.unwrap();
		assert_eq!(updated_mentor.status, "verified".to_string());

		// Clean up
		let user = user_repo.query_user_by_email(email.clone()).await.unwrap();
		let _ = user_repo.query_delete_user(user.id.id.to_raw()).await;
	}
	#[tokio::test]
	async fn test_get_mentor_me_not_found() {
		let app_state = setup_all_test_environment().await;

		// Try to get mentor me for non-existent email
		let response = MentorsService::get_mentor_me(&app_state, "nonexistent@example.com").await;

		// Should return forbidden (mentor profile not found)
		assert_eq!(response.status(), StatusCode::FORBIDDEN);
	}

	#[tokio::test]
	async fn test_update_mentor_me_not_found() {
		let app_state = setup_all_test_environment().await;

		// Try to update mentor me for non-existent email
		let update_dto = MentorUpdateRequestDto {
			legal_name: Some("Test".to_string()),
			..Default::default()
		};

		let response = MentorsService::update_mentor_me(&app_state, "nonexistent@example.com", update_dto).await;

		// Should return forbidden
		assert_eq!(response.status(), StatusCode::FORBIDDEN);
	}

	#[tokio::test]
	async fn test_get_mentor_status_not_found() {
		let app_state = setup_all_test_environment().await;

		// Try to get mentor status for non-existent email
		let response = MentorsService::get_mentor_status(&app_state, "nonexistent@example.com").await;

		// Should return forbidden
		assert_eq!(response.status(), StatusCode::FORBIDDEN);
	}

	#[tokio::test]
	async fn test_get_mentor_by_id_not_found() {
		let app_state = setup_all_test_environment().await;

		// Try to get non-existent mentor by ID
		let response = MentorsService::get_mentor_by_id(&app_state, "nonexistent_id").await;

		// Should return not found
		assert_eq!(response.status(), StatusCode::NOT_FOUND);
	}

	#[tokio::test]
	async fn test_update_mentor_not_found() {
		let app_state = setup_all_test_environment().await;

		// Try to update non-existent mentor
		let update_dto = MentorUpdateRequestDto {
			legal_name: Some("Test".to_string()),
			..Default::default()
		};

		let response = MentorsService::update_mentor(&app_state, "nonexistent_id", update_dto).await;

		// Should return not found
		assert_eq!(response.status(), StatusCode::NOT_FOUND);
	}

	#[tokio::test]
	async fn test_verify_mentor_not_found() {
		let app_state = setup_all_test_environment().await;

		// Try to verify non-existent mentor
		let verify_dto = MentorVerifyRequestDto {
			status: "verified".to_string(),
		};

		let response = MentorsService::verify_mentor(&app_state, "nonexistent_id", verify_dto).await;

		// Should return not found
		assert_eq!(response.status(), StatusCode::NOT_FOUND);
	}

	#[tokio::test]
	async fn test_delete_mentor_not_found() {
		let app_state = setup_all_test_environment().await;

		// Try to delete non-existent mentor
		let response = MentorsService::delete_mentor(&app_state, "nonexistent_id").await;

		// Should return not found
		assert_eq!(response.status(), StatusCode::NOT_FOUND);
	}

	#[tokio::test]
	async fn test_register_mentor_validation_error() {
		let app_state = setup_all_test_environment().await;

		// Create invalid mentor DTO
		let invalid_dto = MentorUserRegisterRequestDto {
			email: "invalid-email".to_string(), // Invalid email
			password: "weak".to_string(), // Weak password
			fullname: "".to_string(), // Empty fullname
			phone_number: "123".to_string(), // Too short phone
			identity_and_verification: IdentityAndVerification {
				legal_name: "ab".to_string(), // Too short legal name
				gender: Some("Laki-laki".to_string()),
				domicile: Some("Jakarta".to_string()),
				identity_document_url: "not-a-url".to_string(), // Invalid URL
				phone_for_verification: "123".to_string(), // Too short
			},
			professional_profile: ProfessionalProfile {
				bio: "short".to_string(), // Too short bio
				last_education: Some("S1".to_string()),
				linkedin_url: Some("not-a-url".to_string()), // Invalid URL
				github_url: Some("invalid-url".to_string()), // Invalid URL
				cv_url: Some("bad-url".to_string()), // Invalid URL
				portfolio_url: Some("wrong-url".to_string()), // Invalid URL
				industries: vec![], // Empty array
				expertise: vec![], // Empty array
				languages: vec![], // Empty array
				current_company: "".to_string(), // Empty company
				current_role: "".to_string(), // Empty role
				years_of_experience: 1, // Too low
			},
			mentoring_logistics: MentoringLogistics {
				topics_of_interest: vec![], // Empty array
				preferred_mentee_level: vec![], // Empty array
				preferred_mentoring_formats: vec![], // Empty array
				availability_commitment: "1234".to_string(), // Too short
				mentoring_rate_amount: 0, // Too low
			},
		};

		let response = MentorsService::register_mentor(&app_state, invalid_dto).await;

		// Should return bad request due to validation
		assert_eq!(response.status(), StatusCode::BAD_REQUEST);
	}

	#[tokio::test]
	async fn test_update_mentor_validation_error() {
		let app_state = setup_all_test_environment().await;
		let user_repo = UsersRepository::new(&app_state);
		let mentor_repo = MentorsRepository::new(&app_state);

		// Create a valid mentor first
		let email = generate_unique_email("test_update_validation");
		let password = "Password123!".to_string();

		let mentor_dto = MentorUserRegisterRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test Mentor Validation".to_string(),
			phone_number: "1234567890".to_string(),
			identity_and_verification: IdentityAndVerification {
				legal_name: "Legal Test Name".to_string(),
				gender: Some("Laki-laki".to_string()),
				domicile: Some("Jakarta".to_string()),
				identity_document_url: "http://example.com/id.pdf".to_string(),
				phone_for_verification: "0987654321".to_string(),
			},
			professional_profile: ProfessionalProfile {
				bio: "Experienced professional with 5+ years of experience in software development.".to_string(),
				last_education: Some("S1".to_string()),
				linkedin_url: Some("http://linkedin.com/in/test".to_string()),
				github_url: None,
				cv_url: None,
				portfolio_url: Some("http://example.com/portfolio".to_string()),
				industries: vec!["Technology".to_string()],
				expertise: vec!["Rust".to_string()],
				languages: vec!["English".to_string()],
				current_company: "Tech Corp".to_string(),
				current_role: "Senior Engineer".to_string(),
				years_of_experience: 5,
			},
			mentoring_logistics: MentoringLogistics {
				topics_of_interest: vec!["Career Development".to_string()],
				preferred_mentee_level: vec!["Beginner".to_string()],
				preferred_mentoring_formats: vec!["Online".to_string()],
				availability_commitment: "5 hours/week".to_string(),
				mentoring_rate_amount: 100,
			},
		};

		let _ = MentorsService::register_mentor(&app_state, mentor_dto).await;

		let mentor = mentor_repo.query_mentor_by_email(email.clone(), false).await.unwrap();
		let mentor_id = mentor.id.id.to_raw();

		// Try to update with invalid data
		let invalid_update_dto = MentorUpdateRequestDto {
			legal_name: Some("ab".to_string()), // Too short
			bio: Some("short".to_string()), // Too short
			linkedin_url: Some("not-a-url".to_string()), // Invalid URL
			industries: Some(vec![]), // Empty array
			years_of_experience: Some(1), // Too low
			mentoring_rate_amount: Some(0), // Too low
			..Default::default()
		};

		let response = MentorsService::update_mentor(&app_state, mentor_id, invalid_update_dto).await;

		// Should return bad request
		assert_eq!(response.status(), StatusCode::BAD_REQUEST);

		// Clean up
		let user = user_repo.query_user_by_email(email.clone()).await.unwrap();
		let _ = user_repo.query_delete_user(user.id.id.to_raw()).await;
	}
}
	#[tokio::test]
	async fn test_register_mentor_duplicate_email() {
		let app_state = setup_all_test_environment().await;
		let user_repo = UsersRepository::new(&app_state);

		let email = generate_unique_email("test_duplicate_email");
		let password = "Password123!".to_string();

		let mentor_dto = MentorUserRegisterRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test Mentor Duplicate".to_string(),
			phone_number: "1234567890".to_string(),
			identity_and_verification: IdentityAndVerification {
				legal_name: "Legal Test Name".to_string(),
				gender: Some("Laki-laki".to_string()),
				domicile: Some("Jakarta".to_string()),
				identity_document_url: "http://example.com/id.pdf".to_string(),
				phone_for_verification: "0987654321".to_string(),
			},
			professional_profile: ProfessionalProfile {
				bio: "Experienced professional with 5+ years of experience in software development.".to_string(),
				last_education: Some("S1".to_string()),
				linkedin_url: Some("http://linkedin.com/in/test".to_string()),
				github_url: None,
				cv_url: None,
				portfolio_url: Some("http://example.com/portfolio".to_string()),
				industries: vec!["Technology".to_string()],
				expertise: vec!["Rust".to_string()],
				languages: vec!["English".to_string()],
				current_company: "Tech Corp".to_string(),
				current_role: "Senior Engineer".to_string(),
				years_of_experience: 5,
			},
			mentoring_logistics: MentoringLogistics {
				topics_of_interest: vec!["Career Development".to_string()],
				preferred_mentee_level: vec!["Beginner".to_string()],
				preferred_mentoring_formats: vec!["Online".to_string()],
				availability_commitment: "5 hours/week".to_string(),
				mentoring_rate_amount: 100,
			},
		};

		// Register first mentor
		let response1 = MentorsService::register_mentor(&app_state, mentor_dto.clone()).await;
		assert_eq!(response1.status(), StatusCode::OK);

		// Try to register second mentor with same email
		let response2 = MentorsService::register_mentor(&app_state, mentor_dto).await;
		assert_eq!(response2.status(), StatusCode::BAD_REQUEST);

		// Clean up
		let user = user_repo.query_user_by_email(email.clone()).await.unwrap();
		let _ = user_repo.query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_register_mentor_boundary_values() {
		let app_state = setup_all_test_environment().await;
		let user_repo = UsersRepository::new(&app_state);

		let email = generate_unique_email("test_boundary_values");
		let password = "Password123!".to_string();

		let mentor_dto = MentorUserRegisterRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test Mentor Boundary".to_string(),
			phone_number: "1234567890".to_string(),
			identity_and_verification: IdentityAndVerification {
				legal_name: "abc".to_string(), // Exactly 3 chars
				gender: Some("Laki-laki".to_string()),
				domicile: Some("Jakarta".to_string()),
				identity_document_url: "http://example.com/id.pdf".to_string(),
				phone_for_verification: "0987654321".to_string(),
			},
			professional_profile: ProfessionalProfile {
				bio: "A".repeat(50), // Exactly 50 chars
				last_education: Some("S1".to_string()),
				linkedin_url: Some("http://linkedin.com/in/test".to_string()),
				github_url: None,
				cv_url: None,
				portfolio_url: Some("http://example.com/portfolio".to_string()),
				industries: vec!["Technology".to_string()],
				expertise: vec!["Rust".to_string()],
				languages: vec!["English".to_string()],
				current_company: "Tech Corp".to_string(),
				current_role: "Senior Engineer".to_string(),
				years_of_experience: 2, // Exactly 2
			},
			mentoring_logistics: MentoringLogistics {
				topics_of_interest: vec!["Career Development".to_string()],
				preferred_mentee_level: vec!["Beginner".to_string()],
				preferred_mentoring_formats: vec!["Online".to_string()],
				availability_commitment: "12345".to_string(), // Exactly 5 chars
				mentoring_rate_amount: 1, // Exactly 1
			},
		};

		let response = MentorsService::register_mentor(&app_state, mentor_dto).await;
		assert_eq!(response.status(), StatusCode::OK);

		// Clean up
		let user = user_repo.query_user_by_email(email.clone()).await.unwrap();
		let _ = user_repo.query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_register_mentor_extreme_values() {
		let app_state = setup_all_test_environment().await;
		let user_repo = UsersRepository::new(&app_state);

		let email = generate_unique_email("test_extreme_values");
		let password = "Password123!".to_string();

		let mentor_dto = MentorUserRegisterRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test Mentor Extreme".to_string(),
			phone_number: "1234567890".to_string(),
			identity_and_verification: IdentityAndVerification {
				legal_name: "A".repeat(100), // Very long name
				gender: Some("Laki-laki".to_string()),
				domicile: Some("Jakarta".to_string()),
				identity_document_url: "http://example.com/id.pdf".to_string(),
				phone_for_verification: "0987654321".to_string(),
			},
			professional_profile: ProfessionalProfile {
				bio: "A".repeat(5000), // Very long bio
				last_education: Some("S1".to_string()),
				linkedin_url: Some("http://linkedin.com/in/test".to_string()),
				github_url: None,
				cv_url: None,
				portfolio_url: Some("http://example.com/portfolio".to_string()),
				industries: vec!["Technology".to_string()],
				expertise: vec!["Rust".to_string()],
				languages: vec!["English".to_string()],
				current_company: "Tech Corp".to_string(),
				current_role: "Senior Engineer".to_string(),
				years_of_experience: 50, // High experience
			},
			mentoring_logistics: MentoringLogistics {
				topics_of_interest: vec!["Career Development".to_string()],
				preferred_mentee_level: vec!["Beginner".to_string()],
				preferred_mentoring_formats: vec!["Online".to_string()],
				availability_commitment: "A".repeat(500), // Very long commitment
				mentoring_rate_amount: 1000000, // High rate
			},
		};

		let response = MentorsService::register_mentor(&app_state, mentor_dto).await;
		assert_eq!(response.status(), StatusCode::OK);

		// Clean up
		let user = user_repo.query_user_by_email(email.clone()).await.unwrap();
		let _ = user_repo.query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_update_mentor_partial_success() {
		let app_state = setup_all_test_environment().await;
		let mentor_repo = MentorsRepository::new(&app_state);
		let user_repo = UsersRepository::new(&app_state);

		// Create test mentor first
		let email = generate_unique_email("test_partial_update");
		let password = "Password123!".to_string();

		let mentor_dto = MentorUserRegisterRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test Mentor Partial".to_string(),
			phone_number: "1234567890".to_string(),
			identity_and_verification: IdentityAndVerification {
				legal_name: "Legal Test Name".to_string(),
				gender: Some("Laki-laki".to_string()),
				domicile: Some("Jakarta".to_string()),
				identity_document_url: "http://example.com/id.pdf".to_string(),
				phone_for_verification: "0987654321".to_string(),
			},
			professional_profile: ProfessionalProfile {
				bio: "Experienced professional with 5+ years of experience in software development.".to_string(),
				last_education: Some("S1".to_string()),
				linkedin_url: Some("http://linkedin.com/in/test".to_string()),
				github_url: None,
				cv_url: None,
				portfolio_url: Some("http://example.com/portfolio".to_string()),
				industries: vec!["Technology".to_string()],
				expertise: vec!["Rust".to_string()],
				languages: vec!["English".to_string()],
				current_company: "Tech Corp".to_string(),
				current_role: "Senior Engineer".to_string(),
				years_of_experience: 5,
			},
			mentoring_logistics: MentoringLogistics {
				topics_of_interest: vec!["Career Development".to_string()],
				preferred_mentee_level: vec!["Beginner".to_string()],
				preferred_mentoring_formats: vec!["Online".to_string()],
				availability_commitment: "5 hours/week".to_string(),
				mentoring_rate_amount: 100,
			},
		};

		let _ = MentorsService::register_mentor(&app_state, mentor_dto).await;

		let mentor = mentor_repo.query_mentor_by_email(email.clone(), false).await.unwrap();
		let mentor_id = mentor.id.id.to_raw();

		// Update with only some fields
		let partial_update_dto = MentorUpdateRequestDto {
			legal_name: Some("Updated Name".to_string()),
			current_role: Some("Lead Engineer".to_string()),
			..Default::default() // Other fields None
		};

		let response = MentorsService::update_mentor(&app_state, mentor_id, partial_update_dto).await;
		assert_eq!(response.status(), StatusCode::OK);

		// Verify only specified fields were updated
		let updated_mentor = mentor_repo.query_mentor_by_id(&mentor.id, false).await.unwrap();
		assert_eq!(updated_mentor.legal_name, Some("Updated Name".to_string()));
		assert_eq!(updated_mentor.current_role, "Lead Engineer".to_string());
		// Other fields should remain unchanged
		assert_eq!(updated_mentor.bio, "Experienced professional with 5+ years of experience in software development.");

		// Clean up
		let user = user_repo.query_user_by_email(email.clone()).await.unwrap();
		let _ = user_repo.query_delete_user(user.id.id.to_raw()).await;
	}

	#[tokio::test]
	async fn test_verify_mentor_invalid_status() {
		let app_state = setup_all_test_environment().await;
		let mentor_repo = MentorsRepository::new(&app_state);
		let user_repo = UsersRepository::new(&app_state);

		// Create test mentor first
		let email = generate_unique_email("test_verify_invalid_status");
		let password = "Password123!".to_string();

		let mentor_dto = MentorUserRegisterRequestDto {
			email: email.clone(),
			password: password.clone(),
			fullname: "Test Mentor Verify Invalid".to_string(),
			phone_number: "1234567890".to_string(),
			identity_and_verification: IdentityAndVerification {
				legal_name: "Legal Test Name".to_string(),
				gender: Some("Laki-laki".to_string()),
				domicile: Some("Jakarta".to_string()),
				identity_document_url: "http://example.com/id.pdf".to_string(),
				phone_for_verification: "0987654321".to_string(),
			},
			professional_profile: ProfessionalProfile {
				bio: "Experienced professional with 5+ years of experience in software development.".to_string(),
				last_education: Some("S1".to_string()),
				linkedin_url: Some("http://linkedin.com/in/test".to_string()),
				github_url: None,
				cv_url: None,
				portfolio_url: Some("http://example.com/portfolio".to_string()),
				industries: vec!["Technology".to_string()],
				expertise: vec!["Rust".to_string()],
				languages: vec!["English".to_string()],
				current_company: "Tech Corp".to_string(),
				current_role: "Senior Engineer".to_string(),
				years_of_experience: 5,
			},
			mentoring_logistics: MentoringLogistics {
				topics_of_interest: vec!["Career Development".to_string()],
				preferred_mentee_level: vec!["Beginner".to_string()],
				preferred_mentoring_formats: vec!["Online".to_string()],
				availability_commitment: "5 hours/week".to_string(),
				mentoring_rate_amount: 100,
			},
		};

		let _ = MentorsService::register_mentor(&app_state, mentor_dto).await;

		let mentor = mentor_repo.query_mentor_by_email(email.clone(), false).await.unwrap();
		let mentor_id = mentor.id.id.to_raw();

		// Try to verify with invalid status
		let invalid_verify_dto = MentorVerifyRequestDto {
			status: "invalid_status".to_string(),
		};

		let response = MentorsService::verify_mentor(&app_state, mentor_id, invalid_verify_dto).await;
		assert_eq!(response.status(), StatusCode::BAD_REQUEST);

		// Clean up
		let user = user_repo.query_user_by_email(email.clone()).await.unwrap();
		let _ = user_repo.query_delete_user(user.id.id.to_raw()).await;
	}
}
}