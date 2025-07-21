use crate::{get_iso_date, hash_password};
use imphnen_entities::AppState;
use imphnen_iam::{PermissionsEnum, UsersSchema};
use imphnen_libs::enviroment::load_env;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use surrealdb::engine::{any, local};
use surrealdb::{opt::auth::Root, sql::Thing, Connection, Surreal};
use tracing::debug;
use uuid::Uuid; 

#[derive(Serialize, Deserialize, Debug)]
struct PermissionSeedData {
	id: String,
	name: String,
	is_deleted: bool,
	created_at: String,
	updated_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RoleSeedData {
	id: String,
	name: String,
	is_deleted: bool,
	created_at: String,
	updated_at: String,
	permissions: Vec<Thing>,
}

pub async fn create_mock_app_state() -> AppState {
	load_env();
	 

	 
	let db_ws = any::connect("ws://127.00.1:8000/rpc").await.unwrap();
	 

	 
	let db_mem = Surreal::new::<local::Mem>(()).await.unwrap();
	 

	let unique_id = Uuid::new_v4().to_string();
	let ns = format!("test_ns_{unique_id}");
	let db = format!("test_db_{unique_id}");
	 

	 
	db_ws
		.signin(Root {
			username: "root",
			password: "root",
		})
		.await
		.unwrap();
	 

	 
	db_ws.use_ns(&ns).use_db(&db).await.unwrap();
	 

	AppState {
		surrealdb_ws: db_ws,
		surrealdb_mem: db_mem,
	}
}
pub async fn cleanup_db() {
	let app_state = create_mock_app_state().await;
	let _ = app_state
		.surrealdb_ws
		.query(
			r#"REMOVE TABLE app_users; REMOVE TABLE app_roles; REMOVE TABLE app_permissions; REMOVE TABLE app_roles_permissions; REMOVE TABLE app_users_cache; REMOVE TABLE app_otp_cache; REMOVE TABLE app_gacha_items; REMOVE TABLE app_gacha_claims; REMOVE TABLE app_gacha_rolls; REMOVE TABLE app_gacha_credits; REMOVE TABLE app_events; REMOVE TABLE app_testimonials; REMOVE TABLE app_mentors;"#,
		)
		.await;
}

pub async fn seed_permissions_and_roles_for_test(
	db: &Surreal<impl Connection>,
) -> Result<(), Box<dyn std::error::Error>> {
	db.query("DEFINE TABLE app_permissions;").await?;
	db.query("DEFINE FIELD name ON app_permissions TYPE string;")
		.await?;
	db.query("DEFINE FIELD is_deleted ON app_permissions TYPE bool;")
		.await?;
	db.query("DEFINE FIELD created_at ON app_permissions TYPE string;")
		.await?;
	db.query("DEFINE FIELD updated_at ON app_permissions TYPE string;")
		.await?;

	db.query("DEFINE TABLE app_roles;").await?;
	db.query("DEFINE FIELD name ON app_roles TYPE string;")
		.await?;
	db.query("DEFINE FIELD is_deleted ON app_roles TYPE bool;")
		.await?;
	// Changed from array<string> to array<record<app_permissions>>
	db.query(
		"DEFINE FIELD permissions ON app_roles TYPE array<record<app_permissions>>;",
	)
	.await?;
	db.query("DEFINE FIELD created_at ON app_roles TYPE string;")
		.await?;
	db.query("DEFINE FIELD updated_at ON app_roles TYPE string;")
		.await?;

	for perm_enum in PermissionsEnum::iter() {
		db.query("INSERT INTO app_permissions (id, name, is_deleted, created_at, updated_at) VALUES ($id, $name, $is_deleted, $created_at, $updated_at);")
            .bind(("id", perm_enum.id()))
            .bind(("name", perm_enum.to_string()))
            .bind(("is_deleted", false))
            .bind(("created_at", get_iso_date()))
            .bind(("updated_at", get_iso_date()))
            .await?;
	}

	let roles = vec![
		("f6b03f25-e416-4893-ac88-caaa690afb07".to_string(), "Admin"),
		("3b9f8c4e-6a2d-4f8a-9a12-2d6f8b3c4e5a".to_string(), "Mentor"),
		("50133429-f4b1-4249-9f97-7b86e6ee9d86".to_string(), "Staff"),
		("5713cb37-dc02-4e87-8048-d7a41d352059".to_string(), "User"),
	];

	for (id, name) in roles {
		db.query("INSERT INTO app_roles (id, name, is_deleted, created_at, updated_at, permissions) VALUES ($id, $name, $is_deleted, $created_at, $updated_at, $permissions);")
            .bind(("id", id))
            .bind(("name", name))
            .bind(("is_deleted", false))
            .bind(("created_at", get_iso_date()))
            .bind(("updated_at", get_iso_date()))
            .bind(("permissions", Vec::<Thing>::new())) // Use Vec<Thing> for permissions
            .await?;
	}
	Ok(())
}

pub async fn seed_users_for_test(
	db: &Surreal<impl Connection>,
) -> Result<(), Box<dyn std::error::Error>> {
	let users_data = vec![
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

	for (id, email, fullname, role_id) in users_data {
		let user = UsersSchema {
			id: Thing::from(("app_users", id)),
			fullname: fullname.into(),
			email: email.into(),
			password: hash_password("password").unwrap(),
			avatar: None,
			phone_number: "081234567890".into(),
			is_active: true,
			is_deleted: false,
			mentor_id: None,
			gender: None,
			birthdate: None,
			role: Thing::from(("app_roles", role_id)),
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
		};
		db.create::<Option<UsersSchema>>(("app_users", id))
			.content(user)
			.await?;

		println!("✅ Inserted user: {fullname} ({email})");
	}
	Ok(())
}

pub async fn setup_all_test_environment() -> AppState {
	let app_state = create_mock_app_state().await;
	app_state
		.surrealdb_mem
		.use_ns("test_namespace")
		.use_db("test_db")
		.await
		.unwrap();
	seed_permissions_and_roles_for_test(&app_state.surrealdb_ws)
		.await
		.unwrap();
	app_state
}
