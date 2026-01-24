//! Soul Vault crypto primitives.
//!
//! Storage format for encrypted blobs (".sola"):
//! - magic: 4 bytes: b"SOLA"
//! - version: 1 byte: 0x01
//! - salt_len: 1 byte (0 if no salt included)
//! - salt: N bytes (optional)
//! - nonce: 12 bytes (AES-GCM nonce)
//! - ciphertext+tag: remaining bytes

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use argon2::{Argon2, Params};
use base64::{engine::general_purpose, Engine as _};
use rand_core::RngCore;
use zeroize::Zeroizing;

pub const SOLA_BLOB_MAGIC: &[u8; 4] = b"SOLA";
pub const SOLA_BLOB_VERSION: u8 = 1;
pub const AES256_KEY_LEN: usize = 32;
pub const AES_GCM_NONCE_LEN: usize = 12;

#[derive(Debug, thiserror::Error)]
pub enum VaultError {
    #[error("invalid blob format")]
    InvalidFormat,
    #[error("crypto error")]
    Crypto,
    #[error("argon2 error")]
    Argon2,
}

/// Derive a session vault key from user-supplied secret (e.g., login passphrase/PIN).
///
/// Note: caller should persist the `salt` (or store alongside encrypted blobs).
pub fn derive_vault_key_argon2id(secret: &[u8], salt: &[u8]) -> Result<Zeroizing<[u8; AES256_KEY_LEN]>, VaultError> {
    let params = Params::new(64 * 1024, 3, 1, Some(AES256_KEY_LEN)).map_err(|_| VaultError::Argon2)?;
    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);

    let mut out = Zeroizing::new([0u8; AES256_KEY_LEN]);
    argon2
        .hash_password_into(secret, salt, out.as_mut_slice())
        .map_err(|_| VaultError::Argon2)?;
    Ok(out)
}

/// Encrypts persona data using AES-256-GCM.
///
/// `vault_key` must be 32 bytes.
pub fn encrypt_persona_data(vault_key: &[u8; AES256_KEY_LEN], plaintext: &[u8], salt: Option<&[u8]>) -> Result<Vec<u8>, VaultError> {
    let cipher = Aes256Gcm::new_from_slice(vault_key).map_err(|_| VaultError::Crypto)?;

    let mut nonce_bytes = [0u8; AES_GCM_NONCE_LEN];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, plaintext).map_err(|_| VaultError::Crypto)?;

    let salt_bytes = salt.unwrap_or(&[]);
    let salt_len: u8 = salt_bytes
        .len()
        .try_into()
        .map_err(|_| VaultError::InvalidFormat)?;

    let mut out = Vec::with_capacity(4 + 1 + 1 + salt_bytes.len() + AES_GCM_NONCE_LEN + ciphertext.len());
    out.extend_from_slice(SOLA_BLOB_MAGIC);
    out.push(SOLA_BLOB_VERSION);
    out.push(salt_len);
    out.extend_from_slice(salt_bytes);
    out.extend_from_slice(&nonce_bytes);
    out.extend_from_slice(&ciphertext);
    Ok(out)
}

/// Decrypts persona data using AES-256-GCM.
pub fn decrypt_persona_data(vault_key: &[u8; AES256_KEY_LEN], blob: &[u8]) -> Result<Vec<u8>, VaultError> {
    if blob.len() < 4 + 1 + 1 + AES_GCM_NONCE_LEN + 16 {
        return Err(VaultError::InvalidFormat);
    }
    if &blob[0..4] != SOLA_BLOB_MAGIC {
        return Err(VaultError::InvalidFormat);
    }
    if blob[4] != SOLA_BLOB_VERSION {
        return Err(VaultError::InvalidFormat);
    }

    let salt_len = blob[5] as usize;
    let header_len = 4 + 1 + 1;
    let nonce_off = header_len + salt_len;
    let nonce_end = nonce_off + AES_GCM_NONCE_LEN;
    if blob.len() <= nonce_end {
        return Err(VaultError::InvalidFormat);
    }

    let nonce = Nonce::from_slice(&blob[nonce_off..nonce_end]);
    let ciphertext = &blob[nonce_end..];

    let cipher = Aes256Gcm::new_from_slice(vault_key).map_err(|_| VaultError::Crypto)?;
    cipher.decrypt(nonce, ciphertext).map_err(|_| VaultError::Crypto)
}

/// Convenience: base64 encode bytes (for frontend transport).
pub fn to_base64(data: &[u8]) -> String {
    general_purpose::STANDARD.encode(data)
}

/// Convenience: generate random salt.
pub fn generate_salt(len: usize) -> Vec<u8> {
    let mut salt = vec![0u8; len];
    OsRng.fill_bytes(&mut salt);
    salt
}

