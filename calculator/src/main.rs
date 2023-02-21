use std::env::{args, Args};
mod calculator;

fn main() {
    let mut args: Args = args();

    // the first argument is the location of the compiled binary
    let first: String = args.nth(1).unwrap();
    let operator: char = args.nth(0).unwrap().chars().next().unwrap();
    let second: String = args.nth(0).unwrap();

    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();
    let result = calculator::run(operator, first_number, second_number);

    println!("{} {} {} = {}", first_number, operator, second_number, result);
}
