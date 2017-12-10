// Rust includes only a limited scope during "prelude" phase
// Import "io" from "std" library
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // Variables are immutable by default
    // "mut" makes "guess" mutable
    // "::" calls associated functions (static methods)
    let mut guess = String::new();

    // "read_line" takes a "reference" where the input is stored
    // "reference" should be "mutable", so we add "&mut guess" instead of "&guess"
    // "reference" are immutable by default
    io::stdin().read_line(&mut guess)
    // "read_line" returns a "Result" which an "enum"
    // There are multiple result types and result of "read_line"
    // belongs to io::Result. It holds two variants "Ok" and "Err"
    // If its "Ok", expect returns whatever "Ok" holds
    // If its "Err", it crashes and prints the msg passed to it
        .expect("Failed to read line");

    // Print formatted string where "{}" is the placeholder
    println!("You guessed: {}", guess);
}