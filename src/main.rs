extern crate lazy_static;
extern crate regex;
extern crate reqwest;
extern crate tokio;

mod satori;
mod satori_client;

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
    client.get_problems("5084441").await?.for_each(|p| {
        let (title, group) = p.unwrap();
        println!("{}", title);
        group.for_each(|p| {
            println!("{:?}", p.unwrap());
        })
    });
    client.get_contests().await?.for_each(|c| {
        println!("{:?}", c.unwrap());
    });
    client.get_results("5084441").await?.for_each(|c| {
        println!("{:?}", c.unwrap());
    });
    Ok(())
}
