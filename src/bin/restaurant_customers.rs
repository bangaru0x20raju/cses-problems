/*
 * Author: Cheerapa Bangaru Raju
 */

#![allow(unused)]

use std::{cmp::max, collections::BTreeSet, fmt::Debug, i64, io, str::FromStr, u64};

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
    let n : i32 = read_number();
    let mut interval_sets : BTreeSet<(u64, i8,i32)> = BTreeSet::new();
    for i in 1..=n{
        let temp : Vec<u64> = read_vector();
        interval_sets.insert((temp[0], 1, i));
        interval_sets.insert((temp[1], -1, i));
    }
    let mut best_count : i64 = i64::MIN;
    let mut temp_count : i64 = 0;
    
    for interval_set in interval_sets.iter(){
        temp_count+=(interval_set.1 as i64);
        best_count = max(best_count, temp_count);
    }
    println!("{best_count}");
}