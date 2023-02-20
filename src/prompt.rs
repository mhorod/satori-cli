pub trait Prompt {
    fn ask_for_credentials(&self) -> Option<(String, String)>;
    fn choose_option(&self, message: &str, options: &Vec<String>) -> Option<usize>;
}
