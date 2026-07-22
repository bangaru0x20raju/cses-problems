/*
 * Author: Cheerapa Bangaru Raju
 */

#![allow(unused)]

use std::{cmp::max, fmt::Debug, io, num, str::FromStr};

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
    let n: usize = read_number();
    let numbers: Vec<u64> = read_vector();
    let mut moves: u64 = 0;
    let mut prev = numbers[0];
    for i in 1..n {
        if numbers[i] < prev {
            let required = (prev - numbers[i]);
            moves += required;
        } else {
            prev = numbers[i];
        }
    }
    println!("{moves}");
}
