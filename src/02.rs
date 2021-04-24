/*
    Rocketman Rust Learning
    2021-04-24
*/

use std::io;
use std::cmp::Ordering;
use rand::Rng;

extern crate colored;
use colored::*;

fn main() {
    println!("Hello, world!");

    let number = 5;          // the first binding is created using the name "number"
    let number = number + 5; // a different binding shadows the name "number"
    let number = number * 2; // again, a new binding is created
    println!("The number is: {}", number);

    let number: u32 = "42".parse().expect("Not a number!");
    println!("The number is: {}", number);

    // Addition
    println!("1 + 2 = {}", 1u32 + 2);
    // Subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // Integer Division
    println!("9 / 2 = {}", 9u32 / 2);
    // Float Division
    println!("9 / 2 = {}", 9.0f64 / 2.0f64);
    // Multiplication
    println!("3 * 6 = {}", 3 * 6);

    // Colored String Print
    let mut message = String::from("Rocket");
    message.push_str(" Engine");
    let eye = String::from("o");
    println!("{}: message", message.bright_yellow().underline().on_purple());
    println!("{}:{}", eye.bright_red().underline(), eye.bright_red().underline());

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                break;
            },
            Ordering::Greater => {
                println!("Too big!");
                break;
            },
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }

    println!("Program End");
}
