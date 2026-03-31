// use std::cmp::Ordering;
// use std::io;
// use rand::prelude::thread_rng;
// use rand::Rng;


fn main() {
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}"); // x is still valid and wasn't moved into y.
}
