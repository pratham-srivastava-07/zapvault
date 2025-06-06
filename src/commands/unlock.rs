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
    // checkk for decrypting and showing user what they have stored in it
    let data_path = path.join("vault.data");
    if !data_path.exists() {
        eprintln!("No entries found (vault.data missing).");
        return;
    }

    let file = fs::File::open(&data_path).expect("Failed to open vault.data");
    let entries: Vec<Vault> = serde_json::from_reader(file).expect("Failed to parse vault.data");

    println!("\nDecrypted Vault Entries:");
    for (i, entry) in entries.iter().enumerate() {
        let key = derive_key_from_password(&password, &entry.nonce);
        let key = Key::<Aes256Gcm>::from_slice(&key);
        let cipher = Aes256Gcm::new(key);

        match cipher.decrypt(Nonce::from_slice(&entry.nonce), entry.ciphertext.as_ref()) {
            Ok(decrypted) => {
                println!("{}. {}", i + 1, String::from_utf8_lossy(&decrypted));
            }
            Err(_) => {
                println!("{}. [Failed to decrypt entry]", i + 1);
            }
        }
    }

    let key = derive_key_from_password(&password, &vault.nonce);
    let key = Key::<Aes256Gcm>::from_slice(&key);
    let cipher = Aes256Gcm::new(key);

    match cipher.decrypt(Nonce::from_slice(&vault.nonce), vault.ciphertext.as_ref()) {
        Ok(_) => {
            println!("Vault unlocked!");
        }
        Err(_) => {
            eprintln!("Failed to decrypt vault contents. Possibly wrong password.");
        }
    }
}
