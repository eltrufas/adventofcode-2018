use std::io::prelude::*;
use std::io;

fn main() {
    let stdin = io::stdin();
    let result = stdin.lock()
        .lines()
        .map(|l| l.unwrap().parse::<i32>())
        .fold(0, |acc, n| acc + n.unwrap());

    println!("{}", result);
}
