use crate::display::*;
use crate::satori::*;

use console::style;

pub struct SimpleDisplay {}
impl SimpleDisplay {
    pub fn new() -> SimpleDisplay {
        SimpleDisplay {}
    }
}

impl SatoriDisplay for SimpleDisplay {
    fn display_contests(&self, contests: &Option<Vec<Contest>>) {
        if contests.is_none() {
            println!("No contests found");
            return;
        }
        for contest in contests.as_ref().unwrap() {
            print!("[{}] {}", contest.id, style(&contest.name).bold());

            if contest.description != "" {
                println!(" ({})", contest.description);
            } else {
                println!();
            }
        }
    }

    fn display_details(&self, details: &Option<ResultDetails>) {
        println!("Details: {:?}", details);
    }

    fn display_logout(&self, logout: &Option<()>) {
        println!("Logout: {:?}", logout);
    }

    fn display_problems(&self, problems: &Option<Vec<Problem>>) {
        if problems.is_none() {
            println!("No problems found");
            return;
        }

        for problem in problems.as_ref().unwrap() {
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

    fn display_pdf(&self, pdf: &Option<()>) {
        println!("PDF: {:?}", pdf);
    }

    fn display_results(&self, results: &Option<Vec<ShortResult>>) {
        println!("Results: {:?}", results);
    }

    fn display_status(&self, status: &Option<String>) {
        println!("Status: {:?}", status);
    }

    fn display_submit(&self, submit: &Option<()>) {
        println!("Submit: {:?}", submit);
    }
}
