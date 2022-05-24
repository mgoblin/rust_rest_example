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
