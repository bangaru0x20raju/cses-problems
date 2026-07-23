/*
 * Author: Cheerapa Bangaru Raju
 */

#![allow(unused)]

use std::{cmp::{max, min}, fmt::Debug, io, ops::Sub, str::FromStr, u16::MAX};

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
    let temp_input: Vec<usize> = read_vector();
    let length = max(temp_input[0], temp_input[1]);
    let width = min(temp_input[0], temp_input[1]);
    if width == length {
        println!("0");
        return;
    }
    let mut dp: Vec<Vec<u16>> = vec![vec![u16::MAX; width + 1]; length + 1];
    for i in 1..=length {
        for j in 1..=width {
            if i == j {
                dp[i][j] = 0;
            } else {
                for k in 1..i {
                    dp[i][j] = min(dp[i][j], dp[i - k][j] + 1 + dp[k][j]);
                }
                for k in 1..j {
                    dp[i][j] = min(dp[i][j], dp[i][j - k] + 1 + dp[i][k]);
                }
            }
        }
    }
    println!("{}", dp[length][width]);
}