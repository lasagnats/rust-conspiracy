use rand::Rng;
use std::cmp::Ordering;
use std::io;
// prelude - a set of std lib items that Rust brings into the scope of every program
// not part of prelude --> import it via 'use'

fn main() {
    println!("Guess the number! Please, input your guess.");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // thread_rng() return a particular generator we will use
    // gen_range() method is defined by the Rng trait that we brought into scope with the `use rand::Rng;` statement
    // range - inclusive on lower & upper bounds

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        // new():  An associated function is a function that’s implemented on a type -->  double colon ::
        // variables are immutable by default; add 'mut' to make mutable

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // without the import: std::io::stdin()
        // &: reference, immutable by default
        // read_line(): appends text to string, but also returns variant of Result enum
        // variant - possible state of an enum
        // Result's variants are Ok and Err; values of Result have methods defined on them e.g. expect()

        // let guess: u32 = guess.trim().parse().expect("Please, type a number!");
        // Shadowing - lets us reuse the var name
        // trim(): eliminates \n (Win: \r\n) that are appended when pressing Enter to submit the entered number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // the underscore, _, is a catchall value - will match all Err values, no matter what information they have inside them
        // continue -> will execute the next iteration of the loop, asking for a new guess

        println!("You guessed: {guess}");

        // with expressions: println!("x = {x} and y + 2 = {}", y + 2);
        // this project - a binary crate, an executable
        // rand - library crate, code meant for being used in other Rust progs, but can't be executed on its own

        //cmp(): takes a reference to whatever to compare with, returns a variant of Ordering
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
