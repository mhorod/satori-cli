use crate::satori::*;

pub trait SatoriParser {
    fn find_username(&self, page: &str) -> Option<String>;
    fn find_joined_contests(&self, page: &str) -> Option<Vec<Contest>>;
}