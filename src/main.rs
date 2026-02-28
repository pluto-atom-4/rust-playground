use std::cmp::Ordering;
use std::io;
use rand::prelude::thread_rng;
use rand::Rng;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 7;
    println!("The value of x is: {}", x);
}