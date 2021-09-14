extern crate lazy_static;
extern crate regex;
extern crate reqwest;
extern crate tokio;

mod satori;
mod satori_client;
mod util;

use itertools::Itertools;
use satori_client::SatoriClient;

use std::path::Path;

use std::io;


use util::event::{Event, Events};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};

use tui::{
    backend::TermionBackend,
    layout::{Constraint, Corner, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};



const URL: &str = "https://satori.tcs.uj.edu.pl";


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut client = create_client_from_saved_credentials().await?;

    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    
    let events = Events::new();

    let contests = client.get_contests().await?.collect::<Result<Vec<_>,_>>()?;

    loop {
        terminal.draw(|f| {
            let block = Block::default();
            f.render_widget(block, f.size());
            let items: Vec<ListItem> =
                contests
                .iter().map(|c| { ListItem::new(format!("{:?}", c)) }).collect();

            let items = List::new(items).block(Block::default().borders(Borders::ALL).title("Contests"));
            f.render_widget(items, f.size());
                
        })?;

        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => { break; }
            _ => {}
            }
            Event::Tick => {}
        } 
    }

    Ok(())
}


async fn create_client_from_saved_credentials() -> Result<SatoriClient, Box<dyn std::error::Error>> {

    let login = std::fs::read_to_string(".satori/login")?.trim().to_owned();
    let password = std::fs::read_to_string(".satori/password")?
        .trim()
        .to_owned();
    
    let client = SatoriClient::new(URL, Path::new(".satori/token"), &login, &password).await?;
    Ok(client)
}
