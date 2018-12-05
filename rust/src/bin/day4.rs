extern crate regex;
extern crate chrono;

use std::io::prelude::*;
use std::io;
use regex::Regex;
use std::collections::HashMap;
use chrono::{NaiveDateTime, Timelike};

enum GuardAction {
    BeginShift(i32),
    FallAsleep,
    WakeUp,
}

struct LogEntry(NaiveDateTime, GuardAction);

/*
[1518-06-27 00:50] wakes up
[1518-09-01 00:03] Guard #1601 begins shift
[1518-03-11 00:03] Guard #1601 begins shift
[1518-10-02 00:24] falls asleep
[1518-04-30 00:00] Guard #2887 begins shift
[1518-11-15 23:57] Guard #1097 begins shift
[1518-11-05 00:19] falls asleep
[1518-07-21 00:03] Guard #1249 begins shift
[1518-08-29 00:59] wakes up
*/

fn main() {
    let stdin = io::stdin();
    let entry_re = Regex::new(r"^\[(.*)\](.*)$").unwrap();
    let shift_re = Regex::new(r"Guard #(\d+) begins shift").unwrap();
    let mut log: Vec<_> = stdin.lock()
        .lines()
        .flatten()
        .map(|l| {
            entry_re.captures(&l).and_then(|captures| {
                let time = captures.get(1)
                    .map(|m| m.as_str())
                    .and_then(|s| {
                        NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M").ok()
                    });

                let action = captures.get(2)
                    .and_then(|m| {
                        match m.as_str().trim() {
                            s if s == "wakes up" => Some(GuardAction::WakeUp),
                            s if s == "falls asleep" => Some(GuardAction::FallAsleep),
                            s => {
                                let s = shift_re.captures(s)
                                    .and_then(|c| c.get(1))
                                    .and_then(|m| m.as_str().parse().ok());
                                Some(GuardAction::BeginShift(s.unwrap()))
                            }
                        }
                    });

                action.and_then(|action| time.map(|time| LogEntry(time, action)))
            })
        })
        .flatten()
        .collect();

    log.sort_unstable_by_key(|e| e.0);

    let mut m: usize = 0;
    let mut sleep_times: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut guard = 0;
    for LogEntry(time, action) in log {
        let new_m = time.minute() as usize;
        match action {
            GuardAction::WakeUp => {
                sleep_times.entry(guard)
                    .or_insert_with(|| vec![0; 60])[m..new_m]
                    .iter_mut().for_each(|t| *t += 1);
            },
            GuardAction::BeginShift(g) => {
                guard = g;
                if guard == 2381 {
                    println!("{} New guard {}", time, g);
                }
            },
            _ => {
                if guard == 2381 {
                    println!("{} Guard sleeping", time);
                }
            }
        };
        m = new_m;
    }

    /*log.iter()
        .filter(|LogEntry(_, a)| {
            if GuardAction::
        }).foreach(|LogEntry(t, a)| {
            println()
        })
        */

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
            println!("Part one: {} * {} = {}", g, minute, g * minute);
        });

    /*log.into_iter.fold((Vec::new(), 0, false), |(shifts, t, asleep), (time, action)| {
        match action {
        }
    });*/
}
