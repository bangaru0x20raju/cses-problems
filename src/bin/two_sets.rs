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
    let n: u64 = read_number();
    let total_sum = (n * (n + 1)) / 2;
    if total_sum & 1 == 1 {
        println!("NO");
    } else {
        println!("YES");
        if n & 1 == 1 {
            let set1_n = 2 + (n - 3) / 2;
            println!("{set1_n}");
            print!("1 2");
            let mut left: u32 = 4;
            let mut right: u32 = n as u32;
            while (left < right) {
                print!(" {left} {right}");
                left += 2;
                right -= 2;
            }
            println!();
            let set2_n = 1 + (n - 3) / 2;
            println!("{set2_n}");
            print!("3");
            left = 5;
            right = n as u32 - 1;
            while (left < right) {
                print!(" {left} {right}");
                left += 2;
                right -= 2;
            }
            println!();
        } else {
            println!("{}", n / 2);
            let mut left = 1 as u32;
            let mut right = n as u32;
            while (left < right) {
                print!("{left} {right} ");
                left += 2;
                right -= 2;
            }
            println!();
            println!("{}", n / 2);
            left = 2;
            right = n as u32 - 1;
            while (left < right) {
                print!("{left} {right} ");
                left += 2;
                right -= 2;
            }
            println!();
        }
    }
}
