use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "lifelog", version, about = "Time-locked encrypted journal CLI")]
pub struct CLI {
    #[command(subcommand)] 
    pub command: Commands,
}


#[derive(Subcommand)]
pub enum Commands {
    Init {
        path: PathBuf
    },

    Write {
        #[arg(long, short)]
        tag: Option<String>
    },
    Unlock,
}