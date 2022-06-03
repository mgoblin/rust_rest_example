# Numbers, enums and Strings. Variables and references

{:no_toc}

* TOC
{:toc}

# Learning resources
[Official Rust language docs](https://doc.rust-lang.org/book/title-page.html)

[The gentle introduction to Rust](https://stevedonovan.github.io/rust-gentle-intro/readme.html)

# Numbers
Signed and unsigned integers in Rust is a good idea. Thanx to Rust language designers.

Floating point values has some pitfalls with precision.
```rust 
assert!(0.1 + 0.2 == 0.3)
```

returns false. If printing addition result 
```rust 
println!("{}", 0.1 + 0.2);
``` 
the reason becomes clear: 0.30000000000000004

This is not surprise for Java developers familiar with float data type.
 

# Immutability and references
Mutable/Immutable variables are good parts of the language. No problems with this concept if you have experience with Scala.
Rust doesn't have nulls. Its great thing.

References are more complex. The simple part is deref 

```rust
let x = 42;  
let r = &x;

println!("{}", r);
```

42 

More complex part is a mutable reference and funtcions call. You can modify values via mutable references inside function and this changes are visible outside function. This side effects make function not pure. But you can explicitly declare mutable reference.

```rust
fn main() {

  let mut x = 42;
  let r = &mut x;
  
  func1(r);

  println!("{}", r);

}

fn func1(a:&mut u32) {
  *a = *a + 1;
  println!("a = {}", a);
}
```

And finally equality... This is broad topic to investigate later, but now//

```rust 
#[derive(PartialEq)]
struct X {
  y: u32,
}

fn main() {
  let x1 = X {y: 1};
  let x2 = X {y: 1};

  let r1 = &x1;
  let r2 = &x2;
  
  println!("{}", x1 == x2);

  println!("{}", r1 == r2);
}
```

Its seems no concept of eq by value and eq by ref. Both x1 and x2 are equals and r1 and r2 are equals too.

# Enums
Enums are types which have a few definite values.

Defining an enum is pretty straightforward.
```rust
enum IpAddrKind {
    V4,
    V6,
}
```
Create enum variable like this.
```rust
 let four = IpAddrKind::V4;
 let six = IpAddrKind::V6;
```

# Strings and slicing
String literals, String type and &str. Its a little annoying and seems overcomplicated. 
```rust
fn main() {
    let mut string1: String = "String1".to_string();
    let string2: String = String::from("String2");

    let string_literal: &str = "String_literal";

    let string_slice: &str = &string1[..];

    let string3 = String::from(string_slice);

    println!("{} {} {} {} {}", string1, string2, string_literal, string_slice, string3);

    string1.push_str(" and new part");
    let string_slice2 = &string1[..];
    println!("{} {}", string1, string_slice2);
}
```

Output is like below
```
String1 String2 String_literal String1 String1
String1 and new part String1 and new part
```

# Summary
Rust core gives the impression of a well-designed programming language with few pain points: 
* strings are not so simple
* floats precision can be a problem on working with money.

let's keep diving.

---
[<< Prev](../index.md) &ensp; [Up](../index.md) &ensp; [Next >>]()  