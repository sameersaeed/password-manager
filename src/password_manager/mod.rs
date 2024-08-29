use rand::Rng;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Write};

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub site: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PasswordManager {
    pub entries: Vec<Entry>,
}

impl PasswordManager {
    pub fn new() -> Self {
        PasswordManager { entries: Vec::new() }
    }

    pub fn load_passwords(filename: &str) -> io::Result<Self> {
        let file_content = fs::read_to_string(filename).unwrap_or("".to_string());

        if file_content.is_empty() {
            Ok(Self::new())
        } else {
            let passwords: PasswordManager = serde_json::from_str(&file_content).unwrap();
            Ok(passwords)
        }
    }

    pub fn save_passwords(&self, filename: &str) -> io::Result<()> {
        let json = serde_json::to_string(&self).unwrap();
        let file = File::create(filename)?;
        writeln!(&file, "{}", json)?;
        Ok(())
    }

    pub fn add_entry(&mut self, site: &str, username: &str, password: Option<&str>) {
        if let Some(existing_entry) = self.entries.iter_mut().find(|entry| entry.site == site) {
            existing_entry.username = username.to_string();
            existing_entry.password = password.unwrap_or(&existing_entry.password).to_string();
            println!("Updated entry for site: {}\n", site);
        } else {
            let entry = Entry {
                site: site.to_string(),
                username: username.to_string(),
                password: password.unwrap_or("default_password").to_string(),
            };

            self.entries.push(entry);
            println!("Successfully added a new password for site: {}\n", site);
        }
    }

    pub fn remove_entry(&mut self, site: &str) {
        let index = self.entries.iter().position(|entry| entry.site == site);

        match index {
            Some(i) => {
                self.entries.remove(i);
                println!("Successfully removed password for site: {}\n", site);
            }
            None => {
                println!("ERROR: No password found for site: {}\n", site);
            }
        }
    }

    pub fn edit_entry(&mut self, site: &str, username: &str, password: &str) {
        let mut found = false;
        for entry in &mut self.entries {
            if entry.site == site {
                entry.username = username.to_string();
                entry.password = password.to_string();
                println!("Successfully edited password for site: {}\n", site);
                found = true;
                break;
            }
        }

        if !found {
            println!("ERROR: No password found for site: {}\n", site);
        }
    }

    pub fn get_entry(&self, site: &str) {
        if let Some(entry) = self.entries.iter().find(|entry| entry.site == site) {
            println!(
                "Found password for site: {}\nUsername: {}\nPassword: {}\n",
                entry.site, entry.username, entry.password
            );
        } else {
            println!("ERROR: No password found for site: {}\n", site);
        }
    }

    pub fn list_entries(&self) {
        if self.entries.is_empty() {
            println!("No passwords found\n");
        } else {
            println!("Listing all passwords");
            for entry in &self.entries {
                println!(
                    "Site: {}\nUsername: {}\nPassword: {}\n",
                    entry.site, entry.username, entry.password
                );
            }
        }
    }

    pub fn generate_password(&mut self, length: usize, site: Option<String>, include_special: bool) {
        let characters: Vec<char> = if include_special {
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()-_=+[]{};:,.<>?/|".chars().collect()
        } else {
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".chars().collect()
        };

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
                println!("Updated password for site: {} from \"{}\" to \"{}\"\n", site, old_password, entry.password);
            } else {
                println!("Site {} does not exist. Adding new entry with generated password.\n", site);
                self.entries.push(Entry {
                    site: site.clone(),
                    username: "default_username".to_string(),
                    password,
                });
                println!("Added new entry for site: {}\n", site);
            }
        } else {
            println!("Generated password: {}\n", password);
        }
    }
}
