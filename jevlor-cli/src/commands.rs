use std::path::PathBuf;

use clap::{ Parser, Subcommand };

fn about_commands() -> String {
    String::from("My cool description")
}

#[derive(Parser)]
#[command(about = about_commands(), version, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start process of word generation.
    Generate {
        path: PathBuf,
        #[arg(default_value_t = 10)]
        amount: usize,
        
        #[arg(short, long)]
        morphems: bool
    },
}

