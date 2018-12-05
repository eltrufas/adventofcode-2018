use std::io::prelude::*;
use std::io;
use std::collections::HashMap;

#[derive(Default)]
struct Rect {
    id: i32,
    pos: Vec<i32>,
    dim: Vec<i32>,
}

fn main() {
    let stdin = io::stdin();
    let mut map = HashMap::new();
    let rects: Vec<Rect> = stdin.lock()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut r: Rect = Default::default();
            let line: Vec<&str> = line.split(' ').collect();
            match &line[..] {
                &[id, _, coords, dim] => {
                    r.id = id[1..].parse().unwrap();
                    r.pos = coords[..coords.len() - 1].split(',')
                        .map(|s| s.parse().unwrap())
                        .collect();
                    r.dim = dim.split('x')
                        .map(|s| s.parse().unwrap())
                        .collect();
                }
                _ => {}
            }
            r
        })
        .collect();

    for r in rects {
        let (x, y, w, h) = (r.pos[0], r.pos[1], r.dim[0], r.dim[1]);

        for i in x..x + w {
            for j in y..y + h {
                let space = map.entry((i, j)).or_insert(0);
                *space += 1;
            }
        }
    }

    let overlap = map.values().filter(|v| **v > 1).count();
    println!("{}", overlap);
}
