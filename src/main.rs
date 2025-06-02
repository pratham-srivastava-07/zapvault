mod cli;
mod hash;
mod meta;
mod commands;
use clap::Parser;
use cli::{CLI, Commands};

fn main() {
   let args = CLI::parse();

   match args.command {
        Commands::Init { path } => {
            commands::init::handle_init(path);
        }

        Commands::Write { tag } => {
            println!("Write a new entry. Tag: {:?}", tag);
            commands::write::handle_write(tag);
        }

        Commands::List => {
            println!("Listing entries...");
        }

        Commands::Unlock => {
          commands::unlock::handle_unlock();
        }
   }
}
