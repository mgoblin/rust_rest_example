# Cargo and crates
{:no_toc}

* TOC
{:toc}

# Cargo
Cargo is Rust’s build system and package manager.
This is convenient tool for create and build rust applications.

I highly recommend reading [Hello Cargo chapter](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)

## Structure of Cargo project
```
src <- root of source files
  ..
  main.rs <- program entry point
target <- build process artifacts. auto generated.
  ..
  debug <- debug build folder (cargo build)
  release <- release build folder (cargo build --release)
Cargo.toml <- describe package, dependencies and so on.
Cargo.lock <- auto generated file. This ensures that all crates are using the same version of all dependencies.
```

Cargo.toml content 
```
[package]
name = "day2_1_functions_clousures"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```
[package] - program description block 
* name - name of compiled executable file (binary)
* version - version of binary
* edition - Rust language edition 

[dependencies] - here you can place your code dependencies (other packages)


# Manage dependencies. Crates.
Community-provided libraries placed in [crates.io](https://crates.io/). crates.io for Rust ecosystem is like maven central for Java. And Rust library named as **crate**.
 
Adding a new library (dependency) for example json:
```
[dependencies]
json = "0.12.4"
```
'json' is a name of lib from crates.io and 0.12.4 is json lib version

Libraries has features that can be enabled or disabled. In this case dependency looks like below
```
serde = { version = "1", features = ["derive"] }
``` 
# Cargo (continued) 
## Cargo workspaces
Some times you need split up code on multiple subprojects. At our case each subproject folder should contains dayX_Y source files.

Cargo have workspaces feature for this. 
Workspace folder structure looks like below.
```
root folder
  Cargo.toml <- list of workspaces
  <ws1 folder>
    src <- source code 
    Cargo.toml <- ws1 package and dependencies description
  <wsN folder>
    src
    Cargo.toml <- ws1 package and dependencies description
```
See github repository structure.

# Summary
Cargo seems as simple pretty good build and dependecy management tool. Crates is useful and centralized in crates.io site. 

Open questions
* How to create my own library/crate and publish it? 
* How to crates in http://crates.io supported and secure against malicious source code?
* Does alternative crates sites exists?

[<< Prev](../day2/functions.md) &ensp; [Up](../index.md) &ensp; [Next >>](./collections.md)  
