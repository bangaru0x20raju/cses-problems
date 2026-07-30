/*
 * Author: Cheerapa Bangaru Raju
 */

#![allow(unused)]

use std::{collections::VecDeque, fmt::Debug, io, str::FromStr};

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
    let n_m : Vec<usize> = read_vector();
    let mut graph : Vec<Vec<usize>> = vec![vec![]; n_m[0]+1];
    let mut in_degree : Vec<u32> = vec![0; n_m[0]+1];
    for i in 0..n_m[1]{
        let edge : Vec<usize> = read_vector();
        graph[edge[0]].push(edge[1]);
        in_degree[edge[1]] += 1;
    }

    let mut queue : VecDeque<usize> = VecDeque::new();
    let mut ans : Vec<usize> = vec![];

    for i in 1..=n_m[0]{
        if in_degree[i] == 0{
            queue.push_back(i);
        }
    }

    while let Some(node) = queue.pop_front() {
        ans.push(node);
        for adj_node in &graph[node]{
            in_degree[*adj_node]-=1;
            if in_degree[*adj_node] == 0{
                queue.push_back(*adj_node);
            }
        }
    }
    if ans.len() < n_m[0]{
        println!("IMPOSSIBLE");
    }else{
        for node in ans{
            print!("{node} ");
        }
        println!();
    }
    
}