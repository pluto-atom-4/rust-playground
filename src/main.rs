use std::cmp::Ordering;
use std::io;
use rand::prelude::thread_rng;
use rand::Rng;


fn main() {
    let condition = true;
    let number = if condition { 5 } else { "six" }; // this will cause a compile error because the types of the two branches are different

    println!("The value of number is: {number}");
}
