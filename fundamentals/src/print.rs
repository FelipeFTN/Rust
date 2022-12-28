pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    // Basic Formatting
    println!("Number: {}", 1);

    println!("{} is from {}", "Brad", "Mass");

    // Positional Arguments
    println!(
        "{} is from {} and {} likes to {}",
        "Brad", "Brazil", "Mass", "code"
     );

    // Named Arguments
    println!(
        "{name} likes to play {activity}",
        activity = "Baseball",
        name = "John"
    );

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}
