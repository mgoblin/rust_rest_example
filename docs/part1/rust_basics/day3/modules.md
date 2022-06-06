# Modules

{:no_toc}

* TOC
{:toc}

 # Modules

## Why?
As the application's source code grows, developers need a way to divide it into logical blocks. Java (and Scala) has package and import keywords for splitting code. In Java packages hierarchy tied to files and folders. 

Rust similar to Scala. One file can contains declarations of multiple structs, functions and etc. And its logical modules not ties with files so strong as in Java.

Rust module is counterpart of Java/Scala package and use keyword in Rust is like import for Java/Scala.  

## Module declaration
Rust has keyword mod for module declaration.

```rust
mod pets {
}
```

Modules can be nested.

```rust 
mod pets {
  mod animals {
  }

  mod fish {
  }
}
```

# Module elements visibility
By default all elements is private outside module and public inside module.

```rust
mod pets {
    use std::fmt::Display;

    #[derive(Debug, Clone, Copy, PartialEq, Hash)]
    enum Color {
        Red, Green, Blue
    }

    impl Display for Color {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let color = match &self {
                Color::Red => "Red",
                Color::Green => "Green",
                Color::Blue => "Blue"
            };
            write!(f, "{}", color)
        }
    }

    trait Greeting {
        fn greet(&self) -> String where Self: std::fmt::Display {
            format!("Silent pet {}", &self)
        }
    }
}
```
Module pets declares enum Color, trait Greeting.

Declare sub module animals as below

```rust
mod pets {
...

  mod animals {
        use super::Greeting;

        #[derive(Debug, Clone, PartialEq, Hash)]
         struct Dog {
             name: String,
         }

        impl Dog {
            fn new(name: &str) -> Dog {
                Dog {
                    name: String::from(name),
                }
            }

            fn name(&self) -> &str {
                &self.name[..]
            }
        }

        impl Greeting for Dog {
            fn greet(&self) -> String {
                String::from("Gaw Gaw")    
            }
        }

  }

}
``` 
At start of animals module `use super::Greeting` make pets::Greeting usable inside animals.

But trying use any pets or animals elements in main function are unsuccessful. Its because outside module and its nested modules all elements are private.

