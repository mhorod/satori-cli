use crate::display::*;
use crate::satori::*;

use console::style;

pub struct SimpleDisplay {}
impl SimpleDisplay {
    pub fn new() -> SimpleDisplay {
        SimpleDisplay {}
    }
    fn print_error(&self, error: &SatoriError) {
        match error {
            SatoriError::NotLoggedIn => {
                println!("You are not logged in.");
            }
            SatoriError::LoginFailed => {
                println!("Login failed.");
            }
            SatoriError::ParsingFailed => {
                println!("Parsing failed.");
            }
            SatoriError::ConnectionFailed => {
                println!("Connection failed.");
            }
            SatoriError::AmbiguousContest(ambiguous_name_error) => {
                println!(
                    "Contest name '{}' is ambiguous. Candidates are:",
                    ambiguous_name_error.name
                );
                for candidate in &ambiguous_name_error.candidates {
                    println!("- {}", candidate.name);
                }
            }
            SatoriError::AmbiguousProblem(ambiguous_name_error) => {
                println!(
                    "Problem name '{}' is ambiguous. Candidates are:",
                    ambiguous_name_error.name
                );
                for candidate in &ambiguous_name_error.candidates {
                    println!("- {} {}", style(&candidate.code).bold(), candidate.name);
                }
            }
            SatoriError::ContestNotFound => {
                println!("Contest not found.");
            }
            SatoriError::ProblemNotFound => {
                println!("Problem not found.");
            }
            SatoriError::SubmissionNotFound => {
                println!("Submission not found.");
            }

            SatoriError::InvalidChoice => {
                println!("Invalid choice.");
            }

            _ => {
                println!("Error: {:?}", error);
            }
        }
    }
}

macro_rules! handle_error {
    ($self:ident, $value:ident) => {
        if let Err(error) = $value {
            $self.print_error(&error);
            return;
        }
        let $value = $value.as_ref().unwrap();
    };
}

impl SatoriDisplay for SimpleDisplay {
    fn display_username(&self, username: &SatoriResult<String>) {
        handle_error!(self, username);
        println!("Logged in as {}.", style(username).bold());
    }

    fn display_contests(&self, contests: &SatoriResult<Vec<Contest>>) {
        handle_error!(self, contests);
        for contest in contests {
            print!("[{}] {}", contest.id, style(&contest.name).bold());

            if contest.description != "" {
                println!(" ({})", contest.description);
            } else {
                println!();
            }
        }
    }

    fn display_details(&self, details: &SatoriResult<ResultDetails>) {
        println!("Details: {:?}", details);
    }

    fn display_login(&self, login: &SatoriResult<String>) {
        handle_error!(self, login);
        println!("Logged in as {}.", style(login).bold());
    }

    fn display_logout(&self, logout: &SatoriResult<()>) {
        handle_error!(self, logout);
        println!("Logged out.");
    }

    fn display_problems(&self, problems: &SatoriResult<Vec<Problem>>) {
        handle_error!(self, problems);
        for problem in problems {
            if problem.id != "" {
                print!("[{}] ", problem.id);
            }
            println!("{} {}", style(&problem.code).bold(), problem.name);
        }
    }

    fn display_pdf(&self, pdf: &SatoriResult<()>) {
        println!("PDF: {:?}", pdf);
    }

    fn display_results(&self, results: &SatoriResult<Vec<ShortResult>>) {
        println!("Results: {:?}", results);
    }

    fn display_status(&self, status: &SatoriResult<String>) {
        println!("Status: {:?}", status);
    }

    fn display_submit(&self, submit: &SatoriResult<()>) {
        println!("Submit: {:?}", submit);
    }

    fn display_error(&self, error: &SatoriError) {
        self.print_error(error);
    }
}
