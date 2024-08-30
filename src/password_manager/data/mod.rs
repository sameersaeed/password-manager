use crate::password_manager::encryption::Key;
use rand::Rng;
use rand::rngs::OsRng;
use rpassword::read_password;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub site: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PasswordManager {
    pub entries: Vec<Entry>,
    encryption_key: Vec<u8>,
    #[serde(skip)]
    pub filename: String,
}

impl PasswordManager {
    pub fn prompt_for_key() -> Vec<u8> {
        println!("Enter your encryption key to access your passwords:");
        let key = read_password().expect("Failed to read key");

        key.into_bytes()
    }

    pub fn new(filename: &str, key: Vec<u8>) -> Self {
        PasswordManager { 
            entries: Vec::new(), 
            encryption_key: key, 
            filename: filename.to_string() 
        }    
    }

    pub fn create_new_file(&self) -> io::Result<()> {
        let json = serde_json::to_vec(self).expect("Failed to serialize data");
        let encrypted_content = Key::encrypt(&json, &self.encryption_key);

        fs::write(&self.filename, encrypted_content)?;
        Ok(())
    }

    pub fn load_data(filename: &str, key: Option<Vec<u8>>) -> io::Result<Self> {
        let key = key.expect("You must provide a key to load your encrypted password data.\nIf you do not have a key, you can create a new file using the \"create-file\" command.");
        let decrypted_content = Key::decrypt(&fs::read(filename)?, &key);
        let mut passwords: PasswordManager = serde_json::from_slice(&decrypted_content).expect("Failed to deserialize data");
        passwords.filename = filename.to_string();

        Ok(passwords)
    }

    pub fn save_passwords(&self) -> io::Result<()> {
        let json = serde_json::to_vec(self).expect("Failed to serialize data");
        let encrypted_content = Key::encrypt(&json, &self.encryption_key);
        fs::write(&self.filename, encrypted_content)?;

        Ok(())
    }
    
    pub fn add_entry(&mut self, site: &str, username: &str, password: Option<&str>) {
        // overwrite if entry exists
        if let Some(existing_entry) = self.entries.iter_mut().find(|entry| entry.site == site) {
            existing_entry.username = username.to_string();
            existing_entry.password = password.unwrap_or(&existing_entry.password).to_string();

            println!("Updated entry for site: \"{}\"\n", site);
        } 
        else {
            let entry = Entry {
                site: site.to_string(),
                username: username.to_string(),
                password: password.unwrap_or("default_password").to_string(),
            };

            self.entries.push(entry);
            println!("Successfully added a new password for site: \"{}\"\n", site);
        }
    }

    pub fn remove_entry(&mut self, site: &str) {
        let index = self.entries.iter().position(|entry| entry.site == site);

        match index {
            Some(i) => {
                self.entries.remove(i);
                println!("Successfully removed password for site: \"{}\"\n", site);
            }
            None => {
                println!("ERROR: No password was found for site: \"{}\"\n", site);
            }
        }
    }

    pub fn edit_entry(&mut self, site: &str, username: &str, password: &str) {
        let mut found = false;
        for entry in &mut self.entries {
            if entry.site == site {
                entry.username = username.to_string();
                entry.password = password.to_string();
                println!("Successfully edited password for site: \"{}\"\n", site);

                found = true;
                break;
            }
        }

        if !found {
            println!("ERROR: No password was found for site: \"{}\"\n", site);
        }
    }

    pub fn get_entry(&self, site: &str) {
        if let Some(entry) = self.entries.iter().find(|entry| entry.site == site) {
            println!(
                "Found password for site: \"{}\"\nUsername: {}\nPassword: {}\n",
                entry.site, entry.username, entry.password
            );
        } 
        else {
            println!("ERROR: No password was found for site: \"{}\"\n", site);
        }
    }

    pub fn list_entries(&self) {
        if self.entries.is_empty() {
            println!("No site entries were found.\nYou can add new entries using the \"add-entry\" command.\n");
        } 
        else {
            println!("Listing all site entries");
            for entry in &self.entries {
                println!(
                    "site: {}\nusername: {}\npassword: {}\n",
                    entry.site, entry.username, entry.password
                );
            }
        }
    }

    fn get_character_set(include_special: bool) -> Vec<char> {
        const ALPHANUMERIC: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        const SPECIAL_CHARACTERS: &str = "!@#$%^&*()-_=+[]{};:,.<>?/|";
    
        let mut characters = ALPHANUMERIC.chars().collect::<Vec<char>>();
        
        if include_special {
            characters.extend(SPECIAL_CHARACTERS.chars());
        }
    
        characters
    }

    pub fn generate_password(&mut self, length: usize, site: Option<String>, include_special: bool) {
        let characters = Self::get_character_set(include_special);
        let mut rng = OsRng;
        let password: String = (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..characters.len());
                characters[idx]
            })
            .collect();

        if let Some(site) = site {
            if let Some(entry) = self.entries.iter_mut().find(|entry| entry.site == site) {
                let old_password = entry.password.clone();
                entry.password = password;
                println!("Updated password for site: \"{}\" from \"{}\" to \"{}\"\n", site, old_password, entry.password);
            } 
            else {
                println!("Site \"{}\" does not exist as an entry. Adding new entry with generated password.\n", site);
                self.entries.push(Entry {
                    site: site.clone(),
                    username: "default_username".to_string(),
                    password,
                });
                println!("Added new entry for site: \"{}\"\n", site);
            }
        } 
        else {
            println!("Generated password: {}\n", password);
        }
    }
}