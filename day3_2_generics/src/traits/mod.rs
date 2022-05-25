pub trait Printable<T> {
    fn pretty(&self, f: T) -> String;
}

impl Printable<&str> for u8 {
    fn pretty(&self, f: &str) -> String {
        format!("{} {}", f, self)
    }
}

impl Printable<String> for u8 {
    fn pretty(&self, f: String) -> String {
        format!("{} {}", f, self)
    }
}