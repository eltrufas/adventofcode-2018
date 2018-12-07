#![feature(slice_patterns)]
use std::io::prelude::*;
use std::io;

fn are_opposite(a: char, b: char) -> bool {
    a.eq_ignore_ascii_case(&b) && b.is_uppercase() != a.is_uppercase()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let len = input.chars().fold(Vec::new(), |mut stack, c| {
        match &stack[..] {
            [head, tail..] if are_opposite(*head, c) => tail.to_vec(),
            s => {
                let mut v = s.to_vec();
                v.push(c);
                v
            },
        }
    }).iter().count();

    println!("{}", len);
}
