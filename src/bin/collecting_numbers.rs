/*
 * Author: Cheerapa Bangaru Raju
 */

#![allow(unused)]

use std::{cmp::Ordering, collections::{BTreeSet, HashMap}, fmt::Debug, io, str::FromStr};

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
    let v : Vec<i32> = read_vector();
    let mut my_map: HashMap<i32, i32> = HashMap::new();
    let mut index = 0;
    for ele in v { 
        my_map.insert(ele, index);
        index+=1;
    }
    my_map.insert(0, -1);
    let mut rounds = 1;
    for ele in 1..=n { 
        let temp = ele-1;
        if my_map.get(&ele).unwrap() < my_map.get(&temp).unwrap(){
            rounds+=1;
        }
    }
    println!("{rounds}");
    
}