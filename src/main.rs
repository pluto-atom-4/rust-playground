use std::cmp::Ordering;
use std::io;
use rand::prelude::thread_rng;
use rand::Rng;


fn five() -> i32 {
    5 // This is an expression, not a statement, so it will return a value.
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}
