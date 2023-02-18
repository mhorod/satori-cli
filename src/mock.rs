use crate::satori::*;
use crate::display::*;

pub struct MockSatori {}

impl MockSatori {
    pub fn new() -> MockSatori {
        MockSatori {}
    }
}

impl Satori for MockSatori {
    fn contests(&self, archived: bool, force: bool) -> Option<Vec<Contest>> {
        Some(vec![Contest {
            id: "1".to_string(),
            name: "Contest 1".to_string(),
            description: "Contest 1 description".to_string(),
        }])
    }

    fn details(&self, contest: &str, problem: &str, submission: &str, force: bool) -> Option<ResultDetails> {
        Some(ResultDetails {
            submission_id: "1".to_string(),
            problem_code: "A".to_string(),
            time: "2020-01-01 00:00:00".to_string(),
            status: "OK".to_string(),
            test_results: vec![TestCaseResult {
                test_case: "1".to_string(),
                status: "OK".to_string(),
                time: "0.01".to_string(),
            }],
        })
    }

    fn logout(&self) -> Option<()> {
        Some(())
    }

    fn problems(&self, contest: &str, force: bool) -> Option<Vec<Problem>> {
        Some(vec![Problem {
            contest_id: "1".to_string(),
            id: "1".to_string(),
            code: "A".to_string(),
            name: "Problem A".to_string(),
            pdf_url: "https://satori.tcs.uj.edu.pl/contest/1/problem/A".to_string(),
            deadline: "2020-01-01 00:00:00".to_string(),
            submit_url: "https://satori.tcs.uj.edu.pl/contest/1/problem/A/submit".to_string(),
        }])
    }

    fn pdf(&self, contest: &str, problem: &str, force: bool) -> Option<()> {
        Some(())
    }

    fn results(&self, contest: &str, problem: &str, force: bool) -> Option<Vec<ShortResult>> {
        Some(vec![ShortResult {
            submission_id: "1".to_string(),
            problem_code: "A".to_string(),
            time: "2020-01-01 00:00:00".to_string(),
            status: "OK".to_string(),
        }])
    }

    fn status(&self, contest: &str, problem: &str, force: bool) -> Option<String> {
        Some("OK".to_string())
    }

    fn submit(&self, contest: &str, problem: &str, file_path: &str) -> Option<()> {
        Some(())
    }
}


pub struct MockDisplay {}

impl MockDisplay {
    pub fn new() -> MockDisplay {
        MockDisplay {}
    }
}

impl SatoriDisplay for MockDisplay {
    fn display_contests(&self, contests: &Option<Vec<Contest>>) {
        println!("Contests: {:?}", contests);
    }

    fn display_details(&self, details: &Option<ResultDetails>) {
        println!("Details: {:?}", details);
    }

    fn display_logout(&self, logout: &Option<()>) {
        println!("Logout: {:?}", logout);
    }

    fn display_problems(&self, problems: &Option<Vec<Problem>>) {
        println!("Problems: {:?}", problems);
    }

    fn display_pdf(&self, pdf: &Option<()>) {
        println!("PDF: {:?}", pdf);
    }

    fn display_results(&self, results: &Option<Vec<ShortResult>>) {
        println!("Results: {:?}", results);
    }

    fn display_status(&self, status: &Option<String>) {
        println!("Status: {:?}", status);
    }

    fn display_submit(&self, submit: &Option<()>) {
        println!("Submit: {:?}", submit);
    }
}