use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Simple Password Manager")]
#[command(version = "1.0")]
#[command(about = "A simple password manager, created using Rust", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Create {
        site: String,
        username: String,
        password: String,
    },
    List {
        site: Option<String>,
    },
    Remove {
        site: String,
    },
    Edit {
        site: String,
        username: String,
        password: String,
    },
    Generate {
        #[arg(short, long, default_value_t = 16)]
        length: usize,
        #[arg(short, long)]
        site: Option<String>,
        #[arg(long, default_value_t = false)]
        include_special: bool,
    },
}