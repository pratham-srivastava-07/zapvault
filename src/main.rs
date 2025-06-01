mod cli;
mod hash;
mod meta;
use clap::Parser;
use cli::{CLI, Commands};
use hash::hash_password;
use meta::Vault;
// use std::path::PathBuf;

fn main() {
   let args = CLI::parse();

   match args.command {
        Commands::Init { path } => {
            println!("Init vault at {:?}", path);

            let password = rpassword::prompt_password("Enter master password: ").unwrap();
            let confirm = rpassword::prompt_password("Confirm password: ").unwrap();
    
            if password != confirm {
                eprintln!("Passwords do not match. Vault creation aborted.");
                return;
            }

            if path.exists() {
                eprintln!("Vault already exists at {:?}.", path);
                return;
            }

            std::fs::create_dir_all(&path).expect("Failed to create vault directory");

            let password_hash = hash_password(&password);

            let meta = Vault {
                password_hash
            };
            
            let file_path = path.join("vault.meta");
            let file = std::fs::File::create(&file_path).expect("Failed to create vault.meta");

            serde_json::to_writer_pretty(file, &meta).expect("Failed to write vault metadata");

            println!("Vault initialized at {:?}", path);
        }


        Commands::Write { tag } => {
            println!("Write a new entry. Tag: {:?}", tag);
        }

        Commands::List => {
            println!("Listing entries...");
        }

        Commands::Unlock => {
            println!("Unlocking entries...");

        }
   }
}
