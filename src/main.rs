use std::cmp::Ordering;
use std::io;
use rand::prelude::thread_rng;
use rand::Rng;

fn main() {
    println!("Hello, world!");

    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
