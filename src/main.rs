use std::cmp::Ordering;
use std::io;
use rand::prelude::thread_rng;
use rand::Rng;


fn main() {
    let mut count = 0;
    // The 'countring_up label allows us to break out of the outer loop from within the inner loop.
    'countring_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'countring_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
