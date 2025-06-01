use crate::hash::{hash_password, derive_key_from_password};
use crate::meta::Vault;
use rand::RngCore;
use aes_gcm::{Aes256Gcm, Key, Nonce}; 
use aes_gcm::aead::{Aead, KeyInit};
use std::{fs};
use rpassword::prompt_password;

pub fn handle_init(path: std::path::PathBuf) {
    println!("Init vault at {:?}", path);

    let password = prompt_password("Enter master password: ").unwrap();
    let confirm = prompt_password("Confirm password: ").unwrap();

    if password != confirm {
        eprintln!("Passwords do not match. Vault creation aborted.");
        return;
    }

    if path.exists() {
        eprintln!("Vault already exists at {:?}.", path);
        return;
    }

    fs::create_dir_all(&path).expect("Failed to create vault directory");

    let password_hash = hash_password(&password);

   
    let mut nonce = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce);

    
    let key = derive_key_from_password(&password, &nonce);
    let key = Key::<Aes256Gcm>::from_slice(&key);
    let cipher = Aes256Gcm::new(key);

    
    let plaintext = b"[]";
    let encrypted = cipher.encrypt(Nonce::from_slice(&nonce), plaintext.as_ref())
        .expect("encryption failed");

    let meta = Vault {
        password_hash,
        ciphertext: encrypted,
        nonce,
    };

    let file_path = path.join("vault.meta");
    let file = fs::File::create(&file_path).expect("Failed to create vault.meta");

    serde_json::to_writer_pretty(file, &meta).expect("Failed to write vault metadata");

    println!("Vault initialized at {:?}", path);
}
