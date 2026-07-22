/*
 * Author: Cheerapa Bangaru Raju
 */

#![allow(unused)]

use std::{
    cmp::{Eq, Ord, PartialEq, PartialOrd, max, min},
    fmt::Debug,
    io,
    str::FromStr,
    sync::atomic::Ordering,
};

fn read_number<T>() -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read from stdin");
    number.trim().parse().expect("Failed to parse the input")
}

fn read_string() -> String {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read the string.");
    line
}

fn read_vector<T>() -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read the vector.");
    line.trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse element"))
        .collect()
}

#[derive(Debug)]
struct IntervalRange {
    start_time: u64,
    end_time: u64,
    index: usize,
}

impl PartialEq for IntervalRange {
    fn eq(&self, rhs: &IntervalRange) -> bool {
        self.start_time.eq(&rhs.start_time) && self.end_time.eq(&rhs.end_time)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Eq for IntervalRange {}

impl PartialOrd for IntervalRange {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for IntervalRange {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let start_time_comp = self.start_time.cmp(&other.start_time);
        match start_time_comp {
            std::cmp::Ordering::Equal => {
                let end_time_comp = self.end_time.cmp(&other.end_time);
                match end_time_comp {
                    std::cmp::Ordering::Greater => std::cmp::Ordering::Less,
                    std::cmp::Ordering::Less => std::cmp::Ordering::Greater,
                    _ => self.index.cmp(&other.index),
                }
            }
            _ => start_time_comp,
        }
    }
}

fn main() {
    let mut n: usize = read_number();
    let mut index: usize = 0;
    let mut intervals: Vec<IntervalRange> = Vec::new();
    while index < n {
        let interval: Vec<u64> = read_vector();
        let interval_range = IntervalRange {
            start_time: interval[0],
            end_time: interval[1],
            index: index,
        };
        intervals.push(interval_range);
        index += 1;
    }
    intervals.sort();
    let mut it_contains: Vec<u8> = vec![0; n];
    let mut contains_it: Vec<u8> = vec![0; n];

    let mut early_end = u64::MAX;
    for i in (0..n).rev() {
        if intervals[i].end_time >= early_end {
            it_contains[intervals[i].index] = 1;
        }
        early_end = min(early_end, intervals[i].end_time);
    }
    let mut last_end = u64::MIN;
    for i in 0..n {
        if intervals[i].end_time <= last_end {
            contains_it[intervals[i].index] = 1;
        }
        last_end = max(last_end, intervals[i].end_time);
    }

    for i in 0..n {
        print!("{}{}", it_contains[i], if i == n - 1 { "" } else { " " });
    }
    println!();
    for i in 0..n {
        print!("{}{}", contains_it[i], if i == n - 1 { "" } else { " " });
    }
    println!();
}
