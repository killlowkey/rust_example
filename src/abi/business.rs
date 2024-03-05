use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use crate::abi::pb::PbPerson;

#[derive(Debug)]
pub struct CustomError {
    message: String,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl CustomError {
    fn new(message: String) -> CustomError {
        CustomError {
            message
        }
    }
}

impl Error for CustomError {}

pub struct PersonService {
    data: HashMap<String, PbPerson>,
}

impl PersonService {
    pub fn new() -> PersonService {
        return PersonService {
            data: HashMap::new()
        };
    }

    pub fn add_person(&mut self, person: PbPerson) -> Result<(), Box<dyn Error>> {
        if person.age > 100 {
            return Err(Box::new(CustomError::new("年龄大于100".to_string())));
        }

        self.data.insert(person.name.clone(), person);
        Ok(())
    }

    pub fn get_person(&mut self, name: &str) -> Option<&PbPerson> {
        self.data.get(name)
    }
}