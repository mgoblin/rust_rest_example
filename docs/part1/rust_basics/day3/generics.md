# Generics
{:no_toc}

* TOC
{:toc}

Generics allows to abstract over types.

# Function generics
Lets start from functions. Try to generalize function that have two arguments the same type and return tuple. 

Generic type T declared after function name. Both arguments have type T. The function knows nothing about the type T, implementing a generalized algorithm   
```rust
pub fn tuple<T>(a: T, b: T) -> (T, T) {
    (a, b)
}
```
The concrete type T is inferred or specified explicitly when the function is called.
```rust
mod functions;

use functions::tuple;

fn main() {
    println!("{:?}", tuple(1, 3)); // Inferred
    println!("{:?}", tuple::<&str>("X","Y")); // specified explicitly

    let s1 = "X";
    let s2 = "Y";
    println!("{:?}", tuple(s1, s2)); // Inferred
}
```

# Struct generics
The same idea can be applied to structs. 
Declare Point struct with generic type T, x and y fields has type T.
```rust
struct Point<T> {
  x: T,
  y: T,
}
```
Now points with u32, f64, strings and etc x, y can be instantiated.
```rust
mod structs;

use structs::Point;

fn main() {
    let point1 = Point {x: 0, y: 10};
    let point2 = Point {x: 15.2, y: 0.0};
    let point3 = Point {x: String::from("zero"), y: String::from("one")};
    println!("{:#?}", point1);
    println!("{:#?}", point2);
    println!("{:#?}", point3);
}
```
Generic can have multiple parameters. For example, implement line struct with two points: start and end.
```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Line<A, B> {
    pub start: Point<A>,
    pub end: Point<B>,
}

...

    let point1 = Point {x: 0, y: 10};
    let point2 = Point {x: 15.2, y: 0.0};
    let point3 = Point {x: String::from("zero"), y: String::from("one")};
    println!("{:#?}", point1);
    println!("{:#?}", point2);
    println!("{:#?}", point3);

    let line = Line {start: point1, end: point2};
    println!("{:?}", line);
```
At this code line variable's generics is inferred as Line<u32, f64>.

# Enum generics and Option type
Enum generics are similar to struct generics. As enum generic example we implement Option type. Rust library has Option type and here we reimplment it only for demonstrate enum generics.

In Java using null is a common way of indicating that there is no value attached to a reference variable. But this way is dangerous,  application crash with unchecked NullPointerException is familiar to each Java developer. 

To provide solution in Java use Optional stead of null. Scala and Rust have Option type for the same reason. Rust have not null references and this is excellent language designers decision.  

Option encode the concept of a value being present or absent. Option is a enum.
```rust
pub enum Option<T> {
    Some(T),
    None,
}
```
Absence is represented as None and presence of value is represented as Some(value). 

```rust
let x = enums::Option::Some(1); //enums is a our example module for demonstration. Rust has core::option::Option  
let none = enums::Option::<i32>::None;
```
It's easy to write a function that get &str and return None if its empty or contains only spaces and Some if argument have non blank characters.
```rust
fn non_blank(s: &str) -> enums::Option<String> {
    match s.trim() {
        "" => enums::Option::None,
        x => enums::Option::Some(x.to_string())
    }
}

...

println!("{}", non_blank("s"));   // Some(s)
println!("{}", non_blank(" z ")); // Some(z)
println!("{}", non_blank("  "));  // None
println!("{}", non_blank(""));    // None

```

# Type Bound
Before this all generic types have not any bounds. T will have any type. But often type need to be bound with trait.
Lets see method generic function max_of that compare two arguments and return largest one.
```rust
// Not comile
pub fn max_of<T>(a: T, b: T) -> T {
    if a > b {a} else {b}
}
```
Not all types have a notion of compare. How to say compiler that type should implements PartialOrd trait? The answer is `T: PartialOrd`
```rust
pub fn max_of<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {a} else {b}
}

...
println!("{}", bounds::max_of(2, 3)); // print 3

```
Ok, but what if i want not only compare but easy print? T should be PartialOrd and Display. No problems - bounds can be enumerated using +. 
```rust
use std::fmt::Display;

pub fn max_of<T: PartialOrd + Display>(a: T, b: T) -> T {
    if a > b {a} else {b}
}
```

# Generics and traits
Trait can have generic parameters too.
```rust
pub trait Printable<T> {
    fn pretty(&self, f: T) -> String;
}
```
Above Prinable trait is declared with method pretty for pretty formatting to string using type T. We can implements trait for concrete T = &str and concrete type u8 like this.
```rust
impl Printable<&str> for u8 {
    fn pretty(&self, f: &str) -> String {
        format!("{} {}", f, self)
    }
}
...
let u8n: u8 = 150;
println!("{}", u8n.pretty("unsigned byte:")); //ok, print unsigned byte: 150
```
But if you try with String argument then compile time error occured.
```rust
println!("{}", u8n.pretty(String::from("just byte:"))); // compile time error
```
Implementing Printable for String generic parameter fix the error.
```rust
impl Printable<String> for u8 {
    fn pretty(&self, f: String) -> String {
        format!("{} {}", f, self)
    }
}
```

# Where clauses
A bound can be expressed using where clause. Let`s see the previous example. 
```rust
use std::fmt::Display;

pub fn max_of<T: PartialOrd + Display>(a: T, b: T) -> T {
    if a > b {a} else {b}
}
```
It can be rewrite as below.
```rust
pub fn max_of<T>(a: T, b: T) -> T where 
T: PartialOrd + Display {
    if a > b {a} else {b}
}
```

In contrast to the usual boundaries syntax, a where clause can describe more complex relationships between types. 
Suppose we want declare trait OPrintable with generic T and in where clause bound of GameMap<T> is described. That means T may be not implement Display, but GameMap<T> should. 
```rust
struct GameMap<T> {
    zero: T,
}

pub trait OPrintable<T> where 
  crate::enums::Option<T>: Display {
    fn pretty(&self) -> String;
}
```
Ok, we have Point and Point2 structs was declared as 
```rust
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

```
And Point and Point2 doesn't implements Display trait.
Now implement Display for Option<Point<u8>>
```rust
impl Display for GameMap<Point> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GameMap<Point>(x: {}, y: {})", self.zero.x, self.zero.y)
    }
}
```
And implements OPrintable for Point
```rust
impl OPrintable<Point> for Point {
    fn pretty(&self) -> String {
        format!("{}", GameMap {zero: self.clone()})
    }
}
```
But if you try implements OPrintable for Point2 compilation error occured 'the trait `std::fmt::Display` is not implemented for `GameMap<Point2>`' 


# Summary
Rust generic substitute on compile time to concrete types. And Rust have not generic type erasure.

```rust
use std::any::Any;

fn main() {
    let v1: Vec<i32> = Vec::new();
    let v2: Vec<String> = Vec::new();
    
    let a1 = &v1 as &dyn Any;
    let a2 = &v2 as &dyn Any;
    
    println!("{:?}", a1.type_id());
    println!("{:?}", a2.type_id());
}
```

---
[<< Prev](./modules.md) &ensp; [Up](../index.md) &ensp; [Next >>]()