use std::cmp::Ordering;
use std::io;
use rand::prelude::thread_rng;
use rand::Rng;

fn main() {
    let tup = (500, 6.4, 1);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one  = tup.2;

    println!("The value of five_hundred is: {}", five_hundred);
    println!("The value of six_point_four is: {}", six_point_four);
    println!("The value of one is: {}", one);
}
