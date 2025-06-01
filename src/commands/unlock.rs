use crate::hash::{verify_password, derive_key_from_password};
use crate::meta::Vault;
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use std::{fs, io};
use std::io::Write;
use rpassword::prompt_password;

pub fn handle_unlock() {
    println!("Unlocking vault...");

    print!("Enter vault path: ");
    io::stdout().flush().unwrap();

    let mut path_input = String::new();
    io::stdin().read_line(&mut path_input).unwrap();
    let path_input = path_input.trim().trim_matches('"');
    let path = std::path::PathBuf::from(path_input);

    let meta_path = path.join("vault.meta");
    if !meta_path.exists() {
        eprintln!("vault.meta not found at {:?}", meta_path);
        return;
    }

    let file = fs::File::open(&meta_path).expect("Failed to open vault.meta");
    let vault: Vault = serde_json::from_reader(file).expect("Failed to parse vault.meta");

    let password = prompt_password("Enter master password: ").unwrap();

    if !verify_password(&vault.password_hash, &password) {
        eprintln!("Incorrect password. Access denied.");
        return;
    }

    let key = derive_key_from_password(&password, &vault.nonce);
    let key = Key::<Aes256Gcm>::from_slice(&key);
    let cipher = Aes256Gcm::new(key);

    match cipher.decrypt(Nonce::from_slice(&vault.nonce), vault.ciphertext.as_ref()) {
        Ok(decrypted) => {
            println!("Vault unlocked! Contents: {}", String::from_utf8_lossy(&decrypted));
        }
        Err(_) => {
            eprintln!("Failed to decrypt vault contents. Possibly wrong password.");
        }
    }
}
