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
    let n_and_x : Vec<u64> = read_vector();
    let mut weights : Vec<u64> = read_vector();
    let n : usize = n_and_x[0] as usize;
    weights.sort();
    let mut ans: u8 = 0;
    let mut left_idx: usize = 0;
    let mut right_idx: usize = n-1;
    let mut sum: u64 = weights[right_idx];
    while(left_idx <= right_idx){
        if left_idx == right_idx{
            ans+=1;
            break;
        }
        if sum + weights[left_idx] > n_and_x[1]{
            ans+=1;
            right_idx-= 1;
            sum = weights[right_idx];
        }else if sum+weights[left_idx] == n_and_x[1]{
            ans+=1;
            left_idx+=1;
            right_idx-=1;
            sum = weights[right_idx];
        }else{
            sum+=weights[left_idx];
            left_idx+=1;
        }
    }
    println!("{ans}");
}