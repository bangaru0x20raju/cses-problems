/*
 * Author: Cheerapa Bangaru Raju
 */

#![allow(unused)]

use std::{cmp::min, cmp::max, fmt::Debug, io, str::FromStr};

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
    
    let n : i32 = read_number();
    let mut arr : Vec<u64> = read_vector();
    arr.sort();
    let mut total_cost: u64 = 0;
    let mut middle_ele = arr[(n/2) as usize];
    for ele in arr.iter(){
        total_cost += (max(*ele, middle_ele)- min(*ele, middle_ele));
    }
    if n&1 == 0{
        let mut total_cost_2 = 0 as u64;
        middle_ele = arr[((n+1)/2) as usize];
        for ele in arr.iter(){
            total_cost_2 += (max(*ele, middle_ele)- min(*ele, middle_ele));
        }
        total_cost = min(total_cost, total_cost_2);
    }
    println!("{total_cost}");
    
}
