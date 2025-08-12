use ::surrealdb::Uuid;
pub use imphnen_entities::*;
pub use imphnen_iam::*;
pub fn create_test_mentor(
	email: &str,
	fullname: &str,
	is_active: bool,
	role_id: &str,
) -> UsersSchema {
	let mut user = create_test_user(email, fullname, is_active, role_id);
	user.mentor_id = Some(user.id.clone());
	user
}
pub fn create_test_user(
	email: &str,
	fullname: &str,
	is_active: bool,
	role_id: &str,
) -> UsersSchema {
	UsersSchema {
		id: make_thing("app_users", &Uuid::new_v4().to_string()),
		email: email.to_string(),
		fullname: format!("Randomize {} {}", fullname, rand::random::<u32>()),
		legal_name: None,
		password: hash_password("secret").unwrap(),
		is_deleted: false,
		avatar: None,
		phone_number: "081234567890".to_string(),
		phone_for_verification: None,
		is_active,
		gender: None,
		birthdate: None,
		domicile: None,
		bio: None,
		last_education: None,
		linkedin_url: None,
		github_url: None,
		cv_url: None,
		portfolio_url: None,
		role: make_thing("app_roles", role_id),
		created_at: get_iso_date(),
		updated_at: get_iso_date(),
		mentor_id: None,
	}
}
#[cfg(test)]
pub mod iam;
pub mod mock_test;

pub use mock_test::{
	cleanup_db, create_mock_app_state, seed_permissions_and_roles_for_test,
	seed_users_for_test, setup_all_test_environment,
};

pub use imphnen_utils::{get_iso_date, hash_password, make_thing, Env};

pub fn generate_unique_email(prefix: &str) -> String {
	format!("{}_{}@example.com", prefix, Uuid::new_v4())
}

pub async fn get_role_id(state: &crate::AppState) -> String {
	let repo = RolesRepository::new(state);
	if let Ok(existing) = repo.query_role_by_name("User".into()).await {
		return existing.id;
	}
	let _ = repo
		.query_create_role(RolesRequestCreateDto {
			name: "User".into(),
			permissions: vec![],
		})
		.await;
	repo
		.query_role_by_name("User".into())
		.await
		.expect("Role not found after creation")
		.id
}

pub async fn setup() {
	cleanup_db().await;
	let app_state = create_mock_app_state().await;
	seed_permissions_and_roles_for_test(&app_state.surrealdb_ws)
		.await
		.unwrap();
	seed_users_for_test(&app_state.surrealdb_ws).await.unwrap();
}
