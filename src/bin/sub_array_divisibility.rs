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
    let n: usize = read_number();
    let arr: Vec<i64> = read_vector();
    let mut values_map: HashMap<i64, u64> = HashMap::new();
    let mut ans: u64 = 0;
    let mut sub_array_sum: i64 = 0;
    let k = n as i64;
    for ele in arr {
        sub_array_sum+=ele;
        sub_array_sum = ((sub_array_sum%k)+k)%k;
        if sub_array_sum == 0{
            ans+=1;
        }
        if values_map.contains_key(&sub_array_sum){
            ans+=values_map.get(&sub_array_sum).unwrap();
        }
        if let Some(x) = values_map.get_mut(&sub_array_sum){
            *x = *x+1;
        }else{
            values_map.insert(sub_array_sum, 1);
        }
        
    }
    println!("{ans}");
}
