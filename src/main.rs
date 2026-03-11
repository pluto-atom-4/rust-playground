use std::cmp::Ordering;
use std::io;
use rand::prelude::thread_rng;
use rand::Rng;

fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    print!("The value of y is: {}", y);
}
