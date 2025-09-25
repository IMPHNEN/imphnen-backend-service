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
}