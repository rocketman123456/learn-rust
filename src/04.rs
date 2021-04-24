/*
    Rocketman Rust Learning
    2021-04-24
*/

extern crate colored;
use colored::*;

fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1.red(), s2.yellow());

    let s = String::from("hello");  // s 进入作用域

    takes_ownership(s);             // s 的值移动到函数里 ...
                                    // ... 所以到这里不再有效

    let x = 5;                      // x 进入作用域

    makes_copy(x);                  // x 应该移动函数里，
                                    // 但 i32 是 Copy 的，所以在后面可继续使用 x
    println!("{}", x);

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);

    let len = calculate_length_ref(&s2);
    println!("The length of '{}' is {}.", s2, len);

    let mut s = String::from("hello");
    println!("Before change '{}'.", s);
    change(&mut s);
    println!("After change '{}'.", s);

    //let mut s = String::from("hello");
    //let r1 = &s; // 没问题
    //let r2 = &s; // 没问题
    //let r3 = &mut s; // 大问题

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    let mut s = String::from("hello world");
    let word = first_word(&s);
    //s.clear(); // 错误!
    println!("the first word is: {}", word);
}

// Slice
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度
    (s, length)
}

fn calculate_length_ref(s: &String) -> usize {
    s.len()
}

fn gives_ownership() -> String {            // gives_ownership 将返回值移动给
                                            // 调用它的函数
    let some_string = String::from("hello"); // some_string 进入作用域.
    some_string                              // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域
    a_string  // 返回 a_string 并移出给调用的函数
}

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作
