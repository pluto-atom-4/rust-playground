use std::cmp::Ordering;
use std::io;
use rand::prelude::thread_rng;
use rand::Rng;


fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    
    println!("{s1}, world!");  // Rust consider `s1` as no longer valid.
}
