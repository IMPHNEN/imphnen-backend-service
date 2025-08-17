use argon2::{
	Argon2,
	password_hash::{
		Error, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
		rand_core::OsRng,
	},
};

pub fn hash_password(password: &str) -> Result<String, Error> {
	let salt = SaltString::generate(&mut OsRng);
	let argon2 = Argon2::new(
		argon2::Algorithm::Argon2id,
		argon2::Version::V0x13,
		argon2::Params::new(1024, 1, 1, None).unwrap() // 1MB, 1 iteration, 1 thread (faster, less secure)
	);
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