## pub keyword
[pub keyword detailed description](https://doc.rust-lang.org/reference/visibility-and-privacy.html)

pub keyword enable make element visible outside module. Add pub to Color declaration
```rust
mod pets {
  pub enum Color {
        Red, Green, Blue
  }
...
}

use pets::Color;
fn main() {
  let blue = Color::Blue;
  println!("{}", blue);
}
```


# Using module elements. Keyword use.
pub keyword changes visibility of module elements. But second part of the puzzle is declare elements usage. 
[Use keyword details](https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html) 

# Separation of code by files
Before this time all source code was placed in one file main.rs. For the real world codebases split source code by files is a "must have".

Create folder pets and file mod.rs in a folder.
```
..
pets
  ..
  mod.rs
main.rs
```
mode.rs is a root file for pets module. Its contents listed below. 
Note: no mod pet { in a source code. Rust compiler understand that file mod.rs inside pets folder contains pet module. 
```rust
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum Color {
    Red, Green, Blue
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color = match &self {
            Color::Red => "Red",
            Color::Green => "Green",
            Color::Blue => "Blue"
        };
        write!(f, "{}", color)
    }
}

pub trait Greeting {
    fn greet(&self) -> String {
        String::from("Silent pet")
    }
}
```
In main.rs add `mod pets;` at first line and `use pets::Color;` at second line. That's all.
```rust
mod pets;

use pets::Color;

fn main() {
    let blue = Color::Blue;
    println!("Color is {}", blue);
}
``` 
Try to implements fish module in pets.
```
..
pets
  ..
  fish
    ..
    mod.rs
  mod.rs
```
Fish mod.rs
```rust
use super::{Greeting, Color};

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Fish {
    color: Color,
} 

impl Fish {
    pub fn new(c: Color) -> Fish {
        Fish {
            color: c
        }
    }

    pub fn color(&self) -> Color {
        self.color
    }
}

impl Greeting for Fish {}
```  
In pets mod.rs add `pub mod fish;` to make visible fish sub module.
Now you can use fish module element in main function.
```rust
mod pets;

use pets::{Color, Greeting};

use pets::fish::Fish;

fn main() {
    let blue = Color::Blue;
    println!("Color is {}", blue);

    let red_fish = Fish::new(Color::Red);
    let green_fish = Fish::new(Color::Green);
    let blue_fish = Fish::new(Color::Blue);
    println!("Its a fish {} that say {}", red_fish.color(), red_fish.greet());
    println!("Its a fish {} that say {}", green_fish.color(), green_fish.greet());
    println!("Its a fish {} that say {}", blue_fish.color(), blue_fish.greet());

}

```

So far so good. Now start to implements animals module. I want to divide each animal to separate file. 
```
..
pets
  ..
  animals
    ..
    cat.rs
    dog.rs
    mod.rs
  fish
    ..
    mod.rs
main.rs
``` 
Dog and Cat implementation is straight forward.

dog.rs
```rust
use crate::pets::Greeting;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Dog {
    name: String,
}

impl Dog {
    pub fn new(name: &str) -> Dog {
        Dog {
            name: String::from(name),
        }
    }

    pub fn name(&self) -> &str {
        &self.name[..]
    }
}


impl Greeting for Dog {
    fn greet(&self) -> String {
        String::from("Gaw Gaw")    
    }
}
```

cat.rs
```rust
use std::fmt::Display;

use crate::pets::Greeting;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RgbColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub const WHITE: RgbColor = RgbColor {red: 255, green: 255, blue: 255};
pub const BLACK: RgbColor = RgbColor {red: 0, green: 0, blue: 0};

impl Display for RgbColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color = match self {
            &BLACK => String::from("Black"),
            &WHITE => String::from("White"),
            &RgbColor { red: r, green: g, blue:b } => {
                format!("RGB color r:{}, g:{}, b:{}", r, g, b)
            },
        };
        write!(f, "{}", color)
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Cat {
    name: String,
    color: RgbColor,
}

impl Cat {
    pub fn new(cname: &str, rgb: RgbColor) -> Cat {
        Cat {name: String::from(cname), color: rgb}
    }

    pub fn name(&self) -> &str {
        &self.name[..]
    }

    pub fn color(&self) -> RgbColor {
        self.color.clone()
    }
}

impl Greeting for Cat {
    fn greet(&self) -> String {
        format!("Meow {}", &self.name())
    }
}
```

Add to pets mod.rs
```rust
pub mod animals;
```
And finally animals mod.rs
```rust
pub mod dog;
pub mod cat;
```

And now Cat and Dog can be used in main.rs like this
```rust
use pets::animals::dog::Dog;
use pets::animals::cat::Cat;
use pets::animals::cat::{WHITE, BLACK, RgbColor};
```
But `use pets::animals::dog::Dog;` and `use pets::animals::cat::Cat;` could be more idiomatic.

Add animals mod.rs 
```rust
pub use dog::Dog;
pub use cat::Cat;
```
And now using Cat and Dog as 
```rust
use pets::animals::Dog;
use pets::animals::Cat;
```
Full source main.rs code is below
```rust
mod pets;

use pets::{Color, Greeting};
use pets::animals::Dog;
use pets::animals::Cat;
use pets::animals::cat::{WHITE, BLACK, RgbColor};
use pets::fish::Fish;

fn main() {
    let blue = Color::Blue;
    println!("Color is {}", blue);

    let red_fish = Fish::new(Color::Red);
    let green_fish = Fish::new(Color::Green);
    let blue_fish = Fish::new(Color::Blue);
    println!("Its a fish {} that say {}", red_fish.color(), red_fish.greet());
    println!("Its a fish {} that say {}", green_fish.color(), green_fish.greet());
    println!("Its a fish {} that say {}", blue_fish.color(), blue_fish.greet());

    let dog = Dog::new("Lucky");
    println!("Its a dog {} that say {}", dog.name(), dog.greet());

    let white_cat = Cat::new("Kitty", WHITE);
    let black_cat = Cat::new("Blacky", BLACK);
    let cat = Cat::new("Kiss", RgbColor{red: 10, green: 0, blue: 5});
    println!("Its a cat {} with color {} that say {}", white_cat.name(), white_cat.color() , white_cat.greet());
    println!("Its a cat {} with color {} that say {}", black_cat.name(), black_cat.color() , black_cat.greet());
    println!("Its a cat {} with color {} that say {}", cat.name(), cat.color() , cat.greet());
}
```


# Summary
---
[<< Prev](../day2/collections.md) &ensp; [Up](../index.md) &ensp; [Next >>]()  
