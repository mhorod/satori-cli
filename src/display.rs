use crate::satori::*;

pub trait SatoriDisplay {
    fn display_contests(&self, contests: &Option<Vec<Contest>>);
    fn display_details(&self, details: &Option<ResultDetails>);
    fn display_logout(&self, logout: &Option<()>);
    fn display_problems(&self, problems: &Option<Vec<Problem>>);
    fn display_pdf(&self, pdf: &Option<()>);
    fn display_results(&self, results: &Option<Vec<ShortResult>>);
    fn display_status(&self, status: &Option<String>);
    fn display_submit(&self, submit: &Option<()>);
}