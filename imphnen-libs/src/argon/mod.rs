//! Argon2 password hashing utilities.
//!
//! This module provides secure password hashing and verification using the Argon2 algorithm.
//! The hashing parameters are configured for a balance between security and performance.

use argon2::{
    password_hash::{
        rand_core::OsRng, Error, PasswordHash, PasswordHasher, PasswordVerifier,
        SaltString,
    },
    Argon2,
};

/// Hash a password using Argon2id algorithm.
///
/// This function generates a cryptographically secure salt and hashes the password
/// with predefined parameters optimized for a balance of security and performance.
///
/// # Arguments
/// * `password` - The plain text password to hash
///
/// # Returns
/// * `Ok(String)` - The hashed password in PHC string format
/// * `Err(Error)` - If hashing fails
///
/// # Example
/// ```
/// use imphnen_libs::hash_password;
///
/// let hash = hash_password("my_password")?;
/// assert!(hash.starts_with("$argon2id$"));
/// # Ok::<(), argon2::password_hash::Error>(())
/// ```
pub fn hash_password(password: &str) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    Ok(password_hash)
}

/// Verify a password against its hash.
///
/// This function checks if the provided password matches the given hash.
/// Returns false for both incorrect passwords and invalid hash formats.
///
/// # Arguments
/// * `password` - The plain text password to verify
/// * `hash` - The hashed password in PHC string format
///
/// # Returns
/// * `Ok(bool)` - true if password matches, false otherwise
/// * `Err(Error)` - If hash parsing fails
///
/// # Example
/// ```
/// use imphnen_libs::{hash_password, verify_password};
///
/// let hash = hash_password("my_password")?;
/// assert!(verify_password("my_password", &hash)?);
/// assert!(!verify_password("wrong_password", &hash)?);
/// # Ok::<(), argon2::password_hash::Error>(())
/// ```
pub fn verify_password(password: &str, hash: &str) -> Result<bool, Error> {
    let parsed_hash = PasswordHash::new(hash)?;
    let argon2 = Argon2::default();
    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}
