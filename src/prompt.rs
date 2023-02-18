
pub trait Prompt {
    fn ask_for_credentials(&self) -> Option<(String, String)>;
}