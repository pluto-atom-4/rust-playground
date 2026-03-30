use std::cmp::Ordering;
use std::io;
use rand::prelude::thread_rng;
use rand::Rng;


fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // The heap data does get copied

    println!("s1 = {s1}, s2 = {s2}");
}
