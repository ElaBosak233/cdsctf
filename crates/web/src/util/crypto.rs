//! Web utility — `crypto` (shared HTTP helpers).

use argon2::{PasswordHasher, PasswordVerifier};
use ring::digest::{SHA256, digest};
use subtle::ConstantTimeEq;

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

/// Hashes a high-entropy PAT secret for storage.
pub fn hash_access_token(secret: &str) -> String {
    hex::encode(digest(&SHA256, secret.as_bytes()).as_ref())
}

/// Compares a presented PAT secret with its stored hash in constant time.
pub fn verify_access_token(secret: &str, expected_hash: &str) -> bool {
    let presented = hash_access_token(secret);
    presented.as_bytes().ct_eq(expected_hash.as_bytes()).into()
}

#[cfg(test)]
mod tests {
    use super::{hash_access_token, verify_access_token};

    #[test]
    fn access_token_hash_is_stable_and_verifiable() {
        let hash = hash_access_token("secret");
        assert_eq!(hash.len(), 64);
        assert!(verify_access_token("secret", &hash));
        assert!(!verify_access_token("different", &hash));
    }
}
