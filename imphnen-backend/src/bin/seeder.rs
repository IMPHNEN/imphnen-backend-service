use std::error::Error;
use std::process::Command;

fn run_seed(bin: &str) -> Result<(), Box<dyn Error>> {
	println!("🔧 Seeding: {bin}");
	let status = Command::new("cargo").args(["run", "--bin", bin]).status()?;

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

	println!("\n✅ All seeding completed successfully.");
	Ok(())
}
