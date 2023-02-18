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
                _ => Some(username),
            })
    }

    fn find_joined_contests(&self, page: &str) -> Option<Vec<Contest>> {
        let soup = soup::Soup::new(page);
        let table = soup.tag("table").attr("class", "results").find()?;
        let mut contests = Vec::new();
        for row in table.tag("tr").find_all().skip(1) {
            let mut cells = row.tag("td").find_all();
            let cell = cells.next()?;
            let id = cell.tag("a").find()?.get("href").unwrap().split('/').skip(2).next()?.parse().ok()?;
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
            let pdf_url = cells.next()?.tag("a").find()?.get("href").unwrap().to_string();
            let deadline = cells.next()?.text().trim().to_string();

            let mut submit_url = String::new();
            let mut id = String::new();
            let mut contest_id = String::new();
            if let Some(submit_anchor) = cells.next()?.tag("a").find() {
                submit_url = submit_anchor.get("href").unwrap().to_string();
                contest_id= submit_url.split('/').skip(2).next()?.parse().ok()?;
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
}
