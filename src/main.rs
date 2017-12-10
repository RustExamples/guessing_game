// Let rust know that we use external dependency
extern crate rand;

// Rust includes only a limited scope during "prelude" phase
// Import "io" from "std" library
use std::io;
// Rng is a "trait" here, has methods that random number
// generators implement
use rand::Rng;
// Ordering is an enum with values "Less", "Greater" & "Equal"
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

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

    // "guess" is a shadow of original rather than different variable
    // trim any space or new line 
    // parse the input as number and in this case "u32"
    // this makes rust infers "secret_number" as "u32"
    let guess: u32 = guess.trim().parse()
                        .expect("Please type a number!");

    // Print formatted string where "{}" is the placeholder
    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}