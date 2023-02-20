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
}

impl<S: Satori, D: SatoriDisplay, P: Prompt> Satori for InteractiveSatori<S, D, P> {
    fn username(&self) -> SatoriResult<String> {
        let username = self.satori.username();
        self.display.display_username(&username);
        return username;
    }

    fn contests(&self, archived: bool, force: bool) -> SatoriResult<Vec<Contest>> {
        let contests = self.satori.contests(archived, force);
        self.display.display_contests(&contests);
        return contests;
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
        let problems = self.satori.problems(contest, force);
        if let Err(SatoriError::AmbiguousContest(error)) = &problems {
            let message = format!("Contest {} is ambiguous. Please choose one:", error.name);
            let choice = self.prompt.choose_option(&message, &error.candidates);
            match choice {
                None => {
                    self.display.display_error(&SatoriError::InvalidChoice);
                    return Err(SatoriError::InvalidChoice);
                }
                Some(choice) => {
                    let contest = &error.candidates[choice];
                    return self.problems(&contest, force);
                }
            }
        }
        self.display.display_problems(&problems);
        return problems;
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
