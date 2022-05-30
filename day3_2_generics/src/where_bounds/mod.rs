use std::fmt::Display;

#[derive(Clone)]
pub struct Point {
    pub x: u8,
    pub y: u8,
}

#[derive(Clone)]
pub struct Point2 {
    pub x: u8,
    pub y: u8,
}

pub struct GameMap<T> {
    zero: T,
}

pub fn max_of<T>(a: T, b: T) -> T where 
T: PartialOrd + Display {
    if a > b {a} else {b}
}

pub trait OPrintable<T> where 
  GameMap<T>: Display {
    fn pretty(&self) -> String;
}

impl Display for GameMap<Point> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GameMap<Point>(x: {}, y: {})", self.zero.x, self.zero.y)
    }
}

impl OPrintable<Point> for Point {
    fn pretty(&self) -> String {
        format!("{}", GameMap {zero: self.clone()})
    }
}

// Compilation error GameMap<Point2> doesnt implments Display
// impl OPrintable<Point2> for Point2 {
//     fn pretty(&self) -> String {
//         format!("{}", GameMap {zero: self.clone()})
//     }
// }