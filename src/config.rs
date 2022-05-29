// Imports
use serde::{Deserialize, Serialize};
use serde_json;
use std::{fs, path::Path};
use dirs;

// Main config
pub fn read_config(path: impl AsRef<Path>) -> Config {
    let config: Config = match fs::read_to_string(path) {
        Ok(x) => match serde_json::from_str(&x) {
            Ok(y) => y,
            Err(_) => {
                return create_default_config();
            }
        },
        Err(_) => {
            return create_default_config();
        }
    };
    return config;
}

fn create_default_config() -> Config {
    let defaultcfg = Config::default();
    // Check if home directory can be read
    match dirs::home_dir() {
        Some(x) => {
            let configdir = x.join(".config/orochi");
            let configfile = configdir.join("config.json");
            let data = serde_json::to_string_pretty(&defaultcfg).unwrap_or("".to_string());
            // Create needed folder if not exists
            match fs::create_dir_all(configdir) {
                Ok(_) => {
                    match fs::write(configfile, data) {
                        Ok(_) => {
                            return defaultcfg;
                        },
                        Err(_) => {
                            // TODO: Print error
                            return defaultcfg;
                        }
                    }
                },
                Err(_) => {
                    // TODO: Print error
                    return defaultcfg;
                }
            }
        },
        None => {
            // TODO: Print error
        }
    };
    return defaultcfg;
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
}
impl Config {
    fn default() -> Config {
        Config {
            server: ServerConfig::default()
        }
    }
}

// ---- Server config ----
#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    pub ipaddr: String,
    pub ipport: u16
}
impl ServerConfig {
    fn default() -> ServerConfig {
        ServerConfig {
            ipaddr: String::from("0.0.0.0"),
            ipport: 8080
        }
    }
}

// ---- Repo config ----
#[derive(Serialize, Deserialize)]
pub struct RepoConfig {

}