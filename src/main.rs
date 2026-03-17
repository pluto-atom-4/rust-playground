use std::cmp::Ordering;
use std::io;
use rand::prelude::thread_rng;
use rand::Rng;


fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
