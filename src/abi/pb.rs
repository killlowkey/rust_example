#[derive(Debug, Clone, PartialEq)]
pub struct PbPerson {
    pub(crate) name: String,
    pub(crate) age: u8,
}

impl PbPerson {
    pub fn new(name: String, age: u8) -> PbPerson {
        return PbPerson {
            name,
            age,
        };
    }
}