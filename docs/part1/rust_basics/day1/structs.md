# Structs
{:no_toc}

* TOC
{:toc}

# Struct
Structures lets you package together and name multiple related values that make up a meaningful group.

## Declare and instantiate

```rust
struct Person {
  first_name: String,
  last_name: String,
  age: u8,
}
```
To create struct instance 
```rust
struct Person {
  first_name: String,
  last_name: String,
  age: u8,
}

fn main() {
  let mike = Person { 
    first_name: String::from("Mike"), 
    last_name: String::from("Smith"), 
    age: 45,
  };
  
  println!("Person with first name '{}', last name '{}', age = {}", 
    mike.first_name, mike.last_name, mike.age);
}

```

## Implement struct methods
impl keyword allows to declare and implement functions in scope of data type. Function that have &self as argument is an instance mehtod. You call it via variable dot syntax (see mike.name() in the example below). Methods without &self are type level (or static in java termilogy) methods that called with <type name>::method() syntax (see Person::descr() in the example below)

```rust
struct Person {
  first_name: String,
  last_name: String,
  age: u8,
}

impl Person {
    
    fn descr() -> String {
        "Peson is a man, woman, or child.".to_string()
    }
    
    fn name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

fn main() {
  let mike = Person { 
    first_name: String::from("Mike"), 
    last_name: String::from("Smith"), 
    age: 45,
  };
  
  println!("{}", Person::descr());
  
  println!("Person with name '{}' and age = {}", 
    mike.name(), mike.age);
}

```
# Traits (intro)
A trait tells the Rust compiler about functionality a particular type has and can share with other types. We can use traits to define shared behavior in an abstract way. 

## Trait declaration
``` rust
trait Greeter {
    fn greeting(&self, greet: &str) -> String;
}
```
At the example above declare trait Greeter with one method greeting. First argument of greeting method is &self (reference to type instance), second argument is string slice with greeting ("Hi", "Hello", "Good morning" and so on).

## Trait implementation for type
```rust 
impl Greeter for Person {
    fn greeting(&self, greet: &str) -> String {
        format!("{} {}", greet,  self.first_name)
    }   
}
```
## Call trait methods
No diffs with ordinal implementations
```rust
let mike = Person { 
    first_name: String::from("Mike"), 
    last_name: String::from("Smith"), 
    age: 45,
  };
  
println!("{}", mike.greeting("Hello"));
```


# Struct as a case (records) class
Scala has case classes. And last versions of Java language has records.
This is concise syntax for defining immutable data-only classes with automatically implemented methods for creation, equality, comparing, displaying and so on. 

## Create immutable struct instances 
There is no special syntax. You should use immutable variables and references.
To partially change some fields and copy other fields from .. could be used
```rust
let mike = Person { 
    first_name: String::from("Mike"), 
    last_name: String::from("Smith"), 
    age: 45,
  };
  
let john = Person {
    first_name: String::from("John"),
    ..mike
};
  
println!("{}, {}, {}", john.first_name, john.last_name, john.age);
``` 
At example above john struct instance set first_name to value "John" and copy other fields from mike struct instance.

## Struct string representation
For debugging and printing, it is usefult to have a possibility convert struct instance to its string representation.
All java classes has method toString() for this reason.

### Explicit string representation
Trait std::fmt::Display has one method to convert from type instance to string representation.
```rust
fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result;
``` 
Let's Implement Display trait for Person type.
```rust
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Person (first_name: {}, last_name: {}, age: {})", 
            self.first_name, self.last_name, self.age)
    }
}
```

Now we can print Person to console
```rust
fn main() {
  let mike = Person { 
    first_name: String::from("Mike"), 
    last_name: String::from("Smith"), 
    age: 45,
  };
  
  println!("{}", mike);
}
```
and see the result `Person (first_name: Mike, last_name: Smith, age: 45)`

### Declarative string implementation
#[derive(Debug)] attribute (annotation in java terminology) add ability to print struct for debug purpose. Use {:?} or pretty print {:#?} format string in println!.
```rust
#[derive(Debug)]
struct Person {
  first_name: String,
  last_name: String,
  age: u8,
}

fn main() {
  let mike = Person { 
    first_name: String::from("Mike"), 
    last_name: String::from("Smith"), 
    age: 45,
  };
  
  println!("{:#?}", mike);
}
```
The result is printing to console
```
Person {
    first_name: "Mike",
    last_name: "Smith",
    age: 45,
}
```
## Clone 
You can duplicate struct instance. Simplest way to support cloning in structs is deriving trait std::clone::Clone.
```rust
#[derive(Debug, Clone)]
struct Person {
  first_name: String,
  last_name: String,
  age: u8,
}

...
let mike = Person { 
  first_name: String::from("Mike"), 
  last_name: String::from("Smith"), 
  age: 45,
};
  
let mike_clone = mike.clone();
```    
## Equality (equivalence)
Java Object class has method `boolean equals(Object obj)` to implements equality. The first conception in java is equality by reference and by value. Operator `==` for equals by reference and obj1.equals(obj2) for equals by value. Using == instead of equals is typical java newbies error. 

