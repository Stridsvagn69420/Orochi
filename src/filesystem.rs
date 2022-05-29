// Crates
use dirs;
use std::path::PathBuf;

pub fn homedir() -> PathBuf {
    match dirs::home_dir() {
        Some(x) => return x,
        None => return PathBuf::from(""),
    }
}