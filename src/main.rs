extern crate clap;
extern crate lazy_static;
extern crate regex;
extern crate reqwest;
extern crate tokio;

mod satori;
mod satori_client;
mod util;

use satori_client::SatoriClient;

use std::path::Path;

use std::io;

use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use util::event::{Event, Events};

use tui::{
    backend::TermionBackend,
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};

const URL: &str = "https://satori.tcs.uj.edu.pl";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = create_cli_app();
    let matches = app.get_matches();

    match matches.subcommand() {
        ("new", Some(new_matches)) => {
            new_problem(new_matches.value_of("problem").unwrap().to_owned())
        }
        ("tui", _) => start_tui().await?,
        ("get-contests", _) => get_contests().await?,
        ("get-problems", Some(m)) => {
            get_problems(m.value_of("contest-id").unwrap().to_owned()).await?
        }
        _ => {}
    }

    Ok(())
}

fn create_cli_app() -> clap::App<'static, 'static> {
    let clone_subcommand = clap::SubCommand::with_name("new")
        .arg(
            clap::Arg::with_name("problem")
                .help("Problem code")
                .required(true),
        )
        .about("Create a new directory for a problem");
    let tui_subcommand =
        clap::SubCommand::with_name("tui").about("Start interactive terminal application");

    let get_contests_subcommand = clap::SubCommand::with_name("get-contests")
        .about("Get all contests that current user participates in");

    let get_problems_subcommand = clap::SubCommand::with_name("get-problems")
        .about("Get problems inside a contest")
        .arg(
            clap::Arg::with_name("contest-id")
                .help("Contest ID")
                .required(true),
        );

    let app = clap::App::new("satori")
        .version("1.0")
        .about("CLI for Satori testing system on TCS, Jagiellonian University")
        .subcommand(clone_subcommand)
        .subcommand(tui_subcommand)
        .subcommand(get_contests_subcommand)
        .subcommand(get_problems_subcommand);

    return app;
}

async fn start_tui() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = create_client_from_saved_credentials().await?;

    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    let contests = client
        .get_contests()
        .await?
        .collect::<Result<Vec<_>, _>>()?;

    loop {
        terminal.draw(|f| {
            let block = Block::default();
            f.render_widget(block, f.size());
            let items: Vec<ListItem> = contests
                .iter()
                .map(|c| ListItem::new(format!("{:?}", c)))
                .collect();

            let items =
                List::new(items).block(Block::default().borders(Borders::ALL).title("Contests"));
            f.render_widget(items, f.size());
        })?;

        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                _ => {}
            },
            Event::Tick => {}
        }
    }

    Ok(())
}

async fn create_client_from_saved_credentials() -> Result<SatoriClient, Box<dyn std::error::Error>>
{
    let login = std::fs::read_to_string(".satori/login")?.trim().to_owned();
    let password = std::fs::read_to_string(".satori/password")?
        .trim()
        .to_owned();

    let client = SatoriClient::new(URL, Path::new(".satori/token"), &login, &password).await?;
    Ok(client)
}

fn new_problem(name: String) {
    println!("Created new problem directory {}", name);
}

async fn get_contests() -> Result<(), Box<dyn std::error::Error>> {
    println!("Loading contests...");
    let mut client = create_client_from_saved_credentials().await?;
    for contest in client.get_contests().await? {
        println!("{:?}", contest.unwrap());
    }
    Ok(())
}

async fn get_problems(contest_id: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("Loading problems...");
    let mut client = create_client_from_saved_credentials().await?;
    client
        .get_problems(contest_id.as_str())
        .await?
        .for_each(|p| {
            let (title, group) = p.unwrap();
            println!("Group: {}", title);
            group.for_each(|p| println!("{:?}", p.unwrap()))
        });
    Ok(())
}
