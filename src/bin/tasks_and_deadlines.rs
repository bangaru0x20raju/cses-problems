/*
 * Author: Cheerapa Bangaru Raju
 */

#![allow(unused)]

use std::{collections::BTreeSet, fmt::Debug, io, str::FromStr};

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

fn main() {
    let n : usize = read_number();
    let mut dd : Vec<(u64, u64)> = Vec::new();
    for i in 0..n {
        let temp_dd: Vec<u64> = read_vector();
        dd.push((temp_dd[0], temp_dd[1]));
    }
    dd.sort();

    let mut reward : i64 = 0;
    let mut current_time : u64 = 0;
    for temp_dd in dd { 
        current_time+=(temp_dd.0);
        reward+=(temp_dd.1 as i64 - current_time as i64);
    }
    println!("{reward}");
}