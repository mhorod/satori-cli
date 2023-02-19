use crate::display::*;
use crate::satori::*;

use console::style;

pub struct SimpleDisplay {}
impl SimpleDisplay {
    pub fn new() -> SimpleDisplay {
        SimpleDisplay {}
    }

    fn print_contests(&self, contests: &Vec<Contest>) {
        for contest in contests {
            print!("[{}] {}", contest.id, style(&contest.name).bold());

            if contest.description != "" {
                println!(" ({})", contest.description);
            } else {
                println!();
            }
        }
    }

    fn print_problems(&self, problems: &Vec<Problem>) {
        for problem in problems {
            if problem.id != "" {
                print!("[{}] ", problem.id);
            }
            println!(
                "{} {}",
                style(&problem.code).bold(),
                problem.name
            );
        }
    }

    fn print_error(&self, error: &SatoriError) {
        match error {
            SatoriError::NotLoggedIn => {
                println!("You are not logged in.");
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
                    println!("- {}", candidate);
                }
            }
            SatoriError::AmbiguousProblem(ambiguous_name_error) => {
                println!(
                    "Problem name '{}' is ambiguous. Candidates are:",
                    ambiguous_name_error.name
                );
                for candidate in &ambiguous_name_error.candidates {
                    println!("- {}", candidate);
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
        }
    }
}

impl SatoriDisplay for SimpleDisplay {
    fn display_contests(&self, contests: &SatoriResult<Vec<Contest>>) {
        match contests {
            Ok(contests) => {
                self.print_contests(contests);
            }
            Err(error) => self.print_error(error)
        }
    }

    fn display_details(&self, details: &SatoriResult<ResultDetails>) {
        println!("Details: {:?}", details);
    }

    fn display_logout(&self, logout: &SatoriResult<()>) {
        println!("Logout: {:?}", logout);
    }

    fn display_problems(&self, problems: &SatoriResult<Vec<Problem>>) {
        match problems {
            Ok(problems) => {
                self.print_problems(problems);
            }
            Err(error) => self.print_error(error)
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
}
