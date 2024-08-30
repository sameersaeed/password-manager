mod cli;
mod password_manager;

use clap::Parser;
use cli::{Cli, Commands};
use password_manager::data::PasswordManager;

fn main() {
    let args = Cli::parse();

    let key = PasswordManager::prompt_for_key();
    let mut manager = PasswordManager::new("passwords.json", key.clone());

    match args.command {
        Commands::CreateFile { file } => {
            manager.filename = file;
            match manager.create_new_file() {
                Ok(_) => { 
                    println!("File \"{}\" was created successfully", &manager.filename);
                },
                Err(e) => eprintln!("Failed to create file \"{}\": {}", &manager.filename, e),
            }
        }
        Commands::AddEntry { site, username, password } => {
            match PasswordManager::load_data(&manager.filename, Some(key)) {
                Ok(mut mgr) => {
                    mgr.add_entry(&site, &username, Some(&password));
                    match mgr.save_passwords() {
                        Ok(_) => println!("New site entry \"{}\" was added successfully", &site),
                        Err(e) => eprintln!("Failed to add entry \"{}\" to file: {}", &site, e),
                    }
                }
                Err(e) => eprintln!("Failed to load file for entry creation: {}\nThe provided key may be incorrect. Please check your key or create a new file using the \"create-file\" command.", e),
            }
        }
        Commands::ListEntries { site } => {
            match PasswordManager::load_data(&manager.filename, Some(key)) {
                Ok(mgr) => {
                    if let Some(site) = site {
                        mgr.get_entry(&site);
                    } 
                    else {
                        mgr.list_entries();
                    }
                }
                Err(e) => eprintln!("Failed to load file \"{}\" for listing site entries: {}", &manager.filename, e),
            }
        }
        Commands::RemoveEntry { site } => {
            match PasswordManager::load_data(&manager.filename, Some(key)) {
                Ok(mut mgr) => {
                    mgr.remove_entry(&site);
                    match mgr.save_passwords() {
                        Ok(_) => println!("Site entry \"{}\" was removed successfully", &site),
                        Err(e) => eprintln!("Failed to remove site entry \"{}\": {}", &site, e),
                    }
                }
                Err(e) => eprintln!("Failed to load file \"{}\" for removing site entry \"{}\": {}", &manager.filename, &site, e),
            }
        }
        Commands::EditEntry { site, username, password } => {
            match PasswordManager::load_data(&manager.filename, Some(key)) {
                Ok(mut mgr) => {
                    mgr.edit_entry(&site, &username, &password);
                    match mgr.save_passwords() {
                        Ok(_) => println!("Site entry \"{}\" was edited successfully", &site),
                        Err(e) => eprintln!("Failed to save edits for entry \"{}\": {}", &site, e),
                    }
                }
                Err(e) => eprintln!("Failed to load file: {}\nThe provided key may be incorrect. Please check your key or create a new file using the \"create-file\" command.", e),
            }
        }
        Commands::GeneratePassword { length, site, include_special } => {
            match PasswordManager::load_data(&manager.filename, Some(key)) {
                Ok(mut mgr) => {
                    mgr.generate_password(length, site.clone(), include_special); 
                    match mgr.save_passwords() {
                        Ok(_) => println!("A new password was generated successfully for entry {:?}.", site.unwrap()), 
                        Err(e) => eprintln!("Failed to generate new password for entry {:?}: {}", site.unwrap(), e),
                    }
                }
                Err(e) => eprintln!("Failed to load file: {}\nThe provided key may be incorrect. Please check your key or create a new file using the \"create-file\" command.", e),
            }
        }
    }
}