#[derive(Debug)]
pub struct Contest {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug)]
pub struct Problem {
    pub contest_id: String,
    pub id: String,
    pub code: String,
    pub name: String,
    pub pdf_url: String,
    pub deadline: String,
    pub submit_url: String,
}

#[derive(Debug)]
pub struct ShortResult {
    pub submission_id: String,
    pub problem_code: String,
    pub time: String,
    pub status: String,
}

#[derive(Debug)]
pub struct TestCaseResult {
    pub test_case: String,
    pub status: String,
    pub time: String,
}

#[derive(Debug)]
pub struct ResultDetails {
    pub submission_id: String,
    pub problem_code: String,
    pub time: String,
    pub status: String,
    pub test_results: Vec<TestCaseResult>,
}

#[derive(Debug)]
pub enum SatoriError {
    NotLoggedIn,
    LoginFailed,
    ParsingFailed,
    ConnectionFailed,
    InvalidChoice,
    AmbiguousContest(AmbiguousNameError<Contest>),
    AmbiguousProblem(AmbiguousNameError<Problem>),
    ContestNotFound,
    ProblemNotFound,
    SubmissionNotFound,
}

#[derive(Debug)]
pub struct AmbiguousNameError<T> {
    pub name: String,
    pub candidates: Vec<T>,
}

pub type SatoriResult<T> = Result<T, SatoriError>;

pub trait Satori {
    fn username(&self) -> SatoriResult<String>;
    fn contests(&self, archived: bool, force: bool) -> SatoriResult<Vec<Contest>>;
    fn details(&self, contest: &str, submission: &str, force: bool) -> SatoriResult<ResultDetails>;
    fn login(&self, login: &str, password: &str) -> SatoriResult<String>;
    fn logout(&self) -> SatoriResult<()>;
    fn problems(&self, contest: &str, force: bool) -> SatoriResult<Vec<Problem>>;
    fn pdf(&self, contest: &str, problem: &str, force: bool) -> SatoriResult<()>;
    fn results(
        &self,
        contest: &str,
        problem: Option<&str>,
        limit: Option<usize>,
        force: bool,
    ) -> SatoriResult<Vec<ShortResult>>;
    fn status(&self, contest: &str, problem: &str, force: bool) -> SatoriResult<String>;
    fn submit(&self, contest: &str, problem: &str, file_path: &str) -> SatoriResult<()>;
}
