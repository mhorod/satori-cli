mod cli;
mod mock;
mod satori;
mod satori_client;
mod reqwest_satori_client;
mod token;
mod display;

use crate::satori::Satori;
use crate::display::SatoriDisplay;

use std::io::Write;

use soup::prelude::*;

const URL: &str = "https://satori.tcs.uj.edu.pl";
const TOKEN_NAME: &str = "satori_token";

fn main() {
    let client = reqwest_satori_client::ReqwestSatoriClient::new(URL, TOKEN_NAME);
    let satori = mock::MockSatori::new();
    let display = mock::MockDisplay::new();
    run_app(satori, display);
}

fn run_app(satori: impl Satori, display: impl SatoriDisplay) {
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


fn ask_for_credentials() -> (String, String) {
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
    soup.tag("div")
        .attr("id", "header")
        .find()?
        .tag("ul")
        .attr("class", "headerRightUl")
        .find()?
        .tag("li")
        .find()
        .map(|li| li.text().trim().to_string())
        .and_then(|username| match username.as_str() {
            "Register" => None,
            _ => Some(username),
        })
}
