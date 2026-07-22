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
    let mut n : u32 = read_number();
    let mut to_remove : Vec<u32> = (1..=n).collect();
    let mut remove = false;
    while to_remove.len() > 0{
        let mut temp : Vec<u32> = Vec::new();

        for ele in to_remove{
            if remove {
                print!("{ele} ");
                remove = false;
            }else{
                temp.push(ele);
                remove = true;
            }
        }
        to_remove = temp;
    }
}