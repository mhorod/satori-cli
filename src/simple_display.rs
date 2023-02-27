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

    fn style_status(status: &str) -> console::StyledObject<&str> {
        match status {
            "OK" => style(status).green(),
            "ANS" => style(status).red(),
            "TLE" => style(status).red(),
            "RTE" => style(status).red(),
            "QUE" => style(status).yellow(),
            _ => style(status).red(),
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
        handle_error!(self, details);
        println!(
            "[{}] {} {}",
            details.submission_id,
            style(&details.problem_code).bold(),
            Self::style_status(&details.status)
        );

        let test_case_len = details
            .test_results
            .iter()
            .map(|r| r.test_case.len())
            .max()
            .unwrap_or(0);

        let status_len = details
            .test_results
            .iter()
            .map(|r| r.status.len())
            .max()
            .unwrap_or(0);

        for result in details.test_results.iter() {
            // align columns
            let test_case = format!("{:>width$}", result.test_case, width = test_case_len);
            let status = format!(
                "{:<width$}",
                Self::style_status(&result.status),
                width = status_len
            );

            println!(
                "{} {} {}",
                test_case,
                Self::style_status(&result.status),
                result.time
            );
        }
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
        handle_error!(self, pdf);
        println!("PDF: {:?}", pdf);
    }

    fn display_results(&self, results: &SatoriResult<Vec<ShortResult>>) {
        handle_error!(self, results);

        let id_len = results
            .iter()
            .map(|r| r.submission_id.len())
            .max()
            .unwrap_or(0);
        let code_len = results
            .iter()
            .map(|r| r.problem_code.len())
            .max()
            .unwrap_or(0);
        let time_len = results.iter().map(|r| r.time.len()).max().unwrap_or(0);
        let status_len = results.iter().map(|r| r.status.len()).max().unwrap_or(0);

        for result in results {
            // align columns
            let id = format!("{:width$}", result.submission_id, width = id_len);
            let code = format!("{:width$}", result.problem_code, width = code_len);
            let time = format!("{:width$}", result.time, width = time_len);
            let status = format!(
                "{:width$}",
                Self::style_status(&result.status),
                width = status_len
            );

            println!("[{}] {} {}", id, style(&code).bold(), status,);
        }
    }

    fn display_status(&self, status: &SatoriResult<String>) {
        handle_error!(self, status);
        println!("Status: {:?}", status);
    }

    fn display_submit(&self, submit: &SatoriResult<()>) {
        handle_error!(self, submit);
        println!("Submit: {:?}", submit);
    }

    fn display_error(&self, error: &SatoriError) {
        self.print_error(error);
    }
}
