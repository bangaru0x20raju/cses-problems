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
    let n_and_k : Vec<u64> = read_vector();
    let n : usize = n_and_k[0] as usize;
    let k : u64 = n_and_k[1];
    let mut arr : Vec<u64> = read_vector();
    if n == 1 {
        if arr[0] == k {
            println!("1");
        }else{
            println!("0");
        }
        return;
    }
    let mut left : usize = 0;
    let mut right: usize = 1;
    let mut sub_array_sum: u64 = arr[0];
    let mut sub_arrays_count = 0;
    while right < n{
        if sub_array_sum == k {
            sub_arrays_count+=1;
            sub_array_sum-=arr[left];
            sub_array_sum+=arr[right];
            left+=1;
            right+=1;
        }else if sub_array_sum < k {
            sub_array_sum+=arr[right];
            right+=1;
        }else{
            sub_array_sum-=arr[left];
            left+=1;
        }

    }
    while sub_array_sum > k {
        sub_array_sum-=arr[left];
        left+=1;
    }
    if sub_array_sum == k{
        sub_arrays_count+=1;
    }
    println!("{sub_arrays_count}");
}
