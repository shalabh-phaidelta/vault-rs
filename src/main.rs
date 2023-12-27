mod db;
use db::*;


fn clr() {
    println!("{}[2J", 27 as char);
}
fn main() {
    clr();
    let ascii = r#"

    ________  ________  ________   ________           ___      ___ ________  ___  ___  ___   _________   
    |\   __  \|\   __  \|\   ____\ |\   ____\         |\  \    /  /|\   __  \|\  \|\  \|\  \ |\___   ___\ 
    \ \  \|\  \ \  \|\  \ \  \___|_\ \  \___|_        \ \  \  /  / | \  \|\  \ \  \\\  \ \  \\|___ \  \_| 
    \ \   ____\ \   __  \ \_____  \\ \_____  \        \ \  \/  / / \ \   __  \ \  \\\  \ \  \    \ \  \  
     \ \  \___|\ \  \ \  \|____|\  \\|____|\  \        \ \    / /   \ \  \ \  \ \  \\\  \ \  \____\ \  \ 
      \ \__\    \ \__\ \__\____\_\  \ ____\_\  \        \ \__/ /     \ \__\ \__\ \_______\ \_______\ \__\
       \|__|     \|__|\|__|\_________\\_________\        \|__|/       \|__|\|__|\|_______|\|_______|\|__|
                          \|_________\|_________|                                                        



    "#;
    println!("{ascii}");

    let conn = db::init_db().expect("Failed to init db");

    loop{
        println!("Password Manager:");
        println!("1. Add Entry");
        println!("2. List Entries");
        println!("3. Delete Entry");
        println!("4. Search");
        println!("5. Quit");

        let mut choice: String = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                clr();
                let entry = ServiceInfo::new(
                    prompt("Enter service"),
                    prompt("Enter user name"),
                    prompt("Enter password"),
                );
                write_password_to_db(&conn, &entry.service, &entry.username, &entry.password).expect("Unable to add new entry to db");
                println!("Added entry to db successfully!");
            },

            "2" => {
                clr();
                let entries = read_password_from_db(&conn).expect("Failed to read from db");
                for service in entries {
                    println!("Service: {}", service.service);
                    println!("Username: {}", service.username);
                    println!("Password: {}", service.password);
                    
                    println!("********************************")
                }

            },
            "3" => {
                clr();
                todo!();
            },
            "4" => {
                clr();
                let svc_name = prompt("Enter service name to search");
                let svc = search_service_by_name(&conn, &svc_name);
                match svc {
                    Ok(Some(svc)) => {
                        println!("Service: {}
                        - Username: {}
                        - Password: {}
                        ", svc.service, svc.username, svc.password)
                    },
                    Ok(None) => println!("No service found"),
                    Err(e) => eprintln!("Error occurred while searching for svc! {}", e),
                }
            },
            "5" => {
                clr();
                println!("Bye!");
                break;
            },
            _ => {println!("Invalid choice");}
        }
        println!("\n\n");
    }

}
