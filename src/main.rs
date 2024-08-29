mod password_manager;
mod cli;

use password_manager::PasswordManager;
use cli::{Cli, Commands};
use clap::Parser;

fn main() {
    let args = Cli::parse();
    let mut pwmanager = PasswordManager::load_passwords("passwords.json").expect("Failed to load password data");

    match args.command {
        Commands::Create { site, username, password } => {
            pwmanager.add_entry(&site, &username, Some(&password));
        }
        Commands::List { site } => {
            if let Some(site) = site {
                pwmanager.get_entry(&site);
            } else {
                pwmanager.list_entries();
            }
        }
        Commands::Remove { site } => {
            pwmanager.remove_entry(&site);
        }
        Commands::Edit { site, username, password } => {
            pwmanager.edit_entry(&site, &username, &password);
        }
        Commands::Generate { length, site, include_special } => {
            pwmanager.generate_password(length, site, include_special);
        }
    }

    pwmanager.save_passwords("passwords.json").expect("[ERROR]: Failed to save password data");
}