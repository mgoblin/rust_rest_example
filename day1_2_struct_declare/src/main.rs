use std::fmt::{Display, self};

struct SimplePerson {
    first_name: String,
    last_name: String,
    age: u8,
}

impl SimplePerson {

    fn descr() -> String {
        "Peson is a man, woman, or child.".to_string()
    }

    fn name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

trait Greeter {
    fn greeting(&self, greet: &str) -> String;
}

impl Greeter for SimplePerson {
    fn greeting(&self, greet: &str) -> String {
        format!("{} {}", greet,  self.first_name)
    }
}

struct DisplayPerson {
    first_name: String,
    last_name: String,
    age: u8,
}

impl Display for DisplayPerson {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Person with Display trait (first_name: {}, last_name: {}, age: {})",
               self.first_name, self.last_name, self.age)
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct DebugPerson {
  first_name: String,
  last_name: String,
  age: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Person {
  first_name: String,
  last_name: String,
  age: u8,
}

fn main() {
    let smike = SimplePerson {
        first_name: "Mike".to_string(),
        last_name: "Smith".to_string(),
        age: 45
    };
    println!("Person: {}, {}, {}", smike.first_name, smike.last_name, smike.age);
    println!("Person type description: {}", SimplePerson::descr());
    println!("Person name is {}", smike.name());
    println!("{}", smike.greeting("Hello"));

    let john = SimplePerson {
        first_name: String::from("John"),
        ..smike
    };

    println!("Person: {}, {}, {}", john.first_name, john.last_name, john.age);

    let display_mike = DisplayPerson {
        first_name: "Mike".to_string(),
        last_name: "Smith".to_string(),
        age: 45,
    };
    println!("{}", display_mike);

    let debug_mike = DebugPerson {
        first_name: "Mike".to_string(),
        last_name: "Smith".to_string(),
        age: 45,
    };
    println!("{:#?}", debug_mike);

    let mike = Person {
        first_name: "Mike".to_string(),
        last_name: "Smith".to_string(),
        age: 45
    };
    println!("{:#?}", mike);

    let mike2 = mike.clone();
    println!("{:#?}", mike2);

    let mike3 = Person {
        first_name: String::from("Mike"),
        last_name: String::from("Smith"),
        age: 45,
      };
      
      // Symmetry
      println!("{}", mike == mike2);
      println!("{}", mike2 == mike);
      
      // Transitivity
      println!("{}", mike == mike3);
      println!("{}", mike2 == mike3);
      println!("{}", mike == mike2); 
}
