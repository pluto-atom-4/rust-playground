use std::cmp::Ordering;
use std::io;
use rand::prelude::thread_rng;
use rand::Rng;


fn main() {
    let mut s = String::from("hello");
    s = String::from("ahoy"); // The original string immediately goes out of scope

    println!("{s}, world!");  // print ahoy, world!
}
