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
        /// Path to initialize a new repository (optional)
        ///
        /// If not provided, the current directory is used
        ///
        /// If provided, all parent directories are created if they do not exist
        #[arg(short, long)]
        path: Option<String>,
    },
    Find {}, // TODO: for testing only, remove later!
    CatFile {
        /// Type of the object
        type_: utils::git_object_types::GitObjectTypes,
        /// Hash of the object
        hash: String,
    },
    HashObject {
        /// Write the object to disk
        #[arg(short, long)]
        write: bool,
        #[arg(short, long)]
        type_: Option<utils::git_object_types::GitObjectTypes>,
        /// File to hash
        path: String,
    },
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
        Some(Commands::CatFile { type_, hash }) => {
            commands::cat_file::run_cat_file(&current_dir, &hash);
        }
        Some(Commands::HashObject { write, type_, path }) => {
            let mut type_ = type_;
            if type_.is_none() {
                type_ = Some(utils::git_object_types::GitObjectTypes::Blob); // blob by default
            }

            if let Ok(hash) =
                commands::hash_object::run_hash_object(current_dir.join(path), type_.unwrap())
            {
                println!("{}", hash);
            } else {
                println!("Failed to hash object");
            }

            if write {
                todo!();
            }
        }
        None => {}
    }
}
