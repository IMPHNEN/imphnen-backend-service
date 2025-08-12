use imphnen_iam::UsersSchema;
use imphnen_utils::{get_iso_date, hash_password};
use std::error::Error;

use surrealdb::{opt::auth::Root, sql::Thing};
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let env = &imphnen_libs::enviroment::ENV;
	use surrealdb::engine::any;
	let db = any::connect(&env.surrealdb_url).await?;
	db.signin(Root {
		username: &env.surrealdb_username,
		password: &env.surrealdb_password,
	})
	.await?;
	db.use_ns(env.surrealdb_namespace.clone())
		.use_db(env.surrealdb_dbname.clone())
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
			"user@example.com",
			"User",
			"5713cb37-dc02-4e87-8048-d7a41d352059",
		),
	];

	for (id, email, fullname, role_id) in users {
		db.query("DELETE type::thing('app_users', $id)")
			.bind(("id", id))
			.await?;

		let user = UsersSchema {
			id: Thing::from(("app_users", id)),
			fullname: fullname.into(),
			legal_name: None,
			email: email.into(),
			password: hash_password("password").unwrap(),
			avatar: None,
			phone_number: "081234567890".into(),
			phone_for_verification: None,
			is_active: true,
			is_deleted: false,
			mentor_id: None,
			gender: None,
			birthdate: None,
			domicile: None,
			identity_document_url: None,
			bio: None,
			last_education: None,
			linkedin_url: None,
			github_url: None,
			cv_url: None,
			portfolio_url: None,
			role: Thing::from(("app_roles", role_id)),
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
		};

		db.create::<Option<UsersSchema>>(("app_users", id))
			.content(user)
			.await?;

		println!("✅ Inserted user: {fullname} ({email})");
	}

	println!("✅ All Users seeded");
	Ok(())
}
