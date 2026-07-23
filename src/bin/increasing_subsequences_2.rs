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

const MOD : u64 = 1000000007;

fn main() {
    let n : usize = read_number();
    let arr : Vec<u64> = read_vector();
    let mut dp : Vec<u64> = vec![1;n];
    for i in 1..n { 
        for j in (0..i).rev(){
            if arr[i] > arr[j]{
                dp[i]+=dp[j];
                dp[i]%=MOD;
            }
        }
    }
    let mut ans = 0;
    for val in dp{
        ans+=val;
        ans%=MOD;
    }
    println!("{ans}");
}