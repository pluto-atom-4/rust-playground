use std::cmp::Ordering;
use std::io;
use rand::prelude::thread_rng;
use rand::Rng;


fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 }; // no semicolon at the end of the line, otherwise it will be treated as a statement and not an expression

    println!("The value of number is: {number}");
}
