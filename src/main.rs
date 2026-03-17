use std::cmp::Ordering;
use std::io;
use rand::prelude::thread_rng;
use rand::Rng;


fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LITOFF!!!")
}
