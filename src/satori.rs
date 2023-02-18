
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

pub trait Satori {
    fn contests(&self, archived: bool, force: bool) -> Option<Vec<Contest>>;
    fn details(&self, contest: &str, problem: &str, submission: &str, force: bool) -> Option<ResultDetails>;
    fn logout(&self) -> Option<()>;
    fn problems(&self, contest: &str, force: bool) -> Option<Vec<Problem>>;
    fn pdf(&self, contest: &str, problem: &str, force: bool) -> Option<()>;
    fn results(&self, contest: &str, problem: &str, force: bool) -> Option<Vec<ShortResult>>;
    fn status(&self, contest: &str, problem: &str, force: bool) -> Option<String>;
    fn submit(&self, contest: &str, problem: &str, file_path: &str) -> Option<()>;
}