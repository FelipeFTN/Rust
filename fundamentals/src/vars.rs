// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let mut lucky_number = 0;
    let name = "Felipe";
    let age = 18;

    // A variable must be read before changing
    println!(
        "My name is {} and I am {}, my lucky number is {}",
        name, age, lucky_number
    );

    lucky_number = 2;

    println!(
        "My name is {} and I am {}, my lucky number is {}",
        name, age, lucky_number
    );
}
