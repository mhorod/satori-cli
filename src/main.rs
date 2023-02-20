mod cli;
mod display;
mod file_token_storage;
mod interactive_satori;
mod parser;
mod prompt;
mod reqwest_satori_client;
mod satori;
mod satori_client;
mod simple_display;
mod simple_satori;
mod soup_parser;
mod token_storage;

use crate::display::SatoriDisplay;
use crate::satori::Satori;

use std::io::Write;

const URL: &str = "https://satori.tcs.uj.edu.pl";
const TOKEN_NAME: &str = "satori_token";

struct SimplePrompt {}
impl SimplePrompt {
    pub fn new() -> SimplePrompt {
        SimplePrompt {}
    }
}
impl prompt::Prompt for SimplePrompt {
    fn ask_for_credentials(&self) -> Option<(String, String)> {
        let mut login = String::new();

        print!("Login: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut login).unwrap();
        login.pop(); // remove newline
        let password = rpassword::prompt_password("Password: ").unwrap();

        Some((login, password))
    }

    fn choose_option(&self, message: &str, options: &Vec<String>) -> Option<usize> {
        println!("{}", message);
        for (i, option) in options.iter().enumerate() {
            println!("{}. {}", i + 1, option);
        }

        let mut choice = String::new();
        print!("Your choice: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut choice).unwrap();
        choice.pop(); // remove newline

        match choice.parse::<usize>() {
            Ok(choice) => {
                if choice > 0 && choice <= options.len() {
                    Some(choice - 1)
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }
}

fn main() {
    let client = reqwest_satori_client::ReqwestSatoriClient::new(URL, TOKEN_NAME);
    let parser = soup_parser::SoupParser::new();
    let token_storage = file_token_storage::FileTokenStorage::default();
    let satori = simple_satori::SimpleSatori::new(client, parser, token_storage);
    let display = simple_display::SimpleDisplay::new();
    let prompt = SimplePrompt::new();

    let satori = interactive_satori::InteractiveSatori::new(satori, display, prompt);
    run_app(satori);
}

fn run_app(satori: impl Satori) {
    println!("Satori is fucking slow, please be patient. I can't do anything about it :(");
    match cli::build_cli().get_matches().subcommand() {
        Some((cmd, args)) => match cmd {
            "username" => do_username(satori, args),
            "contests" => do_contests(satori, args),
            "details" => do_details(satori, args),
            "logout" => do_logout(satori, args),
            "problems" => do_problems(satori, args),
            "pdf" => do_pdf(satori, args),
            "results" => do_results(satori, args),
            "status" => do_status(satori, args),
            "submit" => do_submit(satori, args),
            _ => println!("Unknown command"),
        },

        _ => println!("Oops, something went terribly wrong."),
    }
}

#[allow(unused)]
fn do_username(satori: impl Satori, args: &clap::ArgMatches) {
    satori.username();
}

#[allow(unused)]
fn do_contests(satori: impl Satori, args: &clap::ArgMatches) {
    let archived = args.get_flag("archived");
    let force = args.get_flag("force");

    satori.contests(archived, force);
}

#[allow(unused)]
fn do_details(satori: impl Satori, args: &clap::ArgMatches) {
    let contest = args.get_one::<String>("contest").unwrap();
    let problem = args.get_one::<String>("problem").unwrap();
    let submission = args.get_one::<String>("submission").unwrap();
    let force = args.get_flag("force");

    satori.details(contest, problem, submission, force);
}

#[allow(unused)]
fn do_logout(satori: impl Satori, _args: &clap::ArgMatches) {
    satori.logout();
}

#[allow(unused)]
fn do_problems(satori: impl Satori, args: &clap::ArgMatches) {
    let contest = args.get_one::<String>("contest").unwrap();
    let force = args.get_flag("force");

    satori.problems(contest, force);
}

#[allow(unused)]
fn do_pdf(satori: impl Satori, args: &clap::ArgMatches) {
    let contest = args.get_one::<String>("contest").unwrap();
    let problem = args.get_one::<String>("problem").unwrap();
    let force = args.get_flag("force");

    satori.pdf(contest, problem, force);
}

#[allow(unused)]
fn do_results(satori: impl Satori, args: &clap::ArgMatches) {
    let contest = args.get_one::<String>("contest").unwrap();
    let problem = args.get_one::<String>("problem").unwrap();
    let force = args.get_flag("force");

    satori.results(contest, problem, force);
}

#[allow(unused)]
fn do_status(satori: impl Satori, args: &clap::ArgMatches) {
    let contest = args.get_one::<String>("contest").unwrap();
    let problem = args.get_one::<String>("problem").unwrap();
    let force = args.get_flag("force");

    satori.status(contest, problem, force);
}

#[allow(unused)]
fn do_submit(satori: impl Satori, args: &clap::ArgMatches) {
    let contest = args.get_one::<String>("contest").unwrap();
    let problem = args.get_one::<String>("problem").unwrap();
    let file = args.get_one::<String>("file").unwrap();

    satori.submit(contest, problem, file);
}
