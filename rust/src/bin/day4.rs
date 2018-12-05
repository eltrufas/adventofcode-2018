extern crate regex;

use std::io::prelude::*;
use std::io;
use regex::Regex;
use std::collections::HashMap;

enum GuardAction {
    BeginShift(i32),
    FallAsleep,
    WakeUp,
}

fn main() {
    let stdin = io::stdin();
    let entry_re = Regex::new(r"^\[.*:(\d+)\](.*)$").unwrap();
    let shift_re = Regex::new(r"Guard #(\d+) begins shift").unwrap();
    let mut lines: Vec<_> = stdin.lock()
        .lines()
        .flatten()
        .collect();

    lines.sort_unstable();

    let (sleep_times, _, _)= lines
        .iter()
        .map(|l| {
            entry_re.captures(&l).and_then(|captures| {
                let time = captures.get(1)
                    .map(|m| m.as_str())
                    .and_then(|s| {
                        s.parse().ok()
                    });

                let action = captures.get(2)
                    .and_then(|m| {
                        match m.as_str().trim() {
                            s if s == "wakes up" => Some(GuardAction::WakeUp),
                            s if s == "falls asleep" => Some(GuardAction::FallAsleep),
                            s => shift_re.captures(s)
                                .and_then(|c| c.get(1))
                                .and_then(|m| m.as_str().parse().ok())
                                .map(|s| GuardAction::BeginShift(s)),
                        }
                    });

                action.and_then(|action| time.map(|time| (time, action)))
            })
        })
        .flatten()
        .fold((HashMap::new(), 0, 0), |(mut sleep_times, m, guard), (time, action)| {
            match action {
                GuardAction::WakeUp => {
                    sleep_times.entry(guard)
                        .or_insert_with(|| vec![0; 60])[m..time]
                        .iter_mut().for_each(|t| *t += 1);

                    (sleep_times, time, guard)
                },
                GuardAction::BeginShift(g) => (sleep_times, time, g),
                GuardAction::FallAsleep => (sleep_times, time, guard)
            }
        });

    sleep_times.iter().max_by_key(|(_, t)| -> i32 { t.iter().sum() }).map(|(g, t)| {
        let (minute, _) = t.iter().enumerate().max_by_key(|(_, t)| t.clone()).unwrap();
        let minute = minute as i32;
        println!("Part one: {} * {} = {}", g, minute, g * minute);
    });

    sleep_times.iter()
        .map(|(g, t)| {
            t.iter().enumerate().max_by_key(|(_, t)| t.clone()).map(|(i, t)| (g, i, t))
        })
        .flatten()
        .max_by_key(|(_, _, t)| t.clone())
        .map(|(g, i, _t)| {
            let minute = i as i32;
            println!("Part two: {} * {} = {}", g, minute, g * minute);
        });
}
