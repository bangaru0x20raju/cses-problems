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
    let n_q : Vec<usize> = read_vector();
    let line : Vec<usize> = read_vector();
    let mut binary_lifting: Vec<Vec<usize>> = vec![vec![0;32]; n_q[0]+1];
    let mut index = 1;
    
    for val in line{
        binary_lifting[index][0] = val;
        index+=1;
    }

    for i in 1..30{
        for j in 1..=n_q[0]{
            binary_lifting[j][i] = binary_lifting[binary_lifting[j][i-1]][i-1];
        }
    }

    for _ in 0..n_q[1]{
        let x_k : Vec<u32> = read_vector();
        let mut x : usize = x_k[0] as usize;
        let k : u32 = x_k[1];
        for i in 0..30{
            if k&(1<<i) > 0  {
                x = binary_lifting[x][i];
            }
        }
        println!("{x}");
    }
}