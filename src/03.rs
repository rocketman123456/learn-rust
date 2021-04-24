/*
    Rocketman Rust Learning
    2021-04-24
*/

//use std::io;
//use rand::Rng;

extern crate colored;
use colored::*;

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn plus_one(x: i32) -> i32 {
    // return value
    x + 1
}

fn main() {
    let tuple = ("hello", 5, 'c');
    let (x, y, z) = tuple;

    assert_eq!(tuple.0, "hello");
    assert_eq!(tuple.1, 5);
    assert_eq!(tuple.2, 'c');

    let a = [1, 2, 3, 4, 5];
    let a = [3; 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
    let second = plus_one(second);

    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];

    another_function(first, second);

    // I’m feeling lucky today
    let lucky_number = 7;

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index = index + 1;
    }

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}