/*
 * Author: Cheerapa Bangaru Raju
 */

#![allow(unused)]

use std::{fmt::Debug, io, ptr::read, str::FromStr};

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

fn brute_force(current_num : u16, set1_sum : u32, set2_sum : u32) -> u64{
    println!("{}, {}", current_num, set1_sum);
    if current_num == 0 {
        if set1_sum == set2_sum { 
            return 1;
        }else{
            return 0;
        }
    }
    return (brute_force(current_num-1, set1_sum+ current_num as u32, set2_sum )%MOD + brute_force(current_num-1, set1_sum, set2_sum+ current_num as u32)%MOD)%MOD
}

fn main() {
    let n : usize = read_number();
    let mut half_sum: usize = (n * (n+1))/2;
    if half_sum & 1 == 1{
        println!("0");
        return;
    }
    half_sum/=2;
    let mut dp : Vec<u64> = Vec::new();
    dp.resize(half_sum+1, 0);
    dp[0] = 1;
    for i in 1..n {
        for j in (1..=half_sum).rev(){
            if j>= i {
                dp[j]+=dp[j-i];
                dp[j]%=MOD;
            }
        }
    }
    println!("{}",dp[half_sum]);
}