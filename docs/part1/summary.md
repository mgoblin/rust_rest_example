# Summary of three days of Rust
{:no_toc}

* TOC
{:toc}

Rust is a language with large sytactic core.Learning a language takes a lot of time.

The absence of garbage collection led to the need to introduce the concepts of ownership  and borrowing. This is the newest and most unusual part of the language for newbies.
Compared to languages with garbage collection, writing code in Rust harder, but on the oher side Rust compiles to native binary with deterministic times and no pauses and freezes.

Is Rust an object oriented programming language? The answer is more yes than no.

Rust have another code reuse model compared to Java and Scala. Rust have not classical object oriented programming concepts, especially inheritance and adhoc polymorpism. 
This is unusual for Java developers. Application design style is seems like Scala with case classes.
Rust has some elements of object-oriented programming. Classical OOP concepts are expressed in other approaches and means.


Is Rust a functional and type level programming language? The answer is more no than yes.

Rust have a basic functional programming concepts: data immutablility. Functions and lambdas are a first class cityzens. But it have not Higher Kinded Types like Scala. In this part Rust more like to Java with prevalent imperative design style. 

Macros is another unusual for Java/Scala developer part of Rust.

Rust made an impression, not simple, but expressive language. 


# Explored features
## Basics
* Simple data types: Numbers, strings, enums
* Structs as structured data
* Functions
* Struct's methods
* Errors handling

## More advanced features
* Generics
* Collections

## Rust specific features
* Copy and Clone semantics
* Ownership and borrowing
* Derive implementation basics

## Program structure
* Use crates
* Modules
* private and public access

# Unexpolred language features
* Advanced data types 
* Macroses
* Traits (and traits inheritance)
* Lifetimes
* Smart pointers and data on heap (Box and etc)
* Annotations #[]
* Iterators
* Concurrency and async
* Pattern matching details
* Popular and useful crates
* unsafe code blocks

## Tooling
* Debuger
* Profiler
* Testing

---
[<< Prev](./rust_basics/day3/errors.md) &ensp; [Up](../index.md) &ensp; [Next >>]()
