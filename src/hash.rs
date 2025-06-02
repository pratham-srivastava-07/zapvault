use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand_core::OsRng;
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;

// use aes_gcm::Aes256Gcm; 
// use aes_gcm::aead::{Aead, KeyInit};
// use aes_gcm::{Key, Nonce};


pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}


pub fn verify_password(hash: &str, password: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash).unwrap();
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}


pub fn derive_key_from_password(password: &str, salt: &[u8]) -> [u8; 32] {
    let mut key = [0u8; 32]; // 256-bit key
    pbkdf2_hmac::<Sha256>(password.as_bytes(), salt, 100_000, &mut key);
    key
}


// pub fn decrypt_entry(ciphertext: &[u8], key_bytes: &[u8], nonce_bytes: &[u8; 12]) -> Vec<u8> {
//     let key = Key::<Aes256Gcm>::from_slice(key_bytes); 
//     let cipher = Aes256Gcm::new(key);
//     let nonce = Nonce::from_slice(nonce_bytes);

//     cipher
//         .decrypt(nonce, ciphertext)
//         .expect("decryption failure!")
// }