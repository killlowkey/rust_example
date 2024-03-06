use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use anyhow::{anyhow, Result};
use crate::abi::pb::PbPerson;

#[derive(Debug)]
pub struct CustomError {
    message: String,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.message)
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

type Err = anyhow::Error;

impl PersonService {
    pub fn new() -> PersonService {
        return PersonService {
            data: HashMap::new()
        };
    }

    // 跳过格式化
    #[rustfmt::skip]
    pub fn add(&mut self, person: PbPerson) -> Result<(), Box<dyn Error>> {
        if person.age > 100 {
            return Err(Box::new(CustomError::new("年龄大于100".into())));
        }

        self.data.insert(person.name.clone(), person);
        Ok(())
    }

    pub fn remove(&mut self, name: &str) -> Result<()> {
        // 为了返回错误，而返回错误
        if name.len() > 100 {
            return Err(anyhow!(format!("key length 不能大于 100，当前长度为 {}", name.len())));
        }

        match self.data.remove(name) {
            None => { Err(anyhow!(format!("remove {} failed", name))) }
            Some(_) => { Ok(()) }
        }
    }

    pub fn get(&mut self, name: &str) -> Option<&PbPerson> {
        self.data.get(name)
    }

    // pub fn get_mut(&mut self, name: &str) -> Option<&mut PbPerson> {
    //     (&mut self.data.get(name)).as_deref_mut()
    // }

    pub fn list(&mut self) -> Option<Vec<&PbPerson>> {
        let mut res: Vec<&PbPerson> = vec![];
        for (_, p) in self.data.iter() {
            res.push(p);
        }

        Some(res)
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}