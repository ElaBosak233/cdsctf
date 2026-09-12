//! Web utility — `crypto` (shared HTTP helpers).

use argon2::{PasswordHasher, PasswordVerifier};

/// Hashes a password with Argon2 for storage.
pub fn hash_password(password: String) -> String {
    let argon2 = argon2::Argon2::default();
    let hash = argon2.hash_password(password.as_bytes()).unwrap();

    hash.to_string()
}

/// Verifies a plaintext password against a stored hash.
pub fn verify_password(password: String, hash: String) -> bool {
    let argon2 = argon2::Argon2::default();
    argon2
        .verify_password(
            password.as_bytes(),
            &argon2::PasswordHash::new(&hash).unwrap(),
        )
        .is_ok()
}
