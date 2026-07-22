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
    let n : usize = read_number();
    let mut arr : Vec<u64> = read_vector();
    arr.insert(0, 0);
    let mut nearest_index : Vec<usize> = Vec::new();
    nearest_index.resize(n+1, 0);
    for i in 1..=n { 
        if arr[i-1] < arr[i]{
            nearest_index[i] = i-1;
        }else{
            let mut prev =  i- 1;
            while arr[prev] >= arr[i]{
                prev = nearest_index[prev];
            }
            nearest_index[i] = prev;
        }
    }
    
    for index in 1..=n {
        print!("{} ", nearest_index[index]);
    }

    println!();
}
