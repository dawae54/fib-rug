use rug::{Complete, Integer, integer::IntegerExt64};
use std::env;
fn main() {
    let num: u64 = env::args()
        .nth(1)
        .expect("Please provide a number as an argument")
        .parse()
        .expect("Invalid number provided");
    let fib = Integer::fibonacci_64(num).complete();
    println!("{}", fib);
}
