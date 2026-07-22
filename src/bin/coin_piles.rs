/*
 * Author: Cheerapa Bangaru Raju
 */

#![allow(unused)]

use std::{cmp::max, cmp::min, fmt::Debug, io, str::FromStr};

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

fn solve() {
    let temp: Vec<u64> = read_vector();
    let a: u64 = temp[0];
    let b: u64 = temp[1];
    let total = a + b;
    if total % 3 != 0 {
        println!("NO");
        return;
    }
    let max_ele = max(a, b);
    let min_ele = min(a, b);
    if min_ele*2 >= max_ele { 
        println!("YES");return;
    }
    println!("NO");
}

fn main() {
    let mut tests: i32 = read_number();
    while tests > 0 {
        tests -= 1;
        solve();
    }
}
