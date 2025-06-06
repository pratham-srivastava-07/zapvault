use std::{fs::{File, OpenOptions}, io::{self, Write, BufReader}};
use crate::meta::{Vault};
use crate::hash::derive_key_from_password;
use aes_gcm::{Aes256Gcm, Key, Nonce, aead::{Aead, KeyInit, OsRng, rand_core::RngCore}};
use serde_json;

pub fn handle_write(tag: Option<String>) {
    // Ask for vault path
    print!("Enter vault path: ");
    io::stdout().flush().unwrap();
    let mut path = String::new();
    io::stdin().read_line(&mut path).unwrap();
    let path = path.trim().trim_matches('"');
    let path_input = std::path::PathBuf::from(path);

    let meta_data = path_input.join("vault.meta");
    let data_file_path = path_input.join("vault.data");

    if !meta_data.exists() {
        eprintln!("vault.meta not found at {:?}", meta_data);
        return;
    }

    // Read vault metadata (to get password hash and salt)
    let file = File::open(&meta_data).expect("Failed to open vault.meta");
    let vault: Vault = serde_json::from_reader(file).expect("Failed to parse vault.meta");

    // Prompt for tag
    match tag {
        Some(t) => t,
        None => {
            print!("Enter a tag: ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read tag");
            input.trim().to_string()
        }
    };

    // Prompt for content
    let mut content = String::new();
    print!("Enter content: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut content).expect("Failed to read content");

    // Encrypt content
    let salt = b"fixedsalt"; // Ideally store or derive a unique salt per user
    let key_bytes = derive_key_from_password(&vault.password_hash, salt); // Use hashed password to derive encryption key
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, content.trim().as_bytes()).expect("encryption failed");

    let new_entry = Vault {
        password_hash: vault.password_hash.clone(),
        ciphertext,
        nonce: nonce_bytes,
    };

    // Read existing entries from vault.data (if exists)
    let mut entries: Vec<Vault> = if data_file_path.exists() {
        let f = File::open(&data_file_path).expect("Failed to open vault.data");
        let reader = BufReader::new(f);
        serde_json::from_reader(reader).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    };

    // Append new entry
    entries.push(new_entry);

    // Write updated entries back
    let f = OpenOptions::new().write(true).create(true).truncate(true).open(&data_file_path)
        .expect("Failed to open or create vault.data");
    serde_json::to_writer_pretty(f, &entries).expect("Failed to write vault.data");

    println!("Entry successfully written to vault.");
}