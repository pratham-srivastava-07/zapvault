use crate::hash::hash_password;
use crate::meta::Vault;
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
    let meta = Vault { password_hash };

    let file_path = path.join("vault.meta");
    let file = fs::File::create(&file_path).expect("Failed to create vault.meta");

    serde_json::to_writer_pretty(file, &meta).expect("Failed to write vault metadata");

    println!("Vault initialized at {:?}", path);
}
