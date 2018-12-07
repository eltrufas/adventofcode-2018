#![feature(slice_patterns)]
use std::io::prelude::*;
use std::io;

fn are_opposite(a: char, b: char) -> bool {
    a.eq_ignore_ascii_case(&b) && b.is_uppercase() != a.is_uppercase()
}

fn reacted_len(input: &str) -> usize {
    input.chars().fold(Vec::new(), |stack, c| {
        match &stack[..] {
            [head, tail..] if are_opposite(*head, c) => tail.to_vec(),
            s => {
                let mut v = s.to_vec();
                v.push(c);
                v
            },
        }
    }).iter().count()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut letters: Vec<_>= input.chars().collect();
    letters.sort_unstable();
    letters.dedup();

    let part_one = reacted_len(&input);

    let part_two = letters.iter().filter(|c| c.is_uppercase()).map(|c| {
        let input: String = input.chars().filter(|d| !d.eq_ignore_ascii_case(&c)).collect();
        reacted_len(&input)
    }).max();


    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two.unwrap());
}
