use std::io::Write;

use reqwest_cookie_store::CookieStoreMutex;
use std::sync::Arc;

use std::io::prelude::*;
use soup::prelude::*;

const URL: &str = "https://satori.tcs.uj.edu.pl";
const DOMAIN: &str = "satori.tcs.uj.edu.pl";
const TOKEN_NAME: &str = "satori_token";

struct SatoriClient
{
    base_url: reqwest::Url,
    client: reqwest::Client,
    cookie_store: Arc<CookieStoreMutex>,
}

impl SatoriClient {
    pub fn new(base_url: &str) -> Self {
        let base_url = reqwest::Url::parse(base_url).unwrap();

        let cookie_store = reqwest_cookie_store::CookieStore::default();
        let cookie_store = reqwest_cookie_store::CookieStoreMutex::new(cookie_store);
        let cookie_store = std::sync::Arc::new(cookie_store);

        let client = reqwest::Client::builder()
            .cookie_provider(std::sync::Arc::clone(&cookie_store))
            .build().unwrap();

        Self {
            base_url,
            client,
            cookie_store,
        }
    }

    pub fn get_token(&self) -> Option<String> {
        self.cookie_store
            .lock()
            .unwrap()
            .get(DOMAIN, "/", TOKEN_NAME)
            .map(|cookie| cookie.value().to_string())
    }

    pub fn set_token(&self, token: &str) {
        let cookie = cookie::Cookie::build(TOKEN_NAME, token)
            .domain(DOMAIN)
            .path("/")
            .secure(true)
            .http_only(true)
            .finish();
        self.cookie_store
            .lock()
            .unwrap()
            .insert_raw(&cookie, &self.base_url)
            .unwrap();
    }

    pub async fn log_in(&self, login: &str, password: &str) -> reqwest::Result<()> {
        self.client
            .post(self.get_url("login"))
            .form(&[("login", login), ("password", password)])
            .send().await?;
        Ok(())
    }

    pub fn get_url(&self, path: &str) -> reqwest::Url {
        self.base_url.join(path).unwrap()
    }

    pub async fn do_get(&self, path: &str) -> reqwest::Response {
        self.client.get(self.get_url(path)).send().await.unwrap()
    }

    pub async fn do_post(&self, path: &str, data: &[(&str, &str)]) -> reqwest::Response {
        self.client.post(self.get_url(path)).form(data).send().await.unwrap()
    }

    pub async fn do_multipart_post(&self, path:& str, form: reqwest::multipart::Form) -> reqwest::Response {
        self.client.post(self.get_url(path)).multipart(form).send().await.unwrap()
    }
}

const TOKEN_PATH: &str = "~/.local/share/satori-cli/token.txt";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = SatoriClient::new(URL);

    let token = load_token_from_file(TOKEN_PATH);

    if let Some(token) = token {
        client.set_token(&token);
        println!("Found token {}", &token);
        let username = find_username(&client.do_get("").await.text().await?);
        if let Some(username) = username {
            println!("{}", username);
        }
        else {
            println!("Invalid token");
            let (login, password) = ask_for_credentials().await;
            client.log_in(&login, &password).await?;
        }
    }
    else {
        let (login, password) = ask_for_credentials().await;
        client.log_in(&login, &password).await?;
    }

    let username = find_username(&client.do_get("").await.text().await?);
    if let Some(username) = username {
        println!("{}", username);
    }

    if let Some(token) = client.get_token() {
        println!("Token: {}", token);
        save_token_to_file(TOKEN_PATH, &token);
        println!("Saved token to {}", TOKEN_PATH);
    }
    else {
        println!("No token");
    }
    Ok(())
}


fn load_token_from_file(path: &str) -> Option<String> {
    let path = shellexpand::tilde(path).to_string();
    let file = std::fs::File::open(path).ok()?;
    let mut reader = std::io::BufReader::new(file);
    let mut token = String::new();
    reader.read_line(&mut token).ok()?;
    Some(token)
}

fn save_token_to_file(path: &str, token: &str) {
    let path = shellexpand::tilde(path).to_string();
    let path = std::path::Path::new(&path);
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();

    let mut file = std::fs::File::create(path).unwrap();
    file.write_all(token.as_bytes()).unwrap();
}

async fn ask_for_credentials() -> (String, String) {
    let mut login = String::new();

    print!("Login: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut login).unwrap();
    login.pop(); // remove newline
    let password = rpassword::prompt_password("Password: ").unwrap();

    (login, password)
}

fn find_username(page: &str) -> Option<String> {
    let soup = soup::Soup::new(page);
    soup
        .tag("div")
        .attr("id", "header")
        .find()?
        .tag("ul")
        .attr("class", "headerRightUl")
        .find()?
        .tag("li")
        .find()
        .map(|li| li.text().trim().to_string())
        .and_then(
            |username| {
                match username.as_str() {
                    "Register" => None,
                    _ => Some(username),
                }
            }
        )
}