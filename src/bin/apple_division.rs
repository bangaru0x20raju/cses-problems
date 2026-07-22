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

fn main() {
    let n: i32 = read_number();
    let mut values: Vec<u64> = read_vector();
    let total_sum: u64 = values.iter().sum();
    let mut min_difference: u64 = u64::MAX;
    for bit in 1..(1 << n) {
        let mut temp_sum: u64 = 0;
        let mut index: usize = 0;
        let mut bit_copy = bit;
        while bit_copy > 0 {
            if bit_copy & 1 == 1 {
                temp_sum += values[index];
            }
            index += 1;
            bit_copy /= 2;
        }
        let res = max(total_sum - temp_sum, temp_sum) - min(total_sum - temp_sum, temp_sum);
        if res < min_difference {
            min_difference = res;
        }
    }
    println!("{min_difference}");
}
