/*
 * Author: Cheerapa Bangaru Raju
 */

#![allow(unused)]

use std::{collections::HashMap, fmt::Debug, io, str::FromStr};

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
    let n_and_k : Vec<i64> = read_vector();
    let n : u32 = n_and_k[0] as u32;
    let k : i64 = n_and_k[1];
    let arr : Vec<i64> = read_vector();

    let mut values_map : HashMap<i64, u64> = HashMap::new();
    values_map.insert(0, 1);
    let mut array_sum : i64 = 0;
    let mut ans = 0 as u64;
    for ele in arr {
        array_sum+=ele;
        let temp = array_sum - k;
        if values_map.contains_key(&temp) {
            ans+=values_map.get(&temp).unwrap();
        }
        if let Some(x) = values_map.get_mut(&array_sum){
            *x = *x+1;
        }
        else{
            values_map.insert(array_sum, 1);
        }
    }
    println!("{ans}");
    
}