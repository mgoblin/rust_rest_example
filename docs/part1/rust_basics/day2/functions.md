# Functions and closures
{:no_toc}

* TOC
{:toc}

# Functions
Funcions is a basic prorgamming concept. This is named piece of code with input arguments and result (return value). Function call with agruments return a value. In math funcion is a mapping arguments to result. If the result depends only on argument values function is a pure, otherwise it have side effects (get or modify something outside it scope).

## Declaration

Functions in Rust are declared using keyword fn <function_name>(<function args>) -> <return type>
```rust
fn say_hello(name: &str) -> String {
    format!("Hello {}", name)
}
```
Each argument have name and explicit type.

Last expression without ; consider as a function result. But you can use return keyword with or without ;
```rust
fn say_hello(name: &str) -> String {
    return format!("Hello {}", name)
}
```

```rust
fn say_hello(name: &str) -> String {
    return format!("Hello {}", name);
}
```

## Known Limitations
Сompared to Java and Scala functions in Rust have some limitations
* No tail recursion optimization (as Scala)
* No varags 
* No function overloading

## Inner functions
Rust enables to declare and call functions inside functions.
```rust
fn say_hello(name: &str) -> String {
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
```

## Functions as variables
Variables can have function as value
```rust
let f = say_hello; // f has value fn (&str) -> String
...

println!("{}", f("John"));
``` 

# Closures
Closure is an anonymous function (function without name). Unlike functions, closures can capture values from the scope in which they’re defined.
Closures in Rust has special syntax
```rust
let f3 = |name: &str| {
  format!("Good day {}", name)
};
...
println!("{}", f3("Alex"));
```
Previous say_hello with inner function can be rewrite using clousures. Closure dear_name see name argument of parent function.
```rust
fn say_hello(name: &str) -> String {
    let dear_name =  || -> String {
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
```

# Functional programming basics

## Functions and closures as arguments
Like Java and Scala Rust can accept functions and closures as argument of function.

Lets see example. Function underscore_unknown_chars replace chars in input string to 'underscore' chars.
First argument is input string (slice). Second argument is underscore char and the last argument is a predicate **function**.

```rust
fn underscore_unknown_chars(s: &str, uchar: char, predicate: fn(&char) -> bool) -> String {
    let mut result = String::from("");
    for c in s.chars() {
      let maybe_underscored = format!(" {} ", if predicate(&c) {c} else {uchar});
      result.push_str(&maybe_underscored);       
    }
    result
}

fn always_undersore(_x: &char) -> bool {false}

fn main() {
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
}
```

Predicate function has one char argument and should retrun false if char should be replaced and true if not.  

Function underscore_unknown_chars iterate input string s chars using for. For each input string char predicate calls and maybe_underscored variable fills char or underscore char depending of predicate function return value.
Chars accumulates in result string. 

We can declare function and than use it as an argument of  underscore_unknown_chars
```rust
fn always_undersore(_x: &char) -> bool {false}

...
fn main() {
  let s = "Mike";
  let u = underscore_unknown_chars(
    s, 
    '_', 
    always_undersore
  );
  println!("{}", u);
```

Predicate can be declared as inner funtion (for example in main function body).

You can use closure as function argument. 

```rust
let nu3 = underscore_unknown_chars(
  s, 
  '_', 
  |x| {
    let chars = ['m', 'e'];
    let lx = &x.to_ascii_lowercase();
    chars.contains(lx)
  }
);
```

In this case closure has limitation. It can't capture extenal variables.This limitation can be overcome, but need using dyn trait calls. Now its outscope.

## Functions and closures as return type.
Functions and closures can not only accept other function as argument, 
but using functions and closures as return.

Let's see example.
```rust
fn f_add_five() -> fn(i32) -> i32 {
    fn inner(x: i32) -> i32 { x + 5 }
    inner 
}
```
In example above function f_add_five has no arruments and declare fn(i32) -> i32 as a return type. This means that f_add_five returns function with one i32 argument and i32 result.

Implementation of f_add_five is quite simple. Function inner add 5 to its argument.

Function in Rust have a trait type std::ops::Fn. And add_five can be declared as below.
```rust
fn f_add_five2() -> impl Fn(i32) -> i32 {
    fn inner(x: i32) -> i32 { x + 5 }
    inner 
}
```

Now rewrite add_five body with closure.
```rust
fn f_add_five_cl() -> impl Fn(i32) -> i32 {
    |x| x + 5
}
```
Its implementation is more readable than previous. And has one useful feature: closure can capture values from scope.

Let's try to capture variable in closure.

```rust
// Not compile
fn f_add_five_cl2() -> impl Fn(i32) -> i32 {
    let five = 5;
    |x| x + five // here we try to use five
}
```
Compile error occured.
```
|x| x + five
   |     ^^^     ---- `five` is borrowed here
   |     |
   |     may outlive borrowed value `five`
```
We need say to compiler that five ownership moved from function to closure. Keyword move in closure declaration transfer ownership.

```rust
fn f_add_five_cl2() -> impl Fn(i32) -> i32 {
    let five = 5;
    move |x| x + five
}
```

Ok. So far so good. But what is the use of constructing functions? See next section

