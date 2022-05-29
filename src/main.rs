// Crates
use std::env;

// Modules
mod server;
mod config;
mod filesystem;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        // Print usage
        std::process::exit(0);
    } else {
        let cfg = config::read_config(filesystem::homedir().join(".config/orochi/config.json"));
        match args[1].as_str() {
            "server" => {
                // Orochi Server
                let returncode: i32;
                if args.len() < 4 {
                    returncode = match server::main_server(cfg.server.ipaddr, cfg.server.ipport) {
                        Ok(_) => 0,
                        Err(_) => 1
                    };
                } else {
                    let ipaddr = &args[2];
                    let ipport = args[3].parse::<u16>().unwrap();
                    returncode = match server::main_server(String::from(ipaddr), ipport) {
                        Ok(_) => 0,
                        Err(_) => 1
                    };
                }
                std::process::exit(returncode);
            },
            _ => {
                // Print usage
            }
        }
    }
}