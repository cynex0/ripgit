use clap::{Parser, Subcommand};
use core::repository::Repository;
use std::env;
use std::path::PathBuf;
mod commands;
mod core;
mod utils;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Init {
        #[arg(short, long)]
        path: Option<String>,
    },
    Find {}, // TODO: for testing only, remove later!
}

fn main() {
    let cli = Cli::parse();
    let current_dir = env::current_dir().expect("Failed to get current directory");

    match cli.command {
        Some(Commands::Init { path }) => {
            let path = if let Some(p) = path {
                PathBuf::from(p)
            } else {
                current_dir
            };

            match commands::init::run_init(&path) {
                Ok(_) => {
                    println!("Initialized a git repository at {:?}", &path);
                }
                Err(e) => println!("{:?}", e),
            }
        }
        Some(Commands::Find {}) => {
            if let Some(repo) = Repository::find(&current_dir) {
                println!("Found repo at {:?}", repo.gitdir);
            } else {
                println!("Not a git repository");
            }
        }
        None => {}
    }
}
