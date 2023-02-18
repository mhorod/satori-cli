
pub trait SatoriClient {
    fn get_token(&self) -> Option<String>;
    fn set_token(&self, token: &str);
    fn get_url(&self, path: &str) -> String;
    fn get(&self, path: &str) -> Option<String>;
    fn post(&self, path: &str, data: &[(&str, &str)]) -> Option<String>;
    fn submit_file(&self, path: &str, file_name: &str, file_path: &str) -> Option<String>;
}