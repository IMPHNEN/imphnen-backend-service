use ::surrealdb::Uuid;
use ::surrealdb::sql;
pub use imphnen_entities::MetaRequestDto;
pub use imphnen_iam::{ResourceEnum, RolesRepository, UsersRepository, AuthOtpSchema, AuthRepository, RolesDetailQueryDto, UsersDetailQueryDto, RolesRequestCreateDto, RolesRequestUpdateDto, RolesDetailItemDto, TeamsRepository, TeamsSchema, TeamMembersSchema, TeamInvitationsSchema, UsersSchema};
use imphnen_libs::AppState;

pub fn create_test_mentor(
	email: &str,
	fullname: &str,
	is_active: bool,
	role_id: &sql::Thing,
) -> UsersSchema {
	let mut user = create_test_user(email, fullname, is_active, role_id);
	user.mentor_id = Some(user.id.clone());
	user
}

pub fn create_test_user(
	email: &str,
	fullname: &str,
	is_active: bool,
	role_id: &sql::Thing,
) -> UsersSchema {
	UsersSchema {
		id: make_thing("app_users", &Uuid::new_v4().to_string()),
		email: email.to_string(),
		fullname: format!("{} {}", fullname, rand::random::<u32>()),
		legal_name: None,
		password: hash_password("password123").unwrap(),
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
		website_url: None,
		twitter_url: None,
		location: None,
		skills: None,
		experience: None,
		education: None,
		career_status: None,
		role: role_id.clone(),
		created_at: get_iso_date(),
		updated_at: get_iso_date(),
		mentor_id: None,
	}
}

#[cfg(test)]
pub mod iam;
pub mod hackathon;
pub mod mock_test;

pub use mock_test::{
	cleanup_db, create_mock_app_state, seed_permissions_and_roles_for_test,
	seed_users_for_test, setup_all_test_environment,
};

pub use imphnen_utils::{get_iso_date, hash_password, make_thing, Env};

pub fn generate_unique_email(prefix: &str) -> String {
	format!("{}_{}@example.com", prefix, Uuid::new_v4())
}

pub async fn get_role_id(role_name: &str, state: &AppState) -> sql::Thing {
	let repo = RolesRepository::new(state);
	if let Ok(existing) = repo.query_role_by_name(role_name.into()).await {
		return make_thing(&ResourceEnum::Roles.to_string(), &existing.id);
	}
	let _ = repo
		.query_create_role(RolesRequestCreateDto {
			name: role_name.into(),
			permissions: vec![],
		})
		.await;
	let role = repo
		.query_role_by_name(role_name.into())
		.await
		.expect("Role not found after creation");
	make_thing(&ResourceEnum::Roles.to_string(), &role.id)
}

pub async fn get_app_state() -> AppState {
	create_mock_app_state().await
}

pub async fn setup() {
	cleanup_db().await;
	let app_state = create_mock_app_state().await;
	seed_permissions_and_roles_for_test(&app_state.surrealdb_ws)
		.await
		.unwrap();
	seed_users_for_test(&app_state.surrealdb_ws).await.unwrap();
}

pub fn get_meta_request_dto(page: u64, per_page: u64) -> imphnen_entities::MetaRequestDto {
    imphnen_entities::MetaRequestDto {
        page: Some(page),
        per_page: Some(per_page),
        search: None,
        sort_by: None,
        order: None,
        filter: None,
        filter_by: None,
    }
}
