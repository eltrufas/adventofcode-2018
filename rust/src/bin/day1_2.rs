use std::io::prelude::*;
use std::io;
use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();
    let mut fq = 0;
    let dfs: Vec<i32> = stdin.lock()
        .lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .collect();

    let mut seen = HashSet::new();

    seen.insert(fq);
    for df in dfs.into_iter().cycle() {
        fq += df;
        if seen.contains(&fq) {
            println!("FREQUENCY FOUND: {}", fq);
            break;
        } else {
            seen.insert(fq);
        }
    }

}
