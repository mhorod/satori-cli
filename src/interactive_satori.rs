use crate::display::*;
use crate::prompt::*;
use crate::satori::*;

pub struct InteractiveSatori<S: Satori, D: SatoriDisplay, P: Prompt> {
    satori: S,
    display: D,
    prompt: P,
}

impl<S: Satori, D: SatoriDisplay, P: Prompt> InteractiveSatori<S, D, P> {
    pub fn new(satori: S, display: D, prompt: P) -> Self {
        Self {
            satori,
            display,
            prompt,
        }
    }

    fn log_in(&self) -> SatoriResult<String> {
        let credentials = self.prompt.ask_for_credentials();
        match credentials {
            None => {
                self.display.display_error(&SatoriError::LoginFailed);
                return Err(SatoriError::LoginFailed);
            }
            Some((login, password)) => {
                return self.satori.login(&login, &password);
            }
        }
    }

    fn disambiguate_contest<'a>(
        &self,
        error: &'a AmbiguousNameError<Contest>,
    ) -> Option<&'a Contest> {
        let message = format!("Contest {} is ambiguous. Please choose one:", error.name);
        let candidates = error
            .candidates
            .iter()
            .map(|c| c.name.clone())
            .collect::<Vec<String>>();
        let choice = self.prompt.choose_option(&message, &candidates);
        match choice {
            None => {
                return None;
            }
            Some(choice) => {
                if choice >= error.candidates.len() {
                    return None;
                }
                return Some(&error.candidates[choice]);
            }
        }
    }

    fn disambiguate_problem<'a>(
        &self,
        error: &'a AmbiguousNameError<Problem>,
    ) -> Option<&'a Problem> {
        let message = format!("Problem {} is ambiguous. Please choose one:", error.name);
        let candidates = error
            .candidates
            .iter()
            .map(|c| format!("[{}] {}", c.code, c.name))
            .collect::<Vec<String>>();
        let choice = self.prompt.choose_option(&message, &candidates);
        match choice {
            None => None,
            Some(choice) => {
                if choice >= error.candidates.len() {
                    None
                } else {
                    Some(&error.candidates[choice])
                }
            }
        }
    }
}

macro_rules! repeat_until_logged_in {
    ($self:ident, $action:expr) => {
        loop {
            let result = $action;
            match result {
                Err(SatoriError::NotLoggedIn) => {
                    let result = $self.log_in();
                    match result {
                        Ok(_) => continue,
                        Err(SatoriError::LoginFailed) => {
                            $self.display.display_error(&SatoriError::LoginFailed);
                            continue;
                        }
                        Err(error) => {
                            break Err(error);
                        }
                    }
                }
                _ => {
                    break result;
                }
            }
        }
    };
}

impl<S: Satori, D: SatoriDisplay, P: Prompt> Satori for InteractiveSatori<S, D, P> {
    fn username(&self) -> SatoriResult<String> {
        let username = self.satori.username();
        self.display.display_username(&username);
        return username;
    }

    fn contests(&self, archived: bool, force: bool) -> SatoriResult<Vec<Contest>> {
        let contests = repeat_until_logged_in!(self, self.satori.contests(archived, force));
        self.display.display_contests(&contests);
        return contests;
    }

    fn details(&self, contest: &str, submission: &str, force: bool) -> SatoriResult<ResultDetails> {
        let details =
            repeat_until_logged_in!(self, self.satori.details(contest, submission, force));
        self.display.display_details(&details);
        return details;
    }

    fn login(&self, login: &str, password: &str) -> SatoriResult<String> {
        let result = self.satori.login(login, password);
        self.display.display_login(&result);
        return result;
    }

    fn logout(&self) -> SatoriResult<()> {
        let result = self.satori.logout();
        self.display.display_logout(&result);
        return result;
    }

    fn problems(&self, contest: &str, force: bool) -> SatoriResult<Vec<Problem>> {
        let problems = repeat_until_logged_in!(self, self.satori.problems(contest, force));
        let problems = match problems {
            Err(SatoriError::AmbiguousContest(error)) => match self.disambiguate_contest(&error) {
                None => Err(SatoriError::InvalidChoice),
                Some(contest) => self.problems(&contest.id, force),
            },
            result => result,
        };
        self.display.display_problems(&problems);
        return problems;
    }

    fn pdf(&self, contest: &str, problem: &str, force: bool) -> SatoriResult<()> {
        let pdf = repeat_until_logged_in!(self, self.satori.pdf(contest, problem, force));
        self.display.display_pdf(&pdf);
        return pdf;
    }

    fn results(
        &self,
        contest: &str,
        problem: Option<&str>,
        limit: Option<usize>,
        force: bool,
    ) -> SatoriResult<Vec<ShortResult>> {
        let results =
            repeat_until_logged_in!(self, self.satori.results(contest, problem, limit, force));

        let results = match results {
            Err(SatoriError::AmbiguousContest(error)) => match self.disambiguate_contest(&error) {
                None => Err(SatoriError::InvalidChoice),
                Some(contest) => self.results(&contest.id, problem, limit, force),
            },
            Err(SatoriError::AmbiguousProblem(error)) => match self.disambiguate_problem(&error) {
                None => Err(SatoriError::InvalidChoice),
                Some(problem) => self.results(contest, Some(&problem.id), limit, force),
            },
            result => result,
        };

        self.display.display_results(&results);
        return results;
    }

    fn status(&self, contest: &str, problem: &str, force: bool) -> SatoriResult<String> {
        let status = repeat_until_logged_in!(self, self.satori.status(contest, problem, force));
        self.display.display_status(&status);
        return status;
    }

    fn submit(&self, contest: &str, problem: &str, file_path: &str) -> SatoriResult<()> {
        let submit = repeat_until_logged_in!(self, self.satori.submit(contest, problem, file_path));
        self.display.display_submit(&submit);
        return submit;
    }
}
