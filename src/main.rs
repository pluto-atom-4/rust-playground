use std::cmp::Ordering;
use std::io;
use rand::prelude::thread_rng;
use rand::Rng;

fn main() {
    let y = {
        let x = 3;
        x + 1    // The value of x is not accessible here, but the value of y is still 4
        // no semicolon at the end of this line, so this expression will be returned as the value of y
    };

    println!("The value of y is {y}");
}
