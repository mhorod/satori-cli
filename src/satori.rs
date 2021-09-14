#[derive(Debug)]
pub struct Contest {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug)]
pub struct Problem {
    pub id: String,
    pub code: String,
    pub name: String,
    pub deadline: String,
}

#[derive(Debug)]
pub struct Submit {
    pub id: String,
    pub problem_code: String,
    pub time: String,
    pub status: String,
}
