pub mod animals;
pub mod fish;

use std::fmt::Display;

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

pub trait Greeting {
    fn greet(&self) -> String {
        String::from("Silent pet")
    }
}