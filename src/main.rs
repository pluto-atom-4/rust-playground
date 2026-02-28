use std::cmp::Ordering;
use std::io;
use rand::prelude::thread_rng;
use rand::Rng;

fn main() {
    let x = 5;

    let x = x + 1;

    // shadowing allows us to reuse the same variable name, but it creates a new variable that shadows the previous one.
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
println!("The value of x is: {}", x);

}