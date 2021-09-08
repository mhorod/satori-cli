extern crate lazy_static;
extern crate regex;
extern crate reqwest;
extern crate tokio;

mod satori;
mod satori_client;
mod simple_html_parser;

use satori_client::SatoriClient;

use std::path::Path;

const URL: &str = "https://satori.tcs.uj.edu.pl";
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let login = std::fs::read_to_string(".satori/login")?.trim().to_owned();
    let password = std::fs::read_to_string(".satori/password")?
        .trim()
        .to_owned();
    let mut client = SatoriClient::new(URL, Path::new(".satori/token"), &login, &password).await?;
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
