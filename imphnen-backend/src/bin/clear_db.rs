#![allow(clippy::all)]

use imphnen_libs::postgres::{PostgresConfig, PostgresConnection};
use sea_orm::{ConnectionTrait, Statement};
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let args: Vec<String> = env::args().collect();
	let dry_run = args
		.iter()
		.any(|s| s == "--dry-run" || s == "--no-exec" || s == "--dry");
	let force = args.iter().any(|s| s == "--force" || s == "-f");

	println!("🔎 Clear DB script - WARNING: This will remove data from tables\n");
	println!("Note: script now runs by default (no --yes required). To preview without executing, use --dry-run.\n");

	let tables = vec![
		"gacha_claims",
		"gacha_rolls",
		"gacha_items",
		"gacha_credits",
		"audit_logs",
		"rate_limits",
		"testimonials",
		"events",
		"app_mentors",
		"app_sessions",
		"app_roles_permissions",
		"app_permissions",
		"app_roles",
		"app_users",
	];

	let postgres_config = PostgresConfig::from_env()?;
	let pg_conn = PostgresConnection::new(postgres_config).await?;
	let db = &pg_conn.conn;

	let mut existing_tables: Vec<&str> = vec![];
	for t in tables.iter() {
		let check_sql = format!(
            "SELECT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_schema = 'public' AND table_name = '{}') as exists;",
            t
        );
		let stmt = Statement::from_string(db.get_database_backend(), check_sql);
		if let Ok(Some(row)) = pg_conn.query_one(stmt).await {
			let exists_val: Option<bool> = row.try_get("", "exists").ok();
			if exists_val.unwrap_or(false) {
				existing_tables.push(t);
			}
		}
	}

	if existing_tables.is_empty() {
		println!("No configured tables found to clear - nothing to do.");
		return Ok(());
	}

	let truncate_sql = format!(
		"TRUNCATE TABLE {} RESTART IDENTITY CASCADE;",
		existing_tables.join(", ")
	);

	println!("The script will run the following SQL (on the DB configured by env vars):\n\n{}", truncate_sql);

	let env_name = imphnen_libs::ENV.rust_env.clone();
	if env_name == "production" && !force {
		println!("Security: RUST_ENV=production; the script will NOT run without --force. Use --force to override.");
		return Ok(());
	}

	if dry_run {
		println!("Dry run enabled. No changes applied. To execute, re-run without --dry-run or use --force (in production).");
		return Ok(());
	}

	println!("Executing truncate...\n");

	let postgres_config = PostgresConfig::from_env()?;
	let pg_conn = PostgresConnection::new(postgres_config).await?;
	let db = &pg_conn.conn;

	let stmt = Statement::from_string(db.get_database_backend(), truncate_sql);
	match pg_conn.execute(stmt).await {
		Ok(_) => println!("✅ Successfully cleared DB tables"),
		Err(e) => println!("❌ Failed to clear DB tables: {}", e),
	}

	Ok(())
}
