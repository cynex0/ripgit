use clap::{Parser, Subcommand};
use core::repository::Repository;
use std::env;
mod commands;
mod core;

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
}

fn main() {
    let cli = Cli::parse();
    let mut repository: Option<Repository> = None;

    match cli.command {
        Some(Commands::Init { path }) => {
            let res = match path {
                Some(p) => commands::init::run_init(p),
                None => commands::init::run_init(env::current_dir().unwrap()),
            };

            match res {
                Ok(r) => {
                    println!("Initialized a git repository at {:?}", r.gitdir);
                    repository = Some(r);
                }
                Err(e) => println!("Error: {}", e),
            }
        }
        None => {}
    }
}
