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
}
