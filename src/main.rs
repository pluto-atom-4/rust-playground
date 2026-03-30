use std::cmp::Ordering;
use std::io;
use rand::prelude::thread_rng;
use rand::Rng;


fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{s}");
}
