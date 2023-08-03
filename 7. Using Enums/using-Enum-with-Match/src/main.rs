enum Fruit {
    Apple(String),      // Apple variant with a String representing its color
    Orange(u32),        // Orange variant with an unsigned 32-bit integer representing its size
    Banana { quantity: u32, is_ripe: bool }, // Banana variant with a tuple struct containing length and ripeness
}

// Function that takes a Fruit enum and performs actions based on the type and associated data
fn process_fruit(fruit: Fruit) {
    match fruit {
        Fruit::Apple(color) => println!("You have an apple with {} color.", color),
        Fruit::Orange(size) => println!("You have an orange of size {}.", size),
        Fruit::Banana { quantity, is_ripe } => println!("You have {} banana and they are {}.", quantity, if is_ripe { "ripe" } else { "not ripe" }),
    }
}

fn main() {
    let apple_color = String::from("red");
    let orange_size: u32 = 7;
    let banana_quantity: u32 = 10;
    let banana_is_ripe: bool = true;

    let fruit1 = Fruit::Apple(apple_color);
    let fruit2 = Fruit::Orange(orange_size);
    let fruit3 = Fruit::Banana {
        quantity: banana_quantity,
        is_ripe: banana_is_ripe,
    };

    process_fruit(fruit1); // Output: You have an apple with red color.
    process_fruit(fruit2); // Output: You have an orange of size 7.
    process_fruit(fruit3); // Output: You have a banana with quantity 10 and it is ripe.
}
