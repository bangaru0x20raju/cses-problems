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
    let n = n_m[0];
    let m = n_m[1];
    let mut graph : Vec<Vec<usize>> = vec![vec![]; n+1];
    let mut in_degree : Vec<usize> = vec![0; n+1];

    for _ in 0..m {
        let edge : Vec<usize> = read_vector();
        graph[edge[0]].push(edge[1]);
        in_degree[edge[1]] += 1;
    }

    let mut queue: VecDeque<usize> = VecDeque::new();
    for i in 1..=n {
        if in_degree[i] == 0 {
            queue.push_back(i);
        }
    }

    let mut topo_order: Vec<usize> = Vec::new();
    while let Some(node) = queue.pop_front() {
        topo_order.push(node);
        for &adj_node in &graph[node] {
            in_degree[adj_node] -= 1;
            if in_degree[adj_node] == 0 {
                queue.push_back(adj_node);
            }
        }
    }

    let mut dist: Vec<i64> = vec![0; n+1];
    let mut parent_node: Vec<usize> = vec![0; n+1];
    dist[1] = 1;

    for &node in &topo_order {
        if dist[node] == 0 {
            continue;
        }
        for &adj_node in &graph[node] {
            if dist[adj_node] < dist[node] + 1 {
                dist[adj_node] = dist[node] + 1;
                parent_node[adj_node] = node;
            }
        }
    }
    
    if dist[n] == 0 {
        println!("IMPOSSIBLE");
    } else {
        let mut path: VecDeque<usize> = VecDeque::new();
        let mut curr_node = n;
        while curr_node != 0 {
            path.push_front(curr_node);
            curr_node = parent_node[curr_node];
        }
        println!("{}", path.len());
        let result: Vec<String> = path.iter().map(|x| x.to_string()).collect();
        println!("{}", result.join(" "));
    }
}