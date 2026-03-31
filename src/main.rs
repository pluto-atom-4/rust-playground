// use std::cmp::Ordering;
// use std::io;
// use rand::prelude::thread_rng;
// use rand::Rng;


fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}");
}

// Rust does let us return multiple values using a tuple.
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
