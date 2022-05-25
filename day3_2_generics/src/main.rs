mod structs;
mod enums;
mod functions;

use structs::*;
use functions::tuple;


fn main() {
    let point1 = Point {x: 0, y: 10};
    let point2 = Point {x: 15.2, y: 0.0};
    let point3 = Point {x: String::from("zero"), y: String::from("one")};
    println!("{:#?}", point1);
    println!("{:#?}", point2);
    println!("{:#?}", point3);

    let line = Line {start: point1, end: point2};
    println!("{:?}", line);

    println!("{:?}", tuple(1, 3));
    println!("{:?}", tuple::<&str>("X","Y"));

    let s1 = "X";
    let s2 = "Y";
    println!("{:?}", tuple(s1, s2));

    let x = enums::Option::Some(1);
    let none = enums::Option::<i32>::None;

    println!("{}", x);
    println!("{}", none);

    println!("{}", non_blank("s"));   // Some(s)
    println!("{}", non_blank(" z ")); // Some(z)
    println!("{}", non_blank("  "));  // None
    println!("{}", non_blank(""));    // None
}

fn non_blank(s: &str) -> enums::Option<String> {
    match s.trim() {
        "" => enums::Option::None,
        x => enums::Option::Some(x.to_string())
    }
}
