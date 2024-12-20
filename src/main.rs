use clap::{Parser, Subcommand};
use std::env;
use std::path::PathBuf;
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

    match cli.command {
        Some(Commands::Init { path }) => {
            let path = if let Some(p) = path {
                PathBuf::from(p)
            } else {
                env::current_dir().unwrap()
            };

            match commands::init::run_init(&path) {
                Ok(_) => {
                    println!("Initialized a git repository at {:?}", &path);
                }
                Err(e) => println!("{:?}", e),
            }
        }
        None => {}
    }
}
