extern crate lazy_static;
extern crate regex;
extern crate reqwest;
extern crate tokio;

use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

const URL: &str = "https://satori.tcs.uj.edu.pl";
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let login = std::fs::read_to_string(".satori/login")?.trim().to_owned();
    let password = std::fs::read_to_string(".satori/password")?
        .trim()
        .to_owned();
    let mut client = SatoriClient::new(Path::new(".satori/token"), &login, &password).await?;
    //let contests = client.get_contests().await?;
    client
        .get_problems("5084441".to_owned())
        .await?
        .iter()
        .for_each(|p| {
            println!("{:?}", p);
        });
    Ok(())
}
struct SatoriClient {
    token: String,
    login: String,
    password: String,
    token_file: PathBuf,
    client: reqwest::Client,
}

impl SatoriClient {
    async fn new(
        token_file: &Path,
        login: &str,
        password: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let token = match std::fs::read_to_string(token_file) {
            Ok(token) => token,
            Err(_) => update_token(token_file, login, password).await?,
        };
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Cookie",
            reqwest::header::HeaderValue::from_str(token.as_str()).unwrap(),
        );
        Ok(Self {
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
        self.token = update_token(&self.token_file, &self.login, &self.password).await?;
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
        self.client.get(URL.to_owned() + address).send().await
    }

    async fn get_problems(
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
                            result.push(Problem { id, code, name, deadline});
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

    async fn get_contests(&mut self) -> Result<Vec<Contest>, Box<dyn std::error::Error>> {
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

#[derive(Debug)]
struct Contest {
    id: String,
    name: String,
    description: String,
}

#[derive(Debug)]
struct Problem {
    id: String,
    code: String,
    name: String,
    deadline: String,
}
async fn update_token(
    token_file: &Path,
    login: &str,
    password: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let token = get_token(login.to_owned(), password.to_owned()).await?;
    std::fs::write(token_file, &token)?;
    Ok(token)
}

async fn get_token(login: String, password: String) -> Result<String, Box<dyn std::error::Error>> {
    let form = reqwest::multipart::Form::new()
        .text("login", login)
        .text("password", password);
    let response = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?
        .post(URL.to_owned() + "/login")
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

struct SimpleHTMLParser<'a> {
    content: &'a str,
}

impl<'a> SimpleHTMLParser<'a> {
    fn next(self, tag: &'static str) -> Self {
        let (_, content) = self
            .content
            .split_once(format!("</{}>", tag).as_str())
            .unwrap();
        Self::new(content)
    }
    fn in_next<F>(self, tag: &'static str, mut func: F) -> Option<Self>
    where
        F: FnMut(Self) -> (),
    {
        let (_, content) = match self.content.split_once(format!("<{}", tag).as_str()) {
            Some(val) => val,
            None => return None,
        };
        let (arg_str, content) = content.split_once(">").unwrap();

        //ignoring recursive tags
        let (content, rest) = content.split_once(format!("</{}>", tag).as_str()).unwrap();

        func(Self::new(content));

        Some(Self::new(rest))
    }
    fn on_next<F>(self, tag: &'static str, mut func: F) -> Option<Self>
    where
        F: FnMut(HashMap<String, String>, &str) -> (),
    {
        let (_, content) = match self.content.split_once(format!("<{}", tag).as_str()) {
            Some(val) => val,
            None => return None,
        };
        let (arg_str, content) = content.split_once(">").unwrap();

        //ignoring recursive tags
        let (content, rest) = content.split_once(format!("</{}>", tag).as_str()).unwrap();

        let mut args: HashMap<String, String> = HashMap::new();

        arg_str.trim().split("\" ").for_each(|arg| {
            if arg.len() == 0 {
                return;
            }
            let (key, value) = arg.split_once("=").unwrap();
            let value = if value.ends_with("\"") {
                &value[1..value.len() - 1]
            } else {
                &value[1..value.len()]
            };
            args.insert(key.to_owned(), value.to_owned());
        });

        func(args, content);

        Some(Self::new(rest))
    }
    fn on_all<F>(self, tag: &'static str, mut func: F)
    where
        F: FnMut(HashMap<String, String>, &str) -> (),
    {
        let mut val = self;
        while let Some(x) = val.on_next(tag, |args, content| func(args, content)) {
            val = x;
        }
    }

    fn new(content: &'a str) -> Self {
        SimpleHTMLParser { content }
    }
}

// TODO: post
// async fn auth_post(path: String, body: String) -> Result<(), Box<dyn std::error::Error>> {
//     Ok(())
// }
