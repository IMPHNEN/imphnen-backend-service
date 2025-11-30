#![allow(clippy::all)]

use imphnen_libs::postgres::{PostgresConfig, PostgresConnection};
use sea_orm::{Statement, ConnectionTrait};
use std::error::Error;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    // New default behavior: execute by default; use --dry-run to preview only.
    let dry_run = args.iter().any(|s| s == "--dry-run" || s == "--no-exec" || s == "--dry");
    let force = args.iter().any(|s| s == "--force" || s == "-f");

    println!("🔎 Clear DB script - WARNING: This will remove data from tables\n");
    println!("Note: script now runs by default (no --yes required). To preview without executing, use --dry-run.\n");

    // List of tables to truncate (order doesn't matter with CASCADE)
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

    // Filter tables that actually exist in the database
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

    // Prevent accidental execution in production without explicit force flag
    let env_name = std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string());
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
