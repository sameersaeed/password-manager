# Password Manager
a simple password manager created using Rust. uses pbkdf2-sha256 encryption for password storage
<br>

## **installation:**
```
git clone https://github.com/sameersaeed/password-manager
cd password-manager/
cargo run
```

you should then see the following output:
```
Usage: password-manager <COMMAND>

Commands:
  create-file        
  add-entry          
  list-entries       
  remove-entry       
  edit-entry         
  generate-password  
  help               Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```