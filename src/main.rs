use std::cmp::Ordering;
use std::io;
use rand::prelude::thread_rng;
use rand::Rng;


fn main() {
    let number = 3;
    if number { // This will cause a compile-time error because `number` is an integer, not a boolean.
        println!("condition was three");
    }
}
