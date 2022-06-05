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

fn f_add_five() -> fn(i32) -> i32 {
    fn inner(x: i32) -> i32 { x + 5 }
    inner 
}

fn f_add_five2() -> impl Fn(i32) -> i32 {
    fn inner(x: i32) -> i32 { x + 5 }
    inner 
}

fn f_add_five_cl() -> impl Fn(i32) -> i32 {
    |x| x + 5
}

fn f_add_five_cl2() -> impl Fn(i32) -> i32 {
    let five = 5;
    move |x| x + five
}

// functions cannot aquire its environment
// commented code below does not compiled.
// fn add_to(x: i32) -> fn(i32) -> i32 {
//     fn adding(y: i32) -> i32 {
//         x + y
//     }
//     adding
// }

fn iter(
    x0: fn(f32) -> f32,
    make_guess: fn(x1: f32, x2: f32, step: i32) -> f32,
    is_good_enough: fn (f32, f32) -> bool
) -> impl Fn(f32) -> f32 {
    move |x| {
        let mut i = 1;
        let mut guess = make_guess(x0(x), x, i);
        while !is_good_enough(guess, x) {
            i += 1;
            guess = make_guess(guess, x, i);    
        }
        guess
    }
} 

fn abs(x: f32) -> f32 {
    if x >= 0.0 {x} else {-x}
}

fn sqrt_good_enough(guess: f32, x: f32) -> bool {
    abs(x - guess * guess) <= 0.001
}

fn ln_good_enough(guess: f32, x: f32) -> bool {
    let e = 2.7182818284590452353602874713527f32;
    abs(e.powf(guess) - x) <= 0.001
}

fn sqrt_guess(guess: f32, x: f32, _i: i32) -> f32 {
    (guess + x / guess) / 2.0
}

fn ln_y(x: f32) -> f32 {
    (x - 1.0)/(x + 1.0)
}

fn ln_nth(n: i32) -> i32 {
    2 * n - 1
}

fn ln_guess(guess: f32, x: f32, i: i32) -> f32 {
    let n = ln_nth(i) as f32;
    guess + 2.0 * (1.0 / n) * ln_y(x).powf(n)
}

fn sqrt_start(x: f32) -> f32 {
    x / 2.0
}

fn ln_start(_x: f32) -> f32 {
    0.0 
}

fn sqrt(x: f32) -> f32 {
    let f = iter(
        sqrt_start, 
        sqrt_guess, 
        sqrt_good_enough
    );
    f(x)
}

fn ln(x: f32) -> f32 {
    let f = iter(
        ln_start,
        ln_guess,
        ln_good_enough
    );
    f(x)
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
    // say_hello is a simple function
    println!("{}", say_hello("Mike"));

    // say_hello2 use inner function
    println!("{}", say_hello2("John"));
    println!("{}", say_hello2("Alex"));

    // function can be stored in variables
    let f = say_hello;
    println!("{}", f("John"));

    // simple closure
    let f3 = |name: &str| {
        format!("Good day {}", name)
    };
    println!("{}", f3("Alex"));

    // say_hello3 is function using inner closure
    println!("{}", say_hello3("Mary"));

    // functions as argument
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

    // funtion that returns function
    let add_five = f_add_five();
    let add_five2 = f_add_five2();
    let add_five3 = f_add_five_cl();
    let add_five4 = f_add_five_cl2();
    println!("{}", add_five(10)); // print 15
    println!("{}", add_five2(10)); // print 15
    println!("{}", add_five3(10)); // print 15
    println!("{}", add_five4(10)); // print 15


    println!("{}", sqrt(2.0));
    println!("{}", ln(10.0));

    // curring
    assert!(add(1, 2) == 3);
    assert!(add_curried(1)(2) == 3);
    let add_3_to = add_curried(3);
    assert!(add_3_to(5) == 8);

    // function composition
    let multiply_and_add_2 = compose(|x| x * 2, |x| x + 2);
    assert!(multiply_and_add_2(5) == 12);
}
