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
    let line = read_string();
    let line = line.trim();
    let mut letters_count = [0 as u32;26];
    for ch in line.chars(){
        let index = (ch as usize) - ('A' as usize);
        letters_count[index]+=1;
    }
    let mut alread_one = false;
    let mut index = 0;
    let mut palindrom_string = String::new();
    let mut odd_index = -1;
    let mut odd_index_count = 0;
    for count in letters_count.iter(){
        if (count&1 == 1) && alread_one { 
            println!("NO SOLUTION");return;
        }
        if count& 1 == 1 { 
            alread_one = true;
            odd_index = index;
            odd_index_count = *count;
        }else{
            let half = count/2;
            let mut temp_string = String::new();
            for i in 0..half{
                temp_string.push((('A' as u8)+ (index as u8)) as char);
            }
            palindrom_string.push_str(&temp_string);
        }
        index+=1;
    }
    if odd_index != -1{
        let temp_char = (('A' as u8)+ (odd_index as u8)) as char;
        let mut temp_string = String::new();
        for _ in 0..odd_index_count { 
            temp_string.push(temp_char);
        }
        println!("{palindrom_string}{temp_string}{}", palindrom_string.chars().rev().collect::<String>())
    }else{
        println!("{palindrom_string}{}", palindrom_string.chars().rev().collect::<String>())
    }
    
}