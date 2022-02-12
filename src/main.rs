// Crates
use std::env;

// Modules
mod server;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        // Print usage
        std::process::exit(0);
    } else {
        let cmd = &args[1];
        if *cmd == "server" {
            // Orochi Server
            let ipaddr = &args[2];
            let ipport = args[3].parse::<u16>().unwrap();
            let returncode = match server::main_server(String::from(ipaddr), ipport) {
                Ok(_) => 0,
                Err(_) => 1
            };
            std::process::exit(returncode);
        } else if *cmd == "kagero" {
            // Kagero Repo Manager
    
        } else if *cmd == "pacman" {
            // Pacman Repo Manager
    
        } else if *cmd == "apt" {
            // Apt Repo Manager
    
        } else {
            // Print usage
        }
    }
}