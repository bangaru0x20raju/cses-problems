/*
 * Author: Cheerapa Bangaru Raju
 */

#![allow(unused)]

use std::{cmp::max, collections::{BTreeSet, HashMap}, fmt::Debug, io, str::FromStr};

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
    let mut best_len = i32::MIN;
    let mut left = 0;
    let mut index_map : HashMap<u64, i32> = HashMap::new();
    let mut index = 0;
    while index<n {
        if index_map.contains_key(&(arr[index as usize])) {
            let found_index = *(index_map.get(&(arr[index as usize])).unwrap());
            while left <= found_index{
                index_map.remove(&arr[left as usize]);
                left+=1;
            }
        }
        index_map.insert(arr[index as usize], index);
        best_len = max(best_len, index_map.len() as i32);
        index+=1;
    }
    println!("{best_len}");
}