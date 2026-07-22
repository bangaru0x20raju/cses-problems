/*
 * Author: Cheerapa Bangaru Raju
 */

#![allow(unused)]

use std::{cmp::max, fmt::Debug, io, str::FromStr};

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

fn brute_force(left_index : usize, right_index: usize, turn : u8, p1_score : u64, p2_score : u64, numbers : &Vec<u64>) -> (u64, u64) {
    if left_index > right_index {
        return (p1_score, p2_score);
    }
    if turn == 1{
        let temp_p1_score = max(brute_force(left_index+1, right_index, 0, p1_score+numbers[left_index], p2_score, numbers).0 ,
            brute_force(left_index, right_index-1, 0, p1_score+numbers[right_index], p2_score, numbers).0);
        return (temp_p1_score, p2_score);
    }else{
        let temp_p2_score = max(brute_force(left_index+1, right_index, 1, p1_score, p2_score+numbers[left_index], numbers).1 ,
            brute_force(left_index, right_index-1, 1, p1_score, p2_score+numbers[right_index], numbers).1);
        return (p1_score, temp_p2_score);
    }
}

fn main() {
    let n : usize = read_number();
    let mut arr : Vec<u64> = read_vector();
    arr.insert(0, 0);
    let final_score = brute_force(1, n, 1, 0, 0, &arr);
    println!("{}", final_score.0);
}