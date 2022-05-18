fn say_hello(name: &str) -> String {
    format!("Hello {}", name)
}

fn say_hello2(name: &str) -> String {
    fn dear(name: &str) -> String {
        const GREETING: &str = "Dear ";
        const DEARS: [&str; 2] = ["mike", "alex"];
        let lname = &name.to_lowercase()[..];
        if DEARS.contains(&lname) {
            let mut greeting: String = String::from(GREETING);
            greeting.push_str(name);
            greeting
        } else {
            name.to_string()
        }
    }
    format!("Hello {}", dear(name))
}

fn say_hello3(name: &str) -> String {
    let dear_name = || -> String {
        const GREETING: &str = "Dear ";
        const DEARS: [&str; 2] = ["mike", "alex"];
        let lname = &name.to_lowercase()[..];
        if DEARS.contains(&lname) {
            let mut greeting: String = String::from(GREETING);
            greeting.push_str(name);
            greeting
        } else {
            name.to_string()
        }
    };
    format!("Hello {}", dear_name())
}

fn always_undersore(_x: &char) -> bool {false}

fn underscore_unknown_chars(s: &str, uchar: char, predicate: fn(&char) -> bool) -> String {
    let mut result = String::from("");
    for c in s.chars() {
        let maybe_underscored = format!(" {} ", if predicate(&c) {c} else {uchar});
        result.push_str(&maybe_underscored);
    }
    result
}

fn add(x: u32, y: u32) -> u32 {
    x + y
}

fn add_curried(x: u32) -> impl Fn(u32) -> u32 {
    move |y: u32| add(x, y)
}

fn compose<A, B, C, G, F>(f: F, g: G) -> impl Fn(A) -> C
    where
        F: Fn(A) -> B,
        G: Fn(B) -> C,
{
    move |x| g(f(x))
}

fn main() {
    println!("{}", say_hello("Mike"));

    println!("{}", say_hello2("John"));
    println!("{}", say_hello2("Alex"));

    let f = say_hello;
    println!("{}", f("John"));

    let f3 = |name: &str| {
        format!("Good day {}", name)
    };
    println!("{}", f3("Alex"));

    println!("{}", say_hello3("Mary"));


    let s = "Mike";
    let u = underscore_unknown_chars(
        s,
        '_',
        always_undersore
    );
    println!("{}", u);

    fn always_no_undersore(_x: &char) -> bool {true}
    let nu = underscore_unknown_chars(
        s,
        '?',
        always_no_undersore
    );
    println!("{}", nu);

    let nu2 = underscore_unknown_chars(
        s,
        '_',
        |_x| {true}
    );
    println!("{}", nu2);

    let nu3 = underscore_unknown_chars(
        s,
        '_',
        |x| {
            let chars = ['m', 'e'];
            let lx = &x.to_ascii_lowercase();
            chars.contains(lx)
        }
    );
    println!("{}", nu3);

    assert!(add(1, 2) == 3);
    assert!(add_curried(1)(2) == 3);
    let add_3_to = add_curried(3);
    assert!(add_3_to(5) == 8);

    let multiply_and_add_2 = compose(|x| x * 2, |x| x + 2);
    assert!(multiply_and_add_2(5) == 12);
}
