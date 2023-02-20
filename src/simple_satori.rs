use crate::parser::SatoriParser;
use crate::prompt::Prompt;
use crate::satori::*;
use crate::satori_client::SatoriClient;
use crate::token_storage::TokenStorage;
use Satori;

pub struct SimpleSatori<Client: SatoriClient, Parser: SatoriParser, T: TokenStorage> {
    client: Client,
    parser: Parser,
    token_storage: T,
}

impl<Client: SatoriClient, Parser: SatoriParser, T: TokenStorage> SimpleSatori<Client, Parser, T> {
    pub fn new(client: Client, parser: Parser, token_storage: T) -> Self {
        Self {
            client,
            parser,
            token_storage,
        }
    }

    fn log_in(&self, login: &str, password: &str) -> Option<()> {
        self.client
            .post("/login", &[("login", login), ("password", password)])?;
        self.token_storage.save_token(&self.client.get_token()?);
        Some(())
    }

    fn get_and_ensure_logged_in(&self, path: &str) -> SatoriResult<String> {
        // Try to use token first
        if let Some(token) = self.token_storage.load_token() {
            self.client.set_token(&token);
        }

        match self.client.get(path) {
            None => Err(SatoriError::ConnectionFailed),
            Some(page) => {
                if self.parser.find_username(&page).is_some() {
                    Ok(page)
                } else {
                    Err(SatoriError::NotLoggedIn)
                }
            }
        }
    }
}

impl<Client: SatoriClient, Parser: SatoriParser, T: TokenStorage> Satori
    for SimpleSatori<Client, Parser, T>
{
    fn username(&self) -> SatoriResult<String> {
        let page = self.get_and_ensure_logged_in("")?;
        Ok(self.parser.find_username(&page).unwrap())
    }

    fn contests(&self, archived: bool, force: bool) -> Result<Vec<Contest>, SatoriError> {
        let page = self.get_and_ensure_logged_in("/contest/select")?;

        match self.parser.find_joined_contests(&page) {
            Some(contests) => Ok(contests),
            None => Err(SatoriError::ParsingFailed),
        }
    }

    fn details(
        &self,
        contest: &str,
        problem: &str,
        submission: &str,
        force: bool,
    ) -> SatoriResult<ResultDetails> {
        todo!()
    }

    fn login(&self, username: &str, password: &str) -> SatoriResult<String> {
        match self.log_in(username, password) {
            Some(_) => {
                let page = self.get_and_ensure_logged_in("")?;
                return Ok(self.parser.find_username(&page).unwrap());
            }
            None => Err(SatoriError::LoginFailed),
        }
    }

    fn logout(&self) -> SatoriResult<()> {
        self.token_storage.clear_token();
        Ok(())
    }

    fn problems(&self, contest: &str, force: bool) -> SatoriResult<Vec<Problem>> {
        let contests = self.contests(false, force)?;

        let candidates = contests
            .iter()
            .filter(|c| c.name.contains(contest))
            .collect::<Vec<_>>();

        let contest_id = match candidates.len() {
            0 => return Err(SatoriError::ContestNotFound),
            1 => candidates[0].id.clone(),
            _ => {
                return Err(SatoriError::AmbiguousContest(AmbiguousNameError {
                    name: contest.to_string(),
                    candidates: candidates.iter().map(|c| c.name.clone()).collect(),
                }))
            }
        };

        println!("contest_id: {}", contest_id);

        let page = self.get_and_ensure_logged_in(&format!("/contest/{}/problems", contest_id))?;
        match self.parser.find_problems(&page) {
            Some(problems) => Ok(problems),
            None => Err(SatoriError::ParsingFailed),
        }
    }

    fn pdf(&self, contest: &str, problem: &str, force: bool) -> SatoriResult<()> {
        todo!()
    }

    fn results(&self, contest: &str, problem: &str, force: bool) -> SatoriResult<Vec<ShortResult>> {
        todo!()
    }

    fn status(&self, contest: &str, problem: &str, force: bool) -> SatoriResult<String> {
        todo!()
    }

    fn submit(&self, contest: &str, problem: &str, file_path: &str) -> SatoriResult<()> {
        todo!()
    }
}
