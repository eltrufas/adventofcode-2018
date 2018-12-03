extern crate itertools;
use itertools::Itertools;
use std::io::prelude::*;
use std::io;

fn main() {
    let stdin = io::stdin();

    stdin.lock().lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>()
        .into_iter()
        .tuple_combinations()
        .map(|(a, b)| {
            let matching = a.chars().zip(b.chars())
                .filter_map(|(a, b)| {
                    if a == b {
                        Some(a)
                    } else {
                        None
                    }
                })
                .collect::<String>();

            (matching, a.len() as i32)
        })
        .filter(|(cs, l)| cs.len() as i32 - *l == -1)
        .foreach(|(cs, _)| {
            println!("SEQUENCE FOUND: {}", cs);
        });
}