## Generalize computational alogrithms (you can skip it)

Consider the Newton algorithm for calculating the square root of a number. 

Newton computation algorithm of number's X square root is a series of approximatons. As a first guess is X/2. And next guess can be compute as 
G2 = (G1 + X/G1) / 2, where G1 is a previous guess value, G2 is a current guess. Guesses is making until the previous value and the current value differ by a specific amount, for example 0.01. This specific amount is computational difference between math defined and computed square root value of X.

Next consider one of natural logorithm calculation algorithm.

ln(x) = ln( (1 + y) / (1 - y) ) = 2y * (1/1 + 1/3 y<sup>2</sup> + 1/5 y<sup>4</sup> + 1/7 y<sup>6</sup>  + ...)

where y = (x−1)/(x+1)

Both algorithms have common in a computation. Starting from some guess it iterationally improve the result using previous tryies and optionally iteraion counter. Bunch of asymptotic alrorithms is based on the same idea and computation shape.

How to generalize this computational shape?

It is necessary to combine three parts: calculation of the first approximation, iterative calculation of subsequent approximations until the required accuracy of the calculations is reached. The result of combination is a function than calculate math.

in pseudo code its show like this
```
i = 1
guess = make_guess(x0(x), x, i) // first guess
while not good_enough(guess) {
    increment i
    guess = make_guess(guess, x, i)
}
```
where 
* x is input value
* i is iteration counter
* make_guess is a approximation function
* guess is the current approximation value
* x0 is function that calc first guess
* good_enough is a boolean function that is true when required accuracy is reached.

In Rust that pseudo code can be write as
```rust
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
```

First we implements is_good_enough for square root and natural logorithm.

Square root (sqrt) is_good_enough
```rust
fn sqrt_good_enough(guess: f32, x: f32) -> bool {
    abs(x - guess * guess) <= 0.001
}
```
where 
* abs is number's absolute value (number value without a sign)
* 0.01 - required accuracy

```rust
fn abs(x: f32) -> f32 {
    if x >= 0.0 {x} else {-x}
}
```

Natural logorithm (ln) root is_good_enough
```rust
fn ln_good_enough(guess: f32, x: f32) -> bool {
    let e = 2.7182818284590452353602874713527f32;
    abs(e.powf(guess) - x) <= 0.001
}
```
The next step is x0 function implementation.

For the square root 
```rust
fn sqrt_start(x: f32) -> f32 {
    x / 2.0
}
```
And for the natural logorithm
```rust
fn ln_start(_x: f32) -> f32 {
    0.0 
}
```

And now we should implement make_guess functions for square root and ln.

Square root implemenation is 
```rust
fn sqrt_guess(guess: f32, x: f32, _i: i32) -> f32 {
    (guess + x / guess) / 2.0
}
```

Natural logorithm implementation is 
```rust
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
```

Finally write in Rust function that implements computaion iterative approximation shape

```rust
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
```

Now we can define sqrt and ln functions as

```rust
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
```

## Currying

```rust
fn add(x: u32, y: u32) -> u32 {
    x + y
}

fn add_curried(x: u32) -> impl Fn(u32) -> u32 {
    move |y: u32| add(x, y)
}

fn main() {
  assert!(add(1, 2) == 3);  
  assert!(add_curried(1)(2) == 3);

  let add_3_to = add_curried(3);
  assert!(add_3_to(5) == 8);
}
```
Start with simple implementation of add function: x + y. No functions as return type. 
We want to curring this function. Currying is multi argument function transformation to functions with one argument that returns a function with remaining args and so on.

BiFunction add(x, y) curried to add(x)(y).

Ok. How to do this in Rust?

First step: how to declare that function return function? The function in Rust has a type trait Fn.  
```rust
fn add_curried(x: u32) -> impl Fn(u32) -> u32
``` 
Declare return type as impl Fn(u32) -> 32 than means 'function retrun implementation of function with u32 argument and u32 return type'.
Next step: function implementation.

Try to return closure than capture x value from add curried argument and have second operand as its own argument. 

```rust
fn add_curried(x: u32) -> impl Fn(u32) -> u32 {
    |y: u32| x + y
}
```
And see the error
```
|y: u32| x + y
  |     -------- ^ borrowed value does not live long enough
  |     |
  |     value captured here

```
Here we need move keyword. Move converts any variables captured by reference or mutable reference to variables captured by value.
```rust
fn add_curried(x: u32) -> impl Fn(u32) -> u32 {
    move |y: u32| x + y
}
```

And the last step: call function add inside closure
```rust
fn add_curried(x: u32) -> impl Fn(u32) -> u32 {
    move |y: u32| add(x, y)
}
``` 

## Function composition
```rust
fn compose<A, B, C, G, F>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}
```
This code use generics. Generics described in the next sections. But its readable and uderstandable.

Use compose like below:
```rust
let multiply_and_add_2 = compose(|x| x * 2, |x| x + 2);
assert!(multiply_and_add_2(5) == 12);
```

# Summary
Rust has basic functional programming capabilities. Not so obvious as Scala, but good enough.

---
[<< Prev](../day1/ownership.md) &ensp; [Up](../index.md) &ensp; [Next >>]()  
