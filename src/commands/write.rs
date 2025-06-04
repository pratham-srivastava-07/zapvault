use std::{fs, io::{self, Write}};
use crate::meta::{Vault, VaultWrite};
// use crate::meta::Vault;
// use std::{fs};

pub fn handle_write(tag: Option<String>) {

    // ask for vault path 
    print!("Enter vault path: ");
    io::stdout().flush().unwrap();
    let mut path = String::new();
    io::stdin().read_line(&mut path).unwrap();
    let path = path.trim().trim_matches('"');
    let path_input = std::path::PathBuf::from(path);
    println!("{}", path);
    print!("{:?}", path_input);

    // search if the given folder path consists of vault.meta file
    let meta_data = path_input.join("vault.meta");
    if !meta_data.exists() {
        eprintln!("vault.meta not found at {:?}", meta_data);
        return;
    } else {
        println!("Vault found");
    }
    // getting file contents
    let file = fs::File::open(&meta_data).expect("Failed to open vault.meta file at location at given location");
    let vault: Vault = serde_json::from_reader(file).expect("Failed to parse vault meta data");
    
    let vault_entries = VaultWrite {
        password_hash: vault.password_hash.clone(),
        entries: vec![vault]
    };

    println!("{:?}",vault_entries);

    let tag = match tag {
        Some(t) => t,
        None => {
            print!("Enter a tag: ");
            io::stdout().flush().unwrap(); // ensure prompt is shown
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read tag");
            input.trim().to_string()

        }
    };
    let mut content = String::new();
    print!("Enter content: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut content).expect("Failed to read content");

    println!("The tag is: {}", tag);
    println!("The content received is: {}", content.trim());
}