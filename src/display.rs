use crate::satori::*;

pub trait SatoriDisplay {
    fn display_username(&self, username: &SatoriResult<String>);
    fn display_contests(&self, contests: &SatoriResult<Vec<Contest>>);
    fn display_details(&self, details: &SatoriResult<ResultDetails>);
    fn display_login(&self, login: &SatoriResult<String>);
    fn display_logout(&self, logout: &SatoriResult<()>);
    fn display_problems(&self, problems: &SatoriResult<Vec<Problem>>);
    fn display_pdf(&self, pdf: &SatoriResult<()>);
    fn display_results(&self, results: &SatoriResult<Vec<ShortResult>>);
    fn display_status(&self, status: &SatoriResult<String>);
    fn display_submit(&self, submit: &SatoriResult<()>);
    fn display_error(&self, error: &SatoriError);
}
