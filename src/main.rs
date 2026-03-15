use std::cmp::Ordering;
use std::io;
use rand::prelude::thread_rng;
use rand::Rng;

fn main() {
    let x = (let y = 6); // error as a statement does not return values,
    // whereas C and Ruby, where assignment returns the value of the assignment :)
}
