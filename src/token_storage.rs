pub trait TokenStorage {
    fn load_token(&self) -> Option<String>;
    fn save_token(&self, token: &str);
    fn clear_token(&self);
}
