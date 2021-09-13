use itertools::Itertools;
use regex::Regex;
use soup::prelude::*;

use std::path::{Path, PathBuf};

use crate::satori::*;

pub struct SatoriClient {
    url: String,
    token: String,
    login: String,
    password: String,
    token_file: PathBuf,
    client: reqwest::Client,
}

impl SatoriClient {
    pub async fn new(
        url: &str,
        token_file: &Path,
        login: &str,
        password: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let token = match std::fs::read_to_string(token_file) {
            Ok(token) => token.trim().to_owned(),
            Err(_) => update_token(url, token_file, login, password).await?,
        };
        println!("{}", token);
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Cookie",
            reqwest::header::HeaderValue::from_str(token.as_str()).unwrap(),
        );
        Ok(Self {
            url: url.to_owned(),
            token,
            login: login.to_owned(),
            password: password.to_owned(),
            token_file: token_file.to_owned(),
            client: reqwest::Client::builder()
                .default_headers(headers)
                .build()?,
        })
    }

    async fn update_token(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("updating_token");
        self.token = update_token(&self.url, &self.token_file, &self.login, &self.password).await?;
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Cookie",
            reqwest::header::HeaderValue::from_str(self.token.as_str()).unwrap(),
        );
        self.client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;
        Ok(())
    }

    async fn get(
        &mut self,
        address: &str,
    ) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        match self.client.get(self.url.clone() + address).send().await {
            Err(_) => {
                self.update_token().await?;
                Ok(self.client.get(self.url.clone() + address).send().await?)
            }
            Ok(response) => Ok(response),
        }
    }

    pub async fn get_problems(
        &mut self,
        contest_id: &str,
    ) -> Result<
        impl Iterator<Item = ParsingResult<(String, impl Iterator<Item = ParsingResult<Problem>>)>>,
        Box<dyn std::error::Error>,
    > {
        let html = self
            .get(format!("/contest/{}/problems", contest_id).as_str())
            .await?
            .text()
            .await?;
        let soup = Soup::new(&html);
        let content = soup
            .tag("div")
            .attr("id", "content")
            .find()
            .ok_or(ParsingError)?;
        let tags = content.tag(Regex::new("table|h4").unwrap()).find_all();

        Ok(tags.tuples().map(|(header, table)| {
            Ok((
                header
                    .text()
                    .split("[")
                    .next()
                    .ok_or(ParsingError)?
                    .trim()
                    .to_owned(),
                table
                    .tag("tr")
                    .find_all()
                    .skip(1)
                    .map(|row| -> Result<Problem, ParsingError> {
                        let (code_cell, name_cell, _pdf_cell, deadline_cell) =
                            row.children().take(4).collect_tuple().ok_or(ParsingError)?;

                        Ok(Problem {
                            code: code_cell.text(),
                            name: name_cell.text(),
                            deadline: deadline_cell.text().trim().to_owned(),
                            id: name_cell
                                .children()
                                .next()
                                .and_then(|x| x.get("href"))
                                .and_then(|x| x.split("/").last().map(|x| x.to_owned()))
                                .ok_or(ParsingError)?,
                        })
                    }),
            ))
        }))
    }

    pub async fn get_contests(
        &mut self,
    ) -> Result<impl Iterator<Item = ParsingResult<Contest>>, Box<dyn std::error::Error>> {
        let html = self
            .get("/contest/select?participating_limit=0&participating_filter_archived=1")
            .await?
            .text()
            .await?;

        let soup = Soup::new(&html);
        let table = soup
            .tag("table")
            .class("results")
            .find()
            .ok_or(ParsingError)?;

        Ok(table
            .tag("tr")
            .find_all()
            .skip(1)
            .map(|row| -> ParsingResult<Contest> {
                let (name_cell, description_cell) = row
                    .tag("td")
                    .find_all()
                    .take(2)
                    .collect_tuple()
                    .ok_or(ParsingError)?;
                Ok(Contest {
                    name: name_cell.text(),
                    description: description_cell.text(),
                    id: name_cell
                        .children()
                        .next()
                        .and_then(|x| x.get("href"))
                        .and_then(|x| x.split("/").nth(2).map(|x| x.to_owned()))
                        .ok_or(ParsingError)?,
                })
            }))
    }
    pub async fn get_results(
        &mut self,
        contest_id: &str,
    ) -> Result<impl Iterator<Item = ParsingResult<Submit>>, Box<dyn std::error::Error>> {
        let html = self
            .get(format!("/contest/{}/results?results_limit=100000", contest_id).as_str())
            .await?
            .text()
            .await?;
        let soup = Soup::new(&html);
        let table = soup
            .tag("table")
            .class("results")
            .find()
            .ok_or(ParsingError)?;
        Ok(table
            .tag("tr")
            .find_all()
            .skip(1)
            .map(|row| -> ParsingResult<Submit> {
                let (id_cell, problem_cell, time_cell, status_cell) =
                    row.children().take(4).collect_tuple().ok_or(ParsingError)?;
                Ok(Submit {
                    id: id_cell.text(),
                    problem: problem_cell.text(),
                    time: time_cell.text(),
                    status: status_cell.text(),
                })
            }))
    }
}
type ParsingResult<T> = Result<T, ParsingError>;

#[derive(Debug, Clone)]
pub struct ParsingError;

impl std::error::Error for ParsingError {}

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Could not parse Satori HTML, the website layout may have changed."
        )
    }
}

async fn update_token(
    url: &str,
    token_file: &Path,
    login: &str,
    password: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let token = get_token(url.to_owned(), login.to_owned(), password.to_owned()).await?;
    std::fs::write(token_file, &token)?;
    Ok(token)
}

async fn get_token(
    url: String,
    login: String,
    password: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let form = reqwest::multipart::Form::new()
        .text("login", login)
        .text("password", password);
    let response = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?
        .post(url + "/login")
        .multipart(form)
        .send()
        .await?;
    let token = response
        .headers()
        .get("set-cookie")
        .unwrap()
        .to_str()?
        .split(";")
        .next()
        .unwrap();

    Ok(String::from(token))
}
