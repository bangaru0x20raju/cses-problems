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

const MOD : u32 = 1_000_000_007;

fn main() {
    let n_m: Vec<usize> = read_vector();
    let n : usize = n_m[0];
    let m : usize = n_m[1];
    let mut graph: Vec<Vec<usize>> = vec![vec![];n+1];
    let mut ways : Vec<u32> = vec![0;n+1]; 
    let mut in_degree : Vec<u32> = vec![0;n+1];
    for _ in 0..m{
        let edge : Vec<usize> = read_vector();
        graph[edge[0]].push(edge[1]);
        in_degree[edge[1]] += 1; 
    }
    let mut topo_order : Vec<usize> = vec![];
    let mut queue : VecDeque<usize> = VecDeque::new();
    for node in 1..=n{
        if in_degree[node] == 0{
            queue.push_back(node);
        }
    }
    while let Some(node) = queue.pop_front(){
        topo_order.push(node);
        for adj_node in &graph[node]{
            in_degree[*adj_node] -= 1; 
            if in_degree[*adj_node] == 0{
                queue.push_back(*adj_node);
            }
        }
    }
    ways[1] = 1;
    for node in topo_order{
        if ways[node] == 0{
            continue;
        }
        for adj_node in &graph[node]{
            ways[*adj_node] += ways[node];
            ways[*adj_node] %= MOD;
        }
    }
    println!("{}", ways[n]);

}
