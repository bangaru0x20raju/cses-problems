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
    let n: u32 = read_number();
    for i in 0..n {
        let index: Vec<u64> = read_vector();
        let mut number: u64 = 0 ;
        if index[0] >= index[1] {
            if index[0] & 1 == 0 {
                number = index[0] * index[0];
                number = number - index[1] + 1;
            } else {
                number = (index[0] - 1) * (index[0] - 1);
                number = number  + index[1];
            }
            
        } else {
            if index[1] & 1 == 1 {
                number  = index[1]* index[1];
                number = number - index[0] + 1;
            }else{
                number = (index[1]-1) * (index[1]-1);
                number = number + index[0];
            }
        }
        println!("{number}");
    }
}
