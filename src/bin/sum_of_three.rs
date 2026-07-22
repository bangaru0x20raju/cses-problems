/*
 * Author: Cheerapa Bangaru Raju
 */

#![allow(unused)]

use std::{fmt::Debug, io, str::FromStr};

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
    let n_and_k: Vec<u64> = read_vector();
    let mut numbers: Vec<u64> = read_vector();
    let mut numbers_with_index: Vec<(u64, usize)> = Vec::new();
    let mut index: usize = 1;
    for num in numbers {
        numbers_with_index.push((num, index));
        index+=1;
    }
    numbers_with_index.sort();
    for i in 0..n_and_k[0] {
        let mut left = (i as usize) + 1;
        let mut right: usize = (n_and_k[0] as usize) - 1;
        while left < right {
            let temp_sum: u64 = numbers_with_index[i as usize].0 + numbers_with_index[left].0 + numbers_with_index[right].0;
            if temp_sum > n_and_k[1] {
                right -= 1;
            } else if temp_sum < n_and_k[1] {
                left += 1;
            } else {
                println!("{} {} {}", numbers_with_index[i as usize].1, numbers_with_index[left].1 , numbers_with_index[right].1);
                return;
            }
        }
    }
    println!("IMPOSSIBLE");
}
