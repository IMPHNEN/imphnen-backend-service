use imphnen_backend_service::{get_iso_date, hash_password, Env};
use serde::{Deserialize, Serialize};
use std::error::Error;
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, sql::Thing, Surreal};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsersSchema {
	pub id: Thing,
	pub fullname: String,
	pub email: String,
	pub password: String,
	pub avatar: Option<String>,
	pub phone_number: String,
	pub referral_code: Option<String>,
	pub referred_by: Option<String>,
	pub identity_number: Option<String>,
	pub is_active: bool,
	pub is_deleted: bool,
	pub student_type: String,
	pub religion: Option<String>,
	pub gender: Option<String>,
	pub birthdate: Option<String>,
	pub is_profile_completed: bool,
	pub role: Thing,
	pub created_at: String,
	pub updated_at: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let env = Env::new();
	let db = Surreal::new::<Ws>(env.surrealdb_url).await?;
	db.signin(Root {
		username: &env.surrealdb_username,
		password: &env.surrealdb_password,
	})
	.await?;
	db.use_ns(env.surrealdb_namespace)
		.use_db(env.surrealdb_dbname)
		.await?;

	let users = vec![
		(
			"c3b1d6a8-8d4f-4b36-b789-2e532ec7a7b2",
			"admin@example.com",
			"Admin",
			"f6b03f25-e416-4893-ac88-caaa690afb07",
		),
		(
			"a4d23fb5-9e31-423c-9842-fbd6e75a5298",
			"staff@example.com",
			"Staff",
			"50133429-f4b1-4249-9f97-7b86e6ee9d86",
		),
		(
			"d5e89c12-72af-4b1a-abc3-ff1234567890",
			"student@example.com",
			"Student",
			"5713cb37-dc02-4e87-8048-d7a41d352059",
		),
	];

	for (id, email, fullname, role_id) in users {
		let user = UsersSchema {
			id: Thing::from(("app_users", id)),
			fullname: fullname.into(),
			email: email.into(),
			password: hash_password("password").unwrap(),
			avatar: None,
			phone_number: "081234567890".into(),
			referral_code: None,
			referred_by: None,
			identity_number: None,
			is_active: true,
			is_deleted: false,
			student_type: "TNI".into(),
			religion: None,
			gender: None,
			birthdate: None,
			is_profile_completed: false,
			role: Thing::from(("app_roles", role_id)),
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
		};

		db.create::<Option<UsersSchema>>(("app_users", id))
			.content(user)
			.await?;

		println!("✅ Inserted user: {} ({})", fullname, email);
	}

	println!("✅ Semua users berhasil disimpan ke SurrealDB!");
	Ok(())
}
