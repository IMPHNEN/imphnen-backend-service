use argon2::{
	password_hash::{
		rand_core::OsRng, Error, PasswordHash, PasswordHasher, PasswordVerifier,
		SaltString,
	},
	Argon2,
};

pub fn hash_password(password: &str) -> Result<String, Error> {
	let salt = SaltString::generate(&mut OsRng);
	let argon2 = Argon2::default();
	let password_hash = argon2
		.hash_password(password.as_bytes(), &salt)?
		.to_string();
	Ok(password_hash)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, Error> {
	let parsed_hash = PasswordHash::new(hash)?;
	let argon2 = Argon2::default();
	match argon2.verify_password(password.as_bytes(), &parsed_hash) {
		Ok(_) => Ok(true),
		Err(_) => Ok(false),
	}
}
