use std::fmt::Display;

pub fn max_of<T: PartialOrd + Display>(a: T, b: T) -> T {
    if a > b {a} else {b}
}