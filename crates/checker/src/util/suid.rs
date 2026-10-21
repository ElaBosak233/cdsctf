//! Sixteen-byte randomized identity encoding using one complete AES block.
//!
//! Layout before encryption: signed big-endian id (8), randomizer (4), zeros
//! (4). The fixed redundancy is checked only after full-block decryption. Its
//! 32-bit budget is not a general-purpose AEAD guarantee. Existing HMAC-based
//! SUIDs belong to a different key domain and must be reissued when changing
//! formats.

use aes::cipher::{BlockCipherDecrypt, BlockCipherEncrypt, KeyInit};
use anyhow::anyhow;
use ring::{
    hkdf::{HKDF_SHA256, Salt},
    rand::{SecureRandom, SystemRandom},
};
use uuid::Uuid;

fn cipher(key: &str) -> aes::Aes128 {
    let salt = Salt::new(HKDF_SHA256, b"uuid");
    let prk = salt.extract(key.as_bytes());
    let okm = prk
        .expand(&[b"suid-ete-v1"], HKDF_SHA256)
        .expect("SUID HKDF expand failed");
    let mut derived = [0u8; 32];
    okm.fill(&mut derived).expect("SUID HKDF fill failed");
    aes::Aes128::new_from_slice(&derived[..16]).expect("AES-128 key length")
}

/// Encode the complete signed identifier in lowercase simple or hyphenated
/// form.
pub fn encode(data: i64, key: &str, hyphenated: bool) -> String {
    let mut block = aes::Block::default();
    block[..8].copy_from_slice(&data.to_be_bytes());
    SystemRandom::new()
        .fill(&mut block[8..12])
        .expect("SUID randomizer generation failed");
    cipher(key).encrypt_block(&mut block);
    let payload = Uuid::from_bytes(block.into());
    if hyphenated {
        payload.hyphenated().to_string()
    } else {
        payload.simple().to_string()
    }
}

/// Recover the identifier after checking the encrypted 32-bit redundancy.
pub fn decode(payload: &str, key: &str) -> Result<i64, anyhow::Error> {
    let mut block: aes::Block = Uuid::parse_str(payload)?.into_bytes().into();
    cipher(key).decrypt_block(&mut block);
    if block[12..].iter().fold(0u8, |bits, byte| bits | byte) != 0 {
        return Err(anyhow!("Invalid SUID redundancy"));
    }
    Ok(i64::from_be_bytes(block[..8].try_into().unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;

    const KEY: &str = "Fr1endShip1sM4g1c";

    #[test]
    fn round_trips_signed_ids_and_preserves_output_style() {
        for id in [i64::MIN, -i64::MAX, -42, -1, 0, 1, 42, i64::MAX] {
            for hyphenated in [false, true] {
                let payload = encode(id, KEY, hyphenated);
                assert_eq!(payload.len(), if hyphenated { 36 } else { 32 });
                for (i, byte) in payload.bytes().enumerate() {
                    if hyphenated && [8, 13, 18, 23].contains(&i) {
                        assert_eq!(byte, b'-');
                    } else {
                        assert!(byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte));
                    }
                }
                assert_eq!(decode(&payload, KEY).unwrap(), id);
                assert_eq!(decode(&payload.to_uppercase(), KEY).unwrap(), id);
                let uuid = Uuid::parse_str(&payload).unwrap();
                assert_eq!(decode(&uuid.braced().to_string(), KEY).unwrap(), id);
                assert_eq!(decode(&uuid.urn().to_string(), KEY).unwrap(), id);
            }
        }
    }

    #[test]
    fn all_redundancy_bits_are_required() {
        let cipher = cipher(KEY);
        for bit in 0..32 {
            let mut block = aes::Block::default();
            block[..8].copy_from_slice(&(-42i64).to_be_bytes());
            block[8..12].copy_from_slice(&[1, 2, 3, 4]);
            block[12 + bit / 8] = 1 << (bit % 8);
            cipher.encrypt_block(&mut block);
            assert!(decode(&Uuid::from_bytes(block.into()).to_string(), KEY).is_err());
        }
    }

    #[test]
    fn preserves_the_randomizer_in_the_encrypted_layout() {
        let cipher = cipher(KEY);
        let mut randomizers = std::collections::HashSet::new();
        for _ in 0..128 {
            let payload = encode(-42, KEY, false);
            let mut block: aes::Block = Uuid::parse_str(&payload).unwrap().into_bytes().into();
            cipher.decrypt_block(&mut block);
            assert_eq!(&block[..8], &(-42i64).to_be_bytes());
            assert_eq!(&block[12..], &[0; 4]);
            randomizers.insert(block[8..12].to_vec());
        }
        // Require variation, not impossible-to-guarantee collision freedom.
        assert!(randomizers.len() > 1);
    }

    #[test]
    fn rejects_fixed_wrong_key_and_malformed_fixtures() {
        for payload in ["", "broken", "flag{00000000000000000000000000000000}"] {
            assert!(decode(payload, KEY).is_err());
        }
        let mut block = aes::Block::default();
        block[..8].copy_from_slice(&42i64.to_be_bytes());
        block[8..12].copy_from_slice(&[1, 2, 3, 4]);
        cipher(KEY).encrypt_block(&mut block);
        let payload = Uuid::from_bytes(block.into()).to_string();
        assert_eq!(decode(&payload, KEY).unwrap(), 42);
        assert!(decode(&payload, "different-test-key").is_err());
        let mut bytes = *Uuid::parse_str(&payload).unwrap().as_bytes();
        for bit in 0..128 {
            bytes[bit / 8] ^= 1 << (bit % 8);
            assert!(decode(&Uuid::from_bytes(bytes).to_string(), KEY).is_err());
            bytes[bit / 8] ^= 1 << (bit % 8);
        }
    }
}
