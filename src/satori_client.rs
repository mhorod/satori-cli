use lazy_static::lazy_static;
use regex::Regex;

use std::path::{Path, PathBuf};

use crate::satori::*;
use crate::simple_html_parser::*;

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
            Ok(token) => token,
            Err(_) => update_token(url, token_file, login, password).await?,
        };
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
    async fn get(&self, address: &str) -> Result<reqwest::Response, reqwest::Error> {
        self.client.get(self.url.clone() + address).send().await
    }

    pub async fn get_problems(
        &mut self,
        contest_id: String,
    ) -> Result<Vec<Problem>, Box<dyn std::error::Error>> {
        let mut html = self
            .get(format!("/contest/{}/problems", contest_id).as_str())
            .await?
            .text()
            .await?;

        if !html.contains("Sign out") {
            self.update_token().await?;
            html = self
                .get(format!("/contest/{}/problems", contest_id).as_str())
                .await?
                .text()
                .await?;
        }

        let mut result: Vec<Problem> = vec![];
        SimpleHTMLParser::new(html.as_str())
            .next("table")
            .on_next("table", |_, content| {
                SimpleHTMLParser::new(content)
                    .next("tr")
                    .on_all("tr", |_, content| {
                        let mut code = String::new();
                        let mut id = String::new();
                        let mut name = String::new();
                        let mut deadline = String::new();
                        SimpleHTMLParser::new(content)
                            .on_next("td", |_, content| code = content.to_owned())
                            .unwrap()
                            .on_next("a", |args, content| {
                                name = content.to_owned();
                                id = args
                                    .get(&"href".to_owned())
                                    .unwrap()
                                    .split("/")
                                    .last()
                                    .unwrap()
                                    .to_owned();
                            })
                            .unwrap()
                            .on_next("p", |_, content| deadline = content.to_owned());
                        result.push(Problem {
                            id,
                            code,
                            name,
                            deadline,
                        });
                    });
            });

        lazy_static! {
            static ref REG: Regex = {
                Regex::new(
                    r#"(?s)(?x)
                    <td[^<>]*?>(?P<code>[^<>]*?)</td>[ \t]*
                    <td[^<>]*?>[ \t]*
                    <a[^<>]*?href="/contest/[0-9]*/problems/(?P<id>[0-9]*)">
                    (?P<name>.*?)</a>.*?
                    <p>(?P<deadline>.*?)</p>
                    "#,
                )
                .unwrap()
            };
        }
        /*Ok(REG
        .captures_iter(html.as_str())
        .filter_map(|cap| {
            match (
                cap.name("code"),
                cap.name("id"),
                cap.name("name"),
                cap.name("deadline"),
            ) {
                (Some(code), Some(id), Some(name), Some(deadline)) => Some(Problem {
                    id: id.as_str().to_string(),
                    code: code.as_str().to_string(),
                    name: name.as_str().to_string(),
                    deadline: deadline.as_str().to_string(),
                }),
                _ => None,
            }
        })
        .collect());
        */
        Ok(result)
    }

    pub async fn get_contests(&mut self) -> Result<Vec<Contest>, Box<dyn std::error::Error>> {
        let mut html = self.get("/contest/select").await?.text().await?;

        if !html.contains("Sign out") {
            self.update_token().await?;
            html = self.get("/contest/select").await?.text().await?;
        }

        lazy_static! {
            static ref REG: Regex = {
                Regex::new(
                    r#"(?s)(?x)
                    <a.class="stdlink".href="/contest/(?P<id>[0-9]*?)/">
                    (?P<name>.*?)
                    </a>
                    [ \t]*?</td>[ \t]*?
                    <td.class="description">(?P<description>.*?)</td>
                    "#,
                )
                .unwrap()
            };
        }

        return Ok(REG
            .captures_iter(html.as_str())
            .filter_map(
                |cap| match (cap.name("id"), cap.name("name"), cap.name("description")) {
                    (Some(id), Some(name), Some(description)) => Some(Contest {
                        id: id.as_str().to_string(),
                        name: name.as_str().to_string(),
                        description: description.as_str().to_string(),
                    }),
                    _ => None,
                },
            )
            .collect());
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
