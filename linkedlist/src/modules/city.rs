use std::fmt;

#[derive(Clone)]
pub struct City {
    pub name: String,
    pub state: String,
    pub population: i64,
    pub user_number: i64
}

impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\nThe city's name is {} and it is in {} state.\n{} people are living in {}. {}'s user number is {}.",
        self.name,
        self.state,
        self.population,
        self.name,
        self.name,
        self.user_number)
    }
}

impl City {
    pub fn create(name: String, state: String, population: i64, user_number: i64) -> City {
        City{
            name,
            state,
            population,
            user_number,
        }
    }
}