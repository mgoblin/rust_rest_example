mod pets {
    use std::fmt::Display;

    #[derive(Debug, PartialEq, Hash)]
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

    #[derive(Debug, Clone, Copy, PartialEq, Hash)]
    pub enum Color {
        Red, Green, Blue
    }

    impl Display for Color {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let color = match &self {
                Color::Red => "Red",
                Color::Green => "Green",
                Color::Blue => "Blue"
            };
            write!(f, "{}", color)
        }
    }

    #[derive(Debug, Clone, PartialEq, Hash)]
    pub struct Fish {
        color: Color,
    } 

    impl Fish {
        pub fn new(c: Color) -> Fish {
            Fish {
                color: c
            }
        }

        pub fn color(&self) -> Color {
            self.color
        }
    }

    pub trait Greeting {
        fn greet(&self) -> String {
            String::from("Silent pet")
        }
    }

    impl Greeting for Dog {
        fn greet(&self) -> String {
            String::from("Gaw Gaw")
        }
    }

    impl Greeting for Fish {}

}

use pets::{Dog, Fish, Color, Greeting};

fn main() {
    let dog = Dog::new("Lucky");
    println!("Its a dog {} that say {}", dog.name(), dog.greet());

    let red_fish = Fish::new(Color::Red);
    let green_fish = Fish::new(Color::Green);
    let blue_fish = Fish::new(Color::Blue);
    println!("Its a fish {} that say {}", red_fish.color(), red_fish.greet());
    println!("Its a fish {} that say {}", green_fish.color(), green_fish.greet());
    println!("Its a fish {} that say {}", blue_fish.color(), blue_fish.greet());
}
