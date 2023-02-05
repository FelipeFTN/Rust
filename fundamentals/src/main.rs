mod print;
mod vars;
mod types;
mod strings;

fn main() {
    println!("Hello world!");

    // Print Module
    print::run();

    // Variables Module
    vars::run();

    // Types Module
    types::run();

    // Strings Module
    strings::run();
}
