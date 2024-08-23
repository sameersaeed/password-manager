use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Write};

#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    site: String,
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PasswordManager {
    entries: Vec<Entry>,
}

impl PasswordManager {
    fn new() -> Self {
        PasswordManager { entries: Vec::new() }
    }

    fn load_passwords(filename: &str) -> io::Result<Self> {
        let file_content = fs::read_to_string(filename).unwrap_or("".to_string());

        if file_content.is_empty() {
            return Ok(Self::new());
        }
        else {
            let passwords: PasswordManager = serde_json::from_str(&file_content).unwrap();
            return Ok(passwords);
        }
    }

    fn save_passwords(&self, filename: &str) -> io::Result<()> {
        let json = serde_json::to_string(&self).unwrap();
        let file = File::create(filename)?;
        writeln!(&file, "{}", json)?;

        Ok(())
    }

    fn add_entry(&mut self, site: &str, username: &str, password: &str) {
        let entry = Entry {
            site: site.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        };

        self.entries.push(entry);
        println!("\n[LOG]: Successfully added a new password for site: {}\n", site);
    }

    fn remove_entry(&mut self, site: &str) {
        let mut index = None;
        for (i, entry) in self.entries.iter().enumerate() {
            if entry.site == site {
                index = Some(i);
                break;
            }
        }

        match index {
            Some(i) => {
                self.entries.remove(i);
                println!("\n[LOG]: Successfully removed password for site: {}\n", site);
            }
            None => {
                println!("\n[ERROR]: No password found for site: {}\n", site);
            }
        }
    }

    fn edit_entry(&mut self, site: &str, username: &str, password: &str) {
        let mut found = false;
        for entry in &mut self.entries {
            if entry.site == site {
                entry.username = username.to_string();
                entry.password = password.to_string();
                println!("\n[LOG]: Successfully edited password for site: {}\n", site);
                found = true;
                break;
            }
        }

        if !found {
            println!("\n[ERROR]: No password found for site: {}\n", site);
        }
    }

    fn get_entry(&self, site: &str) {
        let mut found = false;
        for entry in &self.entries {
            if entry.site == site {
                println!("\n[LOG]: Found password for site: {}\nUsername: {}\nPassword: {}\n", entry.site, entry.username, entry.password);
                found = true;
                break;
            }
        }

        if !found {
            println!("\n[ERROR]: No password found for site: {}\n", site);
        }
    }

    fn list_entries(&self) {
        if self.entries.is_empty() {
            println!("\n[LOG]: No passwords found\n");
        }
        else {
            println!("\n[LOG]: Listing all passwords");
            for entry in &self.entries {
                println!("Site: {}\nUsername: {}\nPassword: {}\n", entry.site, entry.username, entry.password);
            }
        }
    }
}

fn main() {
    let mut pwmanager = PasswordManager::load_passwords("passwords.json").expect("Failed to load password data");

    loop {
        println!("--- Password Manager ---");
        println!("1. Add site entry");
        println!("2. Remove site entry");
        println!("3. Edit site entry");
        println!("4. Get site password");
        println!("5. List all site passwords");
        println!("6. Exit");
        println!("------------------------\n");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("[ERROR]: Failed to read selected menu option");

        match choice.trim() {
            "1" => {    // create
                let (site, username, password) = get_input("create");
                pwmanager.add_entry(&site, &username.unwrap(), &password.unwrap());
            }
            "2" => {    // remove
                list_sites(&pwmanager);

                let (site, _, _) = get_input("remove");
                pwmanager.remove_entry(&site.trim());
            }
            "3" => {    // edit
                list_sites(&pwmanager);

                let (site,  username, password) = get_input("edit");
                pwmanager.edit_entry(&site, &username.unwrap(), &password.unwrap());
            }
            "4" => {    // view (single entry)
                list_sites(&pwmanager);

                let (site, _, _) = get_input("view");
                pwmanager.get_entry(&site.trim());
            }
            "5" => {    // view (all entries)
                pwmanager.list_entries();
            }
            "6" => {    // save and quit
                pwmanager.save_passwords("passwords.json").expect("[ERROR]: Failed to save password data");
                break;
            }
            _ => {
                println!("[ERROR]: An invalid menu option was selected");
            }
        }
    }
}
    
fn get_input(action: &str) -> (String, Option<String>, Option<String>) {
    // remove / list entry only requires site input
    let mut site = String::new();
    println!("Enter site to {}: ", action);
    io::stdin().read_line(&mut site).expect("[ERROR]: Failed to read site input");

    // get username + password if editing / creating entry
    let (username, password) = if action == "create" || action == "edit" {
        let mut username = String::new();
        let mut password = String::new();

        println!("Enter username: ");
        io::stdin().read_line(&mut username).expect("[ERROR]: Failed to read username input");
        println!("Enter password: ");
        io::stdin().read_line(&mut password).expect("[ERROR]: Failed to read password input");

        (Some(username.trim().to_string()), Some(password.trim().to_string()))
    } else {
        (None, None)
    };

    (site.trim().to_string(), username, password)
}


fn list_sites(pwmanager: &PasswordManager) {
    let mut sites: Vec<&String> = pwmanager.entries.iter().map(|e| &e.site).collect();
    sites.sort();
    
    if sites.is_empty() {
        println!("\n[LOG]: No saved site passwords found.\n");
    } else {
        println!("\nSaved site passwords:");
        for site in &sites {
            println!("- {}", site);
        }
        println!("");
    }
}