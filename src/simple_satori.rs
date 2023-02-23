use crate::parser::SatoriParser;
use crate::satori::*;
use crate::satori_client::SatoriClient;
use crate::token_storage::TokenStorage;
use futures::stream::ForEachConcurrent;
use Satori;

enum UniqueSearchResult<T> {
    NotFound,
    Found(T),
    Ambiguous(Vec<T>),
}

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

    fn find_unique_contest(
        &self,
        contests: Vec<Contest>,
        prefix: &str,
    ) -> UniqueSearchResult<Contest> {
        let mut found_contests = contests
            .into_iter()
            .filter(|contest| contest.id.starts_with(prefix) || contest.name.starts_with(prefix))
            .collect::<Vec<Contest>>();

        match found_contests.len() {
            0 => UniqueSearchResult::NotFound,
            1 => UniqueSearchResult::Found(found_contests.pop().unwrap()),
            _ => UniqueSearchResult::Ambiguous(found_contests),
        }
    }

    fn find_unique_problem(
        &self,
        problems: Vec<Problem>,
        prefix: &str,
    ) -> UniqueSearchResult<Problem> {
        let mut found_problems = problems
            .into_iter()
            .filter(|problem| {
                problem.id.starts_with(prefix)
                    || problem.name.starts_with(prefix)
                    || problem.code.starts_with(prefix)
            })
            .collect::<Vec<Problem>>();

        match found_problems.len() {
            0 => UniqueSearchResult::NotFound,
            1 => UniqueSearchResult::Found(found_problems.pop().unwrap()),
            _ => UniqueSearchResult::Ambiguous(found_problems),
        }
    }

    fn contest(&self, contest: &str, force: bool) -> SatoriResult<Contest> {
        let contests = self.contests(false, force)?;
        let contest = match self.find_unique_contest(contests, contest) {
            UniqueSearchResult::NotFound => return Err(SatoriError::ContestNotFound),
            UniqueSearchResult::Ambiguous(contests) => {
                return Err(SatoriError::AmbiguousContest(AmbiguousNameError {
                    name: contest.to_string(),
                    candidates: contests,
                }))
            }
            UniqueSearchResult::Found(contest) => contest,
        };
        return Ok(contest);
    }

    fn problem(&self, contest: &str, problem: &str, force: bool) -> SatoriResult<Problem> {
        let problems = self.problems(contest, force)?;
        let problem = match self.find_unique_problem(problems, problem) {
            UniqueSearchResult::NotFound => return Err(SatoriError::ProblemNotFound),
            UniqueSearchResult::Ambiguous(problems) => {
                return Err(SatoriError::AmbiguousProblem(AmbiguousNameError {
                    name: problem.to_string(),
                    candidates: problems,
                }))
            }
            UniqueSearchResult::Found(problem) => problem,
        };
        return Ok(problem);
    }
}

impl<Client: SatoriClient, Parser: SatoriParser, T: TokenStorage> Satori
    for SimpleSatori<Client, Parser, T>
{
    fn username(&self) -> SatoriResult<String> {
        let page = self.get_and_ensure_logged_in("")?;
        Ok(self.parser.find_username(&page).unwrap())
    }

    fn contests(&self, _archived: bool, _force: bool) -> Result<Vec<Contest>, SatoriError> {
        let page = self.get_and_ensure_logged_in("/contest/select")?;

        match self.parser.find_joined_contests(&page) {
            Some(contests) => Ok(contests),
            None => Err(SatoriError::ParsingFailed),
        }
    }

    fn details(
        &self,
        contest: &str,
        submission: &str,
        _force: bool,
    ) -> SatoriResult<ResultDetails> {
        let contest = self.contest(contest, false)?;
        let page = self
            .get_and_ensure_logged_in(&format!("/contest/{}/results/{}", contest.id, submission))?;

        match self.parser.find_details(&page) {
            Some(details) => Ok(details),
            None => Err(SatoriError::ParsingFailed),
        }
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
        let contest_id = &self.contest(contest, force)?.id;
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

    fn results(
        &self,
        contest: &str,
        problem: Option<&str>,
        limit: Option<usize>,
        force: bool,
    ) -> SatoriResult<Vec<ShortResult>> {
        let contest = self.contest(contest, force)?;
        let limit_filter = match limit {
            Some(limit) => format!("results_limit={}", limit),
            None => "".to_string(),
        };

        let problem_filter = match problem {
            Some(problem) => {
                let problem = self.problem(contest.id.as_str(), problem, force)?;
                format!("results_filter_problem={}", problem.id)
            }
            None => "".to_string(),
        };

        let filter = match (limit_filter.is_empty(), problem_filter.is_empty()) {
            (true, true) => "".to_string(),
            (false, true) => format!("?{}", limit_filter),
            (true, false) => format!("?{}", problem_filter),
            (false, false) => format!("?{}&{}", limit_filter, problem_filter),
        };

        let page =
            self.get_and_ensure_logged_in(&format!("/contest/{}/results{}", &contest.id, filter))?;

        match self.parser.find_results(&page) {
            Some(results) => Ok(results),
            None => Err(SatoriError::ParsingFailed),
        }
    }

    fn status(&self, contest: &str, problem: &str, force: bool) -> SatoriResult<String> {
        todo!()
    }

    fn submit(&self, contest: &str, problem: &str, file_path: &str) -> SatoriResult<()> {
        todo!()
    }
}
