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
}

fn main() {
    let client = reqwest_satori_client::ReqwestSatoriClient::new(URL, TOKEN_NAME);
    let parser = soup_parser::SoupParser::new();
    let token_storage = file_token_storage::FileTokenStorage::default();
    let satori = simple_satori::SimpleSatori::new(client, parser, token_storage);
    let display = simple_display::SimpleDisplay::new();
    run_app(satori, display);
}

fn run_app(satori: impl Satori, display: impl SatoriDisplay) {
    println!("Satori is fucking slow, please be patient. I can't do anything about it :(");
    match cli::build_cli().get_matches().subcommand() {
        Some((cmd, args)) => match cmd {
            "contests" => do_contests(satori, display, args),
            "details" => do_details(satori, display, args),
            "logout" => do_logout(satori, display, args),
            "problems" => do_problems(satori, display, args),
            "pdf" => do_pdf(satori, display, args),
            "results" => do_results(satori, display, args),
            "status" => do_status(satori, display, args),
            "submit" => do_submit(satori, display, args),
            _ => println!("Unknown command"),
        },

        _ => println!("Oops, something went terribly wrong."),
    }
}

fn do_contests(satori: impl Satori, display: impl SatoriDisplay, args: &clap::ArgMatches) {
    let archived = args.get_flag("archived");
    let force = args.get_flag("force");

    let contests = satori.contests(archived, force);
    display.display_contests(&contests);
}

fn do_details(satori: impl Satori, display: impl SatoriDisplay, args: &clap::ArgMatches) {
    let contest = args.get_one::<String>("contest").unwrap();
    let problem = args.get_one::<String>("problem").unwrap();
    let submission = args.get_one::<String>("submission").unwrap();
    let force = args.get_flag("force");

    display.display_details(&satori.details(contest, problem, submission, force));
}

fn do_logout(satori: impl Satori, display: impl SatoriDisplay, _args: &clap::ArgMatches) {
    display.display_logout(&satori.logout());
}

fn do_problems(satori: impl Satori, display: impl SatoriDisplay, args: &clap::ArgMatches) {
    let contest = args.get_one::<String>("contest").unwrap();
    let force = args.get_flag("force");

    display.display_problems(&satori.problems(contest, force));
}

fn do_pdf(satori: impl Satori, display: impl SatoriDisplay, args: &clap::ArgMatches) {
    let contest = args.get_one::<String>("contest").unwrap();
    let problem = args.get_one::<String>("problem").unwrap();
    let force = args.get_flag("force");

    display.display_pdf(&satori.pdf(contest, problem, force));
}

fn do_results(satori: impl Satori, display: impl SatoriDisplay, args: &clap::ArgMatches) {
    let contest = args.get_one::<String>("contest").unwrap();
    let problem = args.get_one::<String>("problem").unwrap();
    let force = args.get_flag("force");

    display.display_results(&satori.results(contest, problem, force));
}
fn do_status(satori: impl Satori, display: impl SatoriDisplay, args: &clap::ArgMatches) {
    let contest = args.get_one::<String>("contest").unwrap();
    let problem = args.get_one::<String>("problem").unwrap();
    let force = args.get_flag("force");

    display.display_status(&satori.status(contest, problem, force));
}

fn do_submit(satori: impl Satori, display: impl SatoriDisplay, args: &clap::ArgMatches) {
    let contest = args.get_one::<String>("contest").unwrap();
    let problem = args.get_one::<String>("problem").unwrap();
    let file = args.get_one::<String>("file").unwrap();

    display.display_submit(&satori.submit(contest, problem, file));
}