As a java developer you should implement equals on each class that may be using as entity and therefore containing fields. This is verbose and repetitive task. Java IDE can generate code of equals. Tools like Lombok solve this problem at compile time. Starting from Java 14 developers can use record and that is significant improvement.  

Scala initially has case classes. But what about Rust? 

### Equality is simple? No.
Equivalence is relation between group of elements. Equivalence relation may have symmetry, transitivity and reflexivity properties.

Symmetry: if a == b then b == a; and if a != b then b != a.

Transitivity: if a == b and b == c then a == c. 

Reflectivity:  a == a 

If relation satisfy only symmetry and transitivity properties its named **partial equivalence**.

Another question is are the a, b, c should be one type? Atmost cases yes, we compare apples with apples and oranges with oranges. But some times we want to compare fruits.

### PartialEq trait

Trait for equality comparisons which are partial equivalence relations.

```rust
fn eq(&self, other: &Rhs) -> bool;

fn ne(&self, other: &Rhs) -> bool;
```
Implementations must ensure that eq and ne are consistent with each other:

a != b if and only if !(a == b) (ensured by the default implementation).

Really ne method already implemented as 
```rust
!self.eq(other)
```

PartialEq can be used with derive.
```rust
#[derive(Debug, Clone, PartialEq)]
struct Person {
  first_name: String,
  last_name: String,
  age: u8,
}
```
Rust compiler will try call to eq method on type when compile == operator in source code. If type doesn't implements PartialEq compiler generate error 'binary operation `==` cannot be applied to type `<type name>`' 

```rust
fn main() {
  let mike = Person { 
    first_name: String::from("Mike"), 
    last_name: String::from("Smith"), 
    age: 45,
  };
  
  let mike2 = mike.clone();
  
  let mike3 = Person {
    first_name: String::from("Mike"),
    last_name: String::from("Smith"),
    age: 45,
  };
  
  // Symmetry
  println!("{}", mike == mike2);
  println!("{}", mike2 == mike);
  
  // Transitivity
  println!("{}", mike == mike3);
  println!("{}", mike2 == mike3);
  println!("{}", mike == mike2);  
}

```
Result is 
```
true
true
true
true
true
```
You can call eq and ne methods explicitly

```rust
println!("{}", mike.eq(&mike2));
```

Derivable PartialeEq implementation compare structs the same type only. But you can override this behavior by custom implementation. [StackOverflow answer to this question](https://stackoverflow.com/questions/35161176/checking-equality-of-custom-structs) 
Or see PartialEq documentation.

### Eq
Eq trait has the same methods of PartialEq. Really Eq is inherits PartialEq. 
Derivable Eq does not generate implementation. Its only marker that PartialEq implementation for concrete type has reflectivity.
At most cases Eq in derive using together with PartialEq.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
struct Person {
  first_name: String,
  last_name: String,
  age: u8,
}
```  

## Hashing
The short answer is 
```rust
#[derive(Hash)]
```
Equal struct instances should have the same hash values. The same as Java. But details is different from java/scala.

In Java class developer should implement hashCode() method. And class can implements only one hashing algorithm.
Rust use Hasher trait. Trait may be implemented by many structs. Each struct can implements different hashing algorithm.
On Hash default implementation has Hasher as parameter. Therefore struct easily can be hashed using different algs if this required.
This approach is more flexible than Java.  

Now Person struct code looks like this
```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Person {
  first_name: String,
  last_name: String,
  age: u8,
}
```

## Compare and ordering
This story about deriving PartialOrd and Ord traits.
PartialOrd:

when derived on structs, it will produce a lexicographic ordering based on the top-to-bottom declaration order of the struct’s members.

# Summary
Structures are familiar and pretty simple. 

Deriving allow to write clear and concise code on most use cases and override implementation when its really need. 

---
[<< Prev](./numbers.md) &ensp; [Up](../index.md) &ensp; [Next >>]()