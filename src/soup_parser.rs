use crate::parser::SatoriParser;
use crate::satori::*;

use soup::prelude::*;

pub struct SoupParser {}

impl SoupParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl SatoriParser for SoupParser {
    fn find_username(&self, page: &str) -> Option<String> {
        let soup = soup::Soup::new(page);
        soup.tag("div")
            .attr("id", "header")
            .find()?
            .tag("ul")
            .attr("class", "headerRightUl")
            .find()?
            .tag("li")
            .find()
            .map(|li| li.text().trim().to_string())
            .and_then(|username| match username.as_str() {
                "Register" => None,
                _ => Some(username.replace("Logged in as ", "")),
            })
    }

    fn find_joined_contests(&self, page: &str) -> Option<Vec<Contest>> {
        let soup = soup::Soup::new(page);
        let table = soup.tag("table").attr("class", "results").find()?;
        let mut contests = Vec::new();
        for row in table.tag("tr").find_all().skip(1) {
            let mut cells = row.tag("td").find_all();
            let cell = cells.next()?;
            let id = cell
                .tag("a")
                .find()?
                .get("href")
                .unwrap()
                .split('/')
                .skip(2)
                .next()?
                .parse()
                .ok()?;
            let name = cell.text().trim().to_string();
            let description = cells.next()?.text().trim().to_string();
            contests.push(Contest {
                id,
                name,
                description,
            });
        }

        Some(contests)
    }

    fn find_problems(&self, page: &str) -> Option<Vec<Problem>> {
        let soup = soup::Soup::new(page);
        let table = soup.tag("table").attr("class", "results").find()?;
        let mut problems = Vec::new();
        for row in table.tag("tr").find_all().skip(1) {
            let mut cells = row.tag("td").find_all();
            let code = cells.next()?.text().trim().to_string();
            let name = cells.next()?.text().trim().to_string();
            let pdf_url = cells
                .next()?
                .tag("a")
                .find()?
                .get("href")
                .unwrap()
                .to_string();
            let deadline = cells.next()?.text().trim().to_string();

            let mut submit_url = String::new();
            let mut id = String::new();
            let mut contest_id = String::new();
            if let Some(submit_anchor) = cells.next()?.tag("a").find() {
                submit_url = submit_anchor.get("href").unwrap().to_string();
                contest_id = submit_url.split('/').skip(2).next()?.parse().ok()?;
                id = submit_url.split('=').skip(1).next()?.parse().ok()?;
            }

            problems.push(Problem {
                contest_id,
                id,
                code,
                name,
                pdf_url,
                deadline,
                submit_url,
            });
        }

        Some(problems)
    }

    fn find_details(&self, page: &str) -> Option<ResultDetails> {
        let soup = soup::Soup::new(page);
        let main_info_table = soup.tag("table").attr("class", "results").find()?;
        let mut cells = main_info_table.tag("td").find_all();

        let submission_id = cells.next()?.text().trim().to_string();
        let _user = cells.next()?.text().trim().to_string();
        let problem_code = cells.next()?.text().trim().to_string();
        let time = cells.next()?.text().trim().to_string();
        let status = cells.next()?.text().trim().to_string();

        let results_table = soup.tag("tbody").attr("valign", "top").find()?;
        let mut test_results = Vec::new();

        for row in results_table.tag("tr").find_all() {
            let mut cells = row.tag("td").find_all();
            let test_case = cells.next()?.text().trim().to_string();
            let status = cells.next()?.text().trim().to_string();
            let time = cells.next()?.text().trim().to_string();

            test_results.push(TestCaseResult {
                test_case,
                status,
                time,
            });
        }

        Some(ResultDetails {
            submission_id,
            problem_code,
            time,
            status,
            test_results,
        })
    }

    fn find_results(&self, page: &str) -> Option<Vec<ShortResult>> {
        let soup = soup::Soup::new(page);
        let table = soup.tag("table").attr("class", "results").find()?;
        let mut results = Vec::new();

        for row in table.tag("tr").find_all().skip(1) {
            let mut cells = row.tag("td").find_all();

            let submission_id = cells.next()?.text().trim().to_string();
            let problem_code = cells.next()?.text().trim().to_string();
            let time = cells.next()?.text().trim().to_string();
            let status = cells.next()?.text().trim().to_string();

            results.push(ShortResult {
                submission_id,
                problem_code,
                time,
                status,
            });
        }

        Some(results)
    }
}
