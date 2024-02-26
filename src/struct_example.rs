use std::fmt;
use std::fmt::{Formatter};

#[derive(Debug)] // println("{:?}", p)
pub(crate) struct Person {
    pub name: Option<String>,
    pub age: Option<u8>,
}

impl fmt::Display for Person {
    // println("{}", p)
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "name={} age={}", self.name.clone().unwrap().as_str(), self.age.unwrap())
    }
}

impl Person {
    pub fn new(name: &str, age: u8) -> Person {
        return Person {
            name: Some(name.to_string()),
            age: Some(age),
        };
    }
    pub fn get_name(&self) -> String {
        match self.name.clone() {
            None => { "".to_string() }
            Some(name) => { name }
        }
    }

    pub fn other_example(&self) {
        if let Some(name) = self.name.clone() {
            println!("my name is {}", name)
        }

        if let Some(age) = self.age {
            println!("my age is {}", age)
        }
    }
}