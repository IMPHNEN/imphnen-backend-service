#![allow(clippy::all)]

use std::error::Error;
use std::process::Command;

fn run_seed(bin: &str) -> Result<(), Box<dyn Error>> {
	println!("🔧 Seeding: {bin}");
	#[cfg(target_os = "windows")]
	let status = Command::new(format!("./target/release/{}.exe", bin)).status()?;
	#[cfg(not(target_os = "windows"))]
	let status = Command::new(format!("./target/release/{}", bin)).status()?;

	if !status.success() {
		Err(format!("❌ Failed to run seed: {bin}").into())
	} else {
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	println!("🚀 Running all seeders...\n");

	run_seed("seed_permissions")?;
	run_seed("seed_roles")?;
	run_seed("seed_roles_permissions")?;
	run_seed("seed_users")?;
	run_seed("seed_events")?;
	run_seed("seed_gacha_rolls")?;
	run_seed("seed_mentor_user")?;
	run_seed("seed_test_data")?;
	println!("\n✅ All seeding completed successfully.");
	Ok(())
}
