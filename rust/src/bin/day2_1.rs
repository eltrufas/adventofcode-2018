use std::io::prelude::*;
use std::io;

fn main() {
    let stdin = io::stdin();
    let (twos, threes) = stdin.lock()
        .lines()
        .map(|id| {
            let mut arr = [0; 26];
            for c in id.unwrap().chars() {
                let idx = c as usize - 'a' as usize;
                arr[idx] += 1;
            }

            arr
        })
        .fold((0, 0), |(twos, threes), map| {
            let twos = twos + if map.iter().any(|i| *i == 2) { 1 } else { 0 };
            let threes = threes + if map.iter().any(|i| *i == 3) { 1 } else { 0 };
            (twos, threes)
        });

    println!("CHECKSUM: {}", twos * threes);
}
