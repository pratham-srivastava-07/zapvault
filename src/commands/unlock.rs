use crate::{hash::verify_password, meta::Vault};
use std::{fs, io};
use std::{io::{Write}};
use rpassword::prompt_password;

pub fn handle_unlock() {
    println!("Unlocking vault...");

    print!("Enter vault path: ");
    io::stdout().flush().unwrap();

    let mut path_input = String::new();
    io::stdin().read_line(&mut path_input).unwrap();
    let path_input = path_input.trim();
    let path = std::path::PathBuf::from(path_input);

    let meta_path = path.join("vault.meta");
    if !meta_path.exists() {
        eprintln!("vault.meta not found at {:?}", meta_path);
        return;
    }

    let file = fs::File::open(&meta_path).expect("Failed to open vault.meta");
    let vault: Vault = serde_json::from_reader(file).expect("Failed to parse vault.meta");

    let password = prompt_password("Enter master password: ").unwrap();

    let valid = verify_password(&vault.password_hash, &password);

    if valid {
        println!("Vault unlocked successfully.");
    } else {
        eprintln!("Incorrect password. Access denied.");
    }
}
