/*
 * Author: Cheerapa Bangaru Raju
 */

#![allow(unused)]

use std::{collections::BinaryHeap, fmt::Debug, io, str::FromStr};

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

#[derive(PartialEq, Eq, Clone)]
struct Edge{
    dest : usize, 
    cost : u64,
}


impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Edge{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}


fn main() {
    let n_m : Vec<usize> = read_vector();
    let mut graph : Vec<Vec<Edge>> = vec![vec![]; n_m[0]+1];
    let mut visited : Vec<bool> = vec![false; n_m[0]+1];
    let mut queue : BinaryHeap<Edge> = BinaryHeap::new();
    let mut total_cost : u64 = 0;
    let mut visited_count: usize = 0;
    for _ in 0..n_m[1]{
        let edge : Vec<usize> = read_vector();
        graph[edge[0]].push(Edge { dest: edge[1], cost: edge[2] as u64 });
        graph[edge[1]].push(Edge { dest: edge[0], cost: edge[2] as u64 });
    }

    queue.push(Edge { dest: 1, cost: 0 });
    while let Some(node) = queue.pop(){
        if visited[node.dest] == true{
            continue;
        }
        visited[node.dest] = true;
        total_cost += node.cost;
        visited_count += 1;
        for edge in &graph[node.dest]{
            if visited[edge.dest] == false{
                queue.push(Edge { dest: edge.dest, cost: edge.cost});
            }
        }
    }
    if visited_count != n_m[0]{
        println!("IMPOSSIBLE");
    }else{
        println!("{total_cost}");
    }
    
}