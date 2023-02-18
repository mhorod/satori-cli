use crate::parser::SatoriParser;
use crate::prompt::Prompt;
use Satori;
use crate::satori::*;
use crate::satori_client::SatoriClient;
use crate::token_storage::TokenStorage;

pub struct SimpleSatori<Client: SatoriClient, Parser: SatoriParser, P: Prompt, T: TokenStorage> {
    client: Client,
    parser: Parser,
    prompt: P,
    token_storage: T,
}

impl<Client: SatoriClient, Parser: SatoriParser, P: Prompt, T: TokenStorage>
    SimpleSatori<Client, Parser, P, T>
{
    pub fn new(client: Client, parser: Parser, prompt: P, token_storage: T) -> Self {
        Self {
            client,
            parser,
            prompt,
            token_storage,
        }
    }

    fn log_in(&self, login: &str, password: &str) -> Option<()> {
        self.client.post("/login", &[("login", login), ("password", password)])?;
        self.token_storage.save_token(&self.client.get_token()?);
        Some(())
    }

    fn get_and_ensure_logged_in(&self, path: &str) -> Option<String> {
        // Try to use token first
        if let Some(token) = self.token_storage.load_token() {
            self.client.set_token(&token);
        }

        let page = self.client.get(path)?;
        
        // Invalid or expired token
        if self.parser.find_username(&page).is_none() {
            println!("You are not logged in. Please log in.");
            if let Some((login, password)) = self.prompt.ask_for_credentials() {
                self.log_in(&login, &password);
            } else {
                return None;
            }
        }
        Some(page)
    }
}

impl<Client: SatoriClient, Parser: SatoriParser, P: Prompt, T: TokenStorage> Satori
    for SimpleSatori<Client, Parser, P, T>
{
    fn contests(&self, archived: bool, force: bool) -> Option<Vec<Contest>> {
        let page = self.get_and_ensure_logged_in("/contest/select")?;
        self.parser.find_joined_contests(&page)
    }

    fn details(
        &self,
        contest: &str,
        problem: &str,
        submission: &str,
        force: bool,
    ) -> Option<ResultDetails> {
        todo!()
    }

    fn logout(&self) -> Option<()> {
        todo!()
    }

    fn problems(&self, contest: &str, force: bool) -> Option<Vec<Problem>> {
        let contests = self.contests(false, force)?;
        let contest_id = &contests
            .iter()
            .find(|c| c.name.starts_with(contest))?
            .id;

        println!("contest_id: {}", contest_id);

        let page = self.get_and_ensure_logged_in(&format!("/contest/{}/problems", contest_id))?;
        self.parser.find_problems(&page)
    }

    fn pdf(&self, contest: &str, problem: &str, force: bool) -> Option<()> {
        todo!()
    }

    fn results(
        &self,
        contest: &str,
        problem: &str,
        force: bool,
    ) -> Option<Vec<ShortResult>> {
        todo!()
    }

    fn status(&self, contest: &str, problem: &str, force: bool) -> Option<String> {
        todo!()
    }

    fn submit(&self, contest: &str, problem: &str, file_path: &str) -> Option<()> {
        todo!()
    }
}
