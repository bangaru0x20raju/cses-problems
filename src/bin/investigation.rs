/*
 * Author: Cheerapa Bangaru Raju
 */

#![allow(unused)]

use std::{cmp::{max, min}, collections::VecDeque, fmt::Debug, io, str::FromStr};

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
    let n_m : Vec<usize> = read_vector();
    let n: usize = n_m[0];
    let m: usize = n_m[1];
    let mut graph: Vec<Vec<(usize, u64)>> = vec![vec![]; n+1];
    let mut in_degree : Vec<u32> = vec![0; n+1];
    let mut price : Vec<u64> = vec![u64::MAX; n+1];
    let mut paths : Vec<u32> = vec![0; n+1];
    let mut minimum_flights : Vec<u32> = vec![n as u32; n+1];
    let mut maximum_flights : Vec<u32> = vec![n as u32; n+1];
    let mut queue : VecDeque<usize> = VecDeque::new();
    let mut topo : Vec<usize> = vec![];
    for _ in 0..m{
        let edge : Vec<u64> = read_vector();
        //println!("{:?}", edge);
        graph[edge[0] as usize].push((edge[1] as usize, edge[2]));
        in_degree[edge[1] as usize] += 1; 
    }

    println!("{:?}", in_degree);
    for i in 1..=n{
        if in_degree[i] == 0{
            queue.push_back(i);
        }
    }
    println!("{:?}", queue);
    while let Some(node) = queue.pop_front(){
        topo.push(node);
        for &adj_node in &graph[node]{
            in_degree[adj_node.0] -= 1;
            if in_degree[adj_node.0] == 0 {
                println!("Becomes 0: {}", adj_node.0);
                queue.push_back(adj_node.0);
            }
        }
    }

    println!("{:?}", topo);
    price[1] = 0;
    paths[1] = 1;
    minimum_flights[1] = 0; 
    maximum_flights[1] = 0;
    for node in topo{
        println!("{node}");
        if price[node] == u64::MAX{
            continue;
        }
        for &adj_node in &graph[node]{
            if price[adj_node.0] > adj_node.1 + price[node]{
                price[adj_node.0] = adj_node.1 + price[node];
                paths[adj_node.0] = paths[node];
                minimum_flights[adj_node.0] = minimum_flights[node] + 1;
                maximum_flights[adj_node.0] = maximum_flights[node] + 1;
            } else if price[adj_node.0] == adj_node.1 + price[node]{
                paths[adj_node.0] += paths[node];
                paths[adj_node.0] %= MOD;
                minimum_flights[adj_node.0] = min(minimum_flights[adj_node.0], minimum_flights[node]+1);
                maximum_flights[adj_node.0] = max(maximum_flights[adj_node.0], maximum_flights[node]+1);
            }
        }
    }

    println!("{} {} {} {}", price[n], paths[n], minimum_flights[n], maximum_flights[n]);
}