/*
 * Author: Cheerapa Bangaru Raju
 */

#![allow(unused)]

use std::{cmp::max, fmt::Debug, i64, io, str::FromStr};

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
    let n: i32 = read_number();
    let arr: Vec<i64> = read_vector();
    let mut best_sum: i64 = i64::MIN;
    let mut temp_sum: i64 = 0;
    for ele in arr {
        temp_sum = max(temp_sum + ele, ele);
        best_sum = max(best_sum, temp_sum);
    }
    println!("{best_sum}");
}
