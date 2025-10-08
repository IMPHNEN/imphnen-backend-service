use imphnen_iam::UsersSchema;
use imphnen_utils::{get_iso_date, hash_password};
use std::error::Error;

use surrealdb::{opt::auth::Root, sql::Thing};
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let env = &imphnen_libs::environment::ENV;
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
		(
			"testuser1-id",
			"testuser1@example.com",
			"Test User 1",
			"5713cb37-dc02-4e87-8048-d7a41d352059",
		),
		(
			"testuser2-id",
			"testuser2@example.com",
			"Test User 2",
			"5713cb37-dc02-4e87-8048-d7a41d352059",
		),
		(
			"testuser3-id",
			"testuser3@example.com",
			"Test User 3",
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
			legal_name: Some(format!("{} Legal Name", fullname)),
			email: email.into(),
			password: hash_password("password").unwrap(),
			avatar: Some("https://example.com/avatar.jpg".into()),
			phone_number: "081234567890".into(),
			phone_for_verification: Some("081234567890".into()),
			is_active: true,
			is_deleted: false,
			mentor_id: None,
			gender: Some("male".into()),
			birthdate: Some("1990-05-15".into()),
			domicile: Some("Jakarta, Indonesia".into()),
			// identity_document_url: None, // Sudah tidak dipakai, bisa dihapus dari schema jika tidak diperlukan
			bio: Some(format!("{} adalah user dengan data pribadi lengkap untuk testing.", fullname)),
			last_education: Some("S1 Teknik Informatika".into()),
			linkedin_url: Some("https://linkedin.com/in/user".into()),
			github_url: Some("https://github.com/user".into()),
			cv_url: Some("https://example.com/cv.pdf".into()),
			portfolio_url: Some("https://example.com/portfolio".into()),
			website_url: Some("https://example.com/website".into()),
			twitter_url: Some("https://twitter.com/user".into()),
			location: Some("Jakarta, Indonesia".into()),
			skills: Some(vec!["JavaScript".into(), "React".into(), "Node.js".into()]),
			experience: None,
			education: None,
			career_status: Some("Senior Developer".into()),
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
