extern crate reqwest;
extern crate tokio;

const URL: &str = "https://satori.tcs.uj.edu.pl";
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let login = std::fs::read_to_string(".satori/login")?;
    let password = std::fs::read_to_string(".satori/password")?;
    let token = log_in(UserData {
        login: login,
        password: password,
    })
    .await?;

    std::fs::write(".satori/token", &token)?;
    auth_get(String::from("/contest/select"), token).await?;
    Ok(())
}

#[derive(serde::Serialize)]
struct UserData {
    login: String,
    password: String,
}

async fn log_in(user: UserData) -> Result<String, Box<dyn std::error::Error>> {
    let body = format!("login={}&password={}", user.login, user.password);
    let response = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?
        .post(URL.to_owned() + "/login")
        .body(body)
        .send()
        .await?;
    let token = response
        .headers()
        .get("Set-Cookie")
        .unwrap()
        .to_str()?
        .split(";")
        .next()
        .unwrap();
    Ok(String::from(token))
}

// TODO: post
// async fn auth_post(path: String, body: String) -> Result<(), Box<dyn std::error::Error>> {
//     Ok(())
// }

async fn auth_get(
    path: String,
    token_cookie: String,
) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Cookie",
        reqwest::header::HeaderValue::from_str(token_cookie.as_str()).unwrap(),
    );

    let response = reqwest::Client::builder()
        .default_headers(headers)
        .build()?
        .get(URL.to_owned() + &path)
        .send()
        .await?;
    Ok(response)
}
