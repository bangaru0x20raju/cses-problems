/*
 * Author: Cheerapa Bangaru Raju
 */

#![allow(unused)]

use std::{cmp::max, collections::BTreeSet, fmt::Debug, io, str::FromStr, usize};

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

// Brute-force approach
fn find_long_subseq(curr_index : usize, n : usize, current_len : usize, arr : &Vec<u64>) -> usize{
    if curr_index>= n { 
        return current_len;
    }
    let mut ans = current_len;
    for i in curr_index+1..n {
        if arr.get(i) > arr.get(curr_index){
            ans = max(ans, find_long_subseq(i, n, current_len+1, arr));
        }
    }
    ans
}

fn main() {
    let n : usize = read_number();
    let arr : Vec<u64> = read_vector();
    let mut sub_seq: BTreeSet<u64> = BTreeSet::new();
    sub_seq.insert(arr[0]);
    for i in 1..n { 
        let temp = sub_seq.range(arr[i]..).next();
        if let Some(val) = temp{
            let real_val = *val;
            if real_val != arr[i]{
                sub_seq.remove(&real_val);
                sub_seq.insert(arr[i]);
            }
        }else{
            sub_seq.insert(arr[i]);
        }
    }
    println!("{}", sub_seq.len());
}