use imphnen_iam::v1::teams::TeamsSchema;
use imphnen_utils::get_iso_date;
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

	let teams = vec![
		(
			"team-dev-001",
			"Development Team",
			Some("Core development team for the platform".to_string()),
			"c3b1d6a8-8d4f-4b36-b789-2e532ec7a7b2", // admin user
			true,
			Some(10),
			Some(vec!["Rust".to_string(), "Backend".to_string()]),
			Some("Remote".to_string()),
		),
		(
			"team-design-001",
			"Design Team",
			Some("UI/UX design team".to_string()),
			"c3b1d6a8-8d4f-4b36-b789-2e532ec7a7b2", // admin user
			true,
			Some(5),
			Some(vec!["Figma".to_string(), "Design".to_string()]),
			Some("Remote".to_string()),
		),
		(
			"team-qa-001",
			"Quality Assurance Team",
			Some("Testing and quality assurance team".to_string()),
			"c3b1d6a8-8d4f-4b36-b789-2e532ec7a7b2", // admin user
			false,
			Some(8),
			Some(vec!["Testing".to_string(), "Automation".to_string()]),
			Some("Remote".to_string()),
		),
	];

	for (id, name, description, leader_id, is_open, max_members, skills_required, location) in teams {
		db.query("DELETE type::thing('app_teams', $id)")
			.bind(("id", id))
			.await?;

		let team = TeamsSchema {
			id: Thing::from(("app_teams", id)),
			name: name.into(),
			description,
			leader_id: Thing::from(("app_users", leader_id)),
			is_open,
			max_members,
			skills_required,
			location,
			avatar: None,
			website_url: None,
			github_url: None,
			is_active: true,
			is_deleted: false,
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
		};

		db.create::<Option<TeamsSchema>>(("app_teams", id))
			.content(team)
			.await?;

		println!("✅ Inserted team: {name}");
	}

	println!("✅ All Teams seeded");
	Ok(())
}