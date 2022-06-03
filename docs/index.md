# Create REST service on Rust from scratch to production journey

* TOC
{:toc}

## Motivation (you can skip it)

**Well known programming languages**

More than 15 years I has been using Java as primary language for job and pet projects. 
Java is a mainstream object oriented compiled language with rich set of features and strong typing. Spring Boot is an amazing every day tool and i love it. 

Scala (especially Scala 3) is an excellent strong typing language. It naturally mixes object oriented and functional paradigms. But nowadays Scala doesn`t have mature and simple frameworks for regular programming tasks (REST microservices, database CRUD and etc).

Python is a dynamically typed interpret language. Python shines in the niche of rapid prototyping. But low runtime performance and dynamic typing not the good if you developing enterprise software.

**New kids on the block: Golang and Rust**

Golang. Simple, noisy syntax. Decentralized library repositories. Defer, embedded in language data structures and "magical" functions like make. And unusual interfaces hmm. Thanx, next time.

Rust. Compilation to native binaries. Centralized library repository. No GC. Compact syntax. Ok, lets try.

## Starting the journey
One of the paths to learn new language or technology is to implement some application from scratch.

I will try to implement Rest service application around Postrgres DB table users on Linux.
When evaluate a new technology I prefer incremental approach:

Part1

1. Setup development environment: compiler, IDE, DB and so on. Run "hello world"
2. Grasp the language basics. Structure of application, data types,decomposition, reuse, and dealing with complexity.

Part 2

3. Define basic functional requirements.
4. Choose frameworks and libraries
5. Start to implements application as a series of steps from "it seems to work" to enterprise ready level 


## Setup development environment
I use Manjaro Linux. 
Install via package manager Rust, Cargo (Rust dependecy management and build tool), Visual Studio Code with Rust plugin.
Next steps are to install Postgres and DBeaver (GUI tool for Postgre).
Thats all.

Create new project. In linux terminal run 

```
cargo new rust_rest_example` 
cd rust_rest_example`
cargo run
```

After all you can see "Hello wold" in console. And you can see main.rs in src folder with obvious code:

```rust
fn main() {
    println!("Hello, world!");
}
``` 

