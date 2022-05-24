use super::{Greeting, Color};

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

impl Greeting for Fish {}