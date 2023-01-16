use std::io::Write;

const URL: &str = "https://satori.tcs.uj.edu.pl";


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cookie_store = reqwest_cookie_store::CookieStore::default();
    let cookie_store = reqwest_cookie_store::CookieStoreMutex::new(cookie_store);
    let cookie_store = std::sync::Arc::new(cookie_store);

    let client = reqwest::Client::builder()
        .cookie_provider(std::sync::Arc::clone(&cookie_store))
        .build()?;


    let (login, password) = ask_for_credentials().await;
    log_in(&client, &login, &password).await?;

    for cookie in cookie_store.lock().unwrap().iter_any() {
        println!("{:?}", cookie);
    }

    let res = client.get(URL).send().await?;
    let body = res.text().await?;
    println!("{}", body);

    for cookie in cookie_store.lock().unwrap().iter_any() {
        println!("{:?}", cookie);
    }

    Ok(())
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

async fn log_in(client: &reqwest::Client, login: &str, password: &str) -> reqwest::Result<()> {
    let res = client
        .post("https://satori.tcs.uj.edu.pl/login")
        .form(&[("login", login), ("password", password)])
        .send().await?;
    Ok(())
}
