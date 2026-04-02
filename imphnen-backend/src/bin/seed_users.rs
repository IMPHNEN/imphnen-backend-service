#![allow(clippy::all)]

use imphnen_entities::seaorm::auth::users::ActiveModel as UsersActiveModel;
use imphnen_entities::seaorm::auth::users::Entity as UserEntity;
use imphnen_libs::hash_password;
use imphnen_libs::postgres::{PostgresConfig, PostgresConnection};

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait, IntoActiveModel};
use std::error::Error;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let postgres_config = PostgresConfig::from_env()?;
	let pg_conn = PostgresConnection::new(postgres_config).await?;
	let db = &pg_conn.conn;

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
			"665a3cfc-ea5f-4bcd-8769-4a6d8d1451d4",
			"testuser1@example.com",
			"Test User 1",
			"5713cb37-dc02-4e87-8048-d7a41d352059",
		),
		(
			"3972c139-a450-416c-93b0-c42539dc780f",
			"testuser2@example.com",
			"Test User 2",
			"5713cb37-dc02-4e87-8048-d7a41d352059",
		),
		(
			"b426c0a9-0efb-4e26-b078-4f18767255f3",
			"testuser3@example.com",
			"Test User 3",
			"5713cb37-dc02-4e87-8048-d7a41d352059",
		),
		(
			"11111111-1111-1111-1111-111111111111",
			"user4@example.com",
			"User Four",
			"5713cb37-dc02-4e87-8048-d7a41d352059",
		),
		(
			"22222222-2222-2222-2222-222222222222",
			"user5@example.com",
			"User Five",
			"5713cb37-dc02-4e87-8048-d7a41d352059",
		),
		(
			"33333333-3333-3333-3333-333333333333",
			"mentor2@example.com",
			"Mentor Two",
			"3b9f8c4e-6a2d-4f8a-9a12-2d6f8b3c4e5a",
		),
		(
			"44444444-4444-4444-4444-444444444444",
			"staff2@example.com",
			"Staff Two",
			"50133429-f4b1-4249-9f97-7b86e6ee9d86",
		),
		(
			"55555555-5555-5555-5555-555555555555",
			"user6@example.com",
			"User Six",
			"5713cb37-dc02-4e87-8048-d7a41d352059",
		),
		(
			"66666666-6666-6666-6666-666666666666",
			"user7@example.com",
			"User Seven",
			"5713cb37-dc02-4e87-8048-d7a41d352059",
		),
		(
			"77777777-7777-7777-7777-777777777777",
			"user8@example.com",
			"User Eight",
			"5713cb37-dc02-4e87-8048-d7a41d352059",
		),
		(
			"88888888-8888-8888-8888-888888888888",
			"user9@example.com",
			"User Nine",
			"5713cb37-dc02-4e87-8048-d7a41d352059",
		),
		(
			"99999999-9999-9999-9999-999999999999",
			"user10@example.com",
			"User Ten",
			"5713cb37-dc02-4e87-8048-d7a41d352059",
		),
	];

	for (id, email, fullname, role_id_str) in users {
		let role_uuid = Some(
			Uuid::parse_str(role_id_str)
				.map_err(|e| format!("Invalid UUID for role: {role_id_str} - {e}"))?,
		);

		let uid = Uuid::parse_str(id)?;

		let names: Vec<&str> = fullname.split_whitespace().collect();
		let first_name = names.first().map(|s| s.to_string());
		let last_name = if names.len() > 1 {
			Some(names[1..].join(" "))
		} else {
			None
		};

		let password = "password";
		let hashed = hash_password(password).unwrap();

		let existing_user = UserEntity::find_by_id(uid).one(db).await?;
		let is_update = existing_user.is_some();

		let mut user_model: UsersActiveModel = if let Some(existing) = existing_user {
			println!("🔄 Updating user: {fullname} ({email})");
			existing.into_active_model()
		} else {
			println!("✅ Inserting user: {fullname} ({email})");
			let mut active: UsersActiveModel = Default::default();
			active.id = Set(uid);
			active.created_at = Set(Utc::now());
			active
		};

		user_model.email = Set(email.to_string());
		user_model.password_hash = Set(hashed);
		user_model.username = Set(email.to_string());
		user_model.first_name = Set(first_name);
		user_model.last_name = Set(last_name);
		user_model.avatar_url = Set(Some("https://example.com/avatar.jpg".to_string()));
		user_model.is_verified = Set(true);
		user_model.is_active = Set(true);
		user_model.role_id = Set(role_uuid);
		user_model.updated_at = Set(Utc::now());

		if is_update {
			user_model.update(db).await?;
		} else {
			user_model.insert(db).await?;
		}
	}
	println!("✅ All Users seeded");
	Ok(())
}
