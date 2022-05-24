use crate::pets::Greeting;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Dog {
    name: String,
}

impl Dog {
    pub fn new(name: &str) -> Dog {
        Dog {
            name: String::from(name),
        }
    }

    pub fn name(&self) -> &str {
        &self.name[..]
    }
}


impl Greeting for Dog {
    fn greet(&self) -> String {
        String::from("Gaw Gaw")    
    }
}
