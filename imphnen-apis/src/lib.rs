use ::surrealdb::Uuid;
use imphnen_entities::*;
use imphnen_libs::*;
use imphnen_utils::*;

pub mod apps;

pub use apps::*;
pub use imphnen_entities::*;
pub use imphnen_libs::*;

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
		password: hash_password("secret").unwrap(),
		is_deleted: false,
		avatar: None,
		phone_number: "081234567890".to_string(),
		is_active,
		gender: None,
		birthdate: None,
		role: make_thing("app_roles", role_id),
		..Default::default()
	}
}
