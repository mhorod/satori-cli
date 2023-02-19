use crate::token_storage::TokenStorage;

use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};
pub struct FileTokenStorage {
    path: PathBuf,
}

impl FileTokenStorage {
    const DEFAULT_TOKEN_PATH: &str = "~/.local/share/satori-cli/token.txt";

    pub fn default() -> FileTokenStorage {
        return FileTokenStorage::new(FileTokenStorage::DEFAULT_TOKEN_PATH);
    }

    pub fn new(path: &str) -> FileTokenStorage {
        FileTokenStorage {
            path: PathBuf::from(shellexpand::tilde(path).to_string()),
        }
    }
}

impl TokenStorage for FileTokenStorage {
    fn load_token(&self) -> Option<String> {
        let mut file = File::open(&self.path).ok()?;
        let mut token = String::new();
        file.read_to_string(&mut token).ok()?;
        match token.len() {
            0 => None,
            _ => Some(token),
        }
    }

    fn save_token(&self, token: &str) {
        let mut file = File::create(&self.path).unwrap();
        file.write_all(token.as_bytes()).unwrap();
        println!("Token saved to {}", self.path.display());
    }

    fn clear_token(&self) {
        std::fs::remove_file(&self.path);
    }
}
