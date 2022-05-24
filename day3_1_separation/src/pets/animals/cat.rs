use std::fmt::Display;

use crate::pets::Greeting;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RgbColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub const WHITE: RgbColor = RgbColor {red: 255, green: 255, blue: 255};
pub const BLACK: RgbColor = RgbColor {red: 0, green: 0, blue: 0};

impl Display for RgbColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color = match self {
            &BLACK => String::from("Black"),
            &WHITE => String::from("White"),
            &RgbColor { red: r, green: g, blue:b } => {
                format!("RGB color r:{}, g:{}, b:{}", r, g, b)
            },
        };
        write!(f, "{}", color)
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Cat {
    name: String,
    color: RgbColor,
}

impl Cat {
    pub fn new(cname: &str, rgb: RgbColor) -> Cat {
        Cat {name: String::from(cname), color: rgb}
    }

    pub fn name(&self) -> &str {
        &self.name[..]
    }

    pub fn color(&self) -> RgbColor {
        self.color.clone()
    }
}

impl Greeting for Cat {
    fn greet(&self) -> String {
        format!("Meow {}", &self.name())
    }
}