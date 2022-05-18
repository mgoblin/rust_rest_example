fn main() {
    let mut string1: String = "String1".to_string();
    let string2: String = String::from("String2");

    let string_literal: &str = "String_literal";

    let string_slice: &str = &string1[..];

    println!("{} {} {} {}", string1, string2, string_literal, string_slice);

    string1.push_str(" and new part");
    let string_slice2 = &string1[..];
    println!("{} {}", string1, string_slice2);
}
