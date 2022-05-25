use std::fmt::Display;

pub enum Option<T> {
    Some(T),
    None,
}

impl <T: std::fmt::Display> Display for Option<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = if let Option::Some(x) = self {
            format!("Some({})", x)
        } else {
            String::from("None")
        };
        write!(f, "{}", s.as_str())
    }
}