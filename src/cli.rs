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
    CreateFile {
        #[arg(short, long, default_value = "passwords.json")]
        file: String,
    },
    AddEntry {
        site: String,
        username: String,
        password: String,
    },
    ListEntries {
        site: Option<String>,
    },
    RemoveEntry {
        site: String,
    },
    EditEntry {
        site: String,
        username: String,
        password: String,
    },
    GeneratePassword {
        #[arg(short, long, default_value_t = 16)]
        length: usize,
        #[arg(short, long)]
        site: Option<String>,
        #[arg(long, default_value_t = false)]
        include_special: bool,
    },
}