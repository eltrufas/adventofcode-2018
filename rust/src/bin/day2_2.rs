extern crate itertools;
use itertools::Itertools;
use std::io::prelude::*;
use std::io;

fn main() {
    let stdin = io::stdin();

    stdin.lock().lines()
        .collect::<io::Result<Vec<String>>>()
        .unwrap()
        .into_iter()
        .tuple_combinations()
        .filter_map(|(a, b)| {
            let matching = a.chars().zip(b.chars())
                .filter_map(|(a, b)| Some(a).filter(|a| *a == b))
                .collect::<String>();

            Some(matching).filter(|m| m.len() as i32 - a.len() as i32 == -1)
        })
        .foreach(|s| {
            println!("SEQUENCE FOUND: {}", s);
        });
}
