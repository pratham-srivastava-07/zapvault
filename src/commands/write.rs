use std::{fs::{File, OpenOptions}, io::{self, Write, BufReader}};
use crate::meta::Vault;
use crate::hash::{derive_key_from_password, verify_password};
use aes_gcm::{Aes256Gcm, Key, Nonce, aead::{Aead, KeyInit, OsRng, rand_core::RngCore}};
use rpassword::prompt_password;
use serde_json;

pub fn handle_write(tag: Option<String>) {
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

    let file = File::open(&meta_data).expect("Failed to open vault.meta");
    let vault: Vault = serde_json::from_reader(file).expect("Failed to parse vault.meta");

    let password = prompt_password("Enter master password: ").unwrap();

    // verify user password
    if !verify_password(&vault.password_hash, &password) {
        eprintln!("Incorrect password. Cannot write entry.");
        return;
    }

    let tag = match tag {
        Some(t) => t,
        None => {
            print!("Enter a tag: ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read tag");
            input.trim().to_string()
        }
    };

    print!("Enter content: ");
    io::stdout().flush().unwrap();
    let mut content = String::new();
    io::stdin().read_line(&mut content).expect("Failed to read content");

    //  real password and generate unique nonce
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let key_bytes = derive_key_from_password(&password, &vault.nonce);
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let plaintext = format!("{}: {}", tag, content.trim());
    let ciphertext = cipher.encrypt(nonce, plaintext.as_bytes()).expect("encryption failed");

    let new_entry = Vault {
        password_hash: vault.password_hash.clone(),
        ciphertext,
        nonce: nonce_bytes,
    };

    let mut entries: Vec<Vault> = if data_file_path.exists() {
        let f = File::open(&data_file_path).expect("Failed to open vault.data");
        let reader = BufReader::new(f);
        serde_json::from_reader(reader).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    };

    entries.push(new_entry);

    let f = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&data_file_path)
        .expect("Failed to open or create vault.data");
    serde_json::to_writer_pretty(f, &entries).expect("Failed to write vault.data");

    println!("Entry successfully written to vault.");
}