/*
 * Author: Cheerapa Bangaru Raju
 */

#![allow(unused)]

use std::{
    collections::{HashSet, VecDeque},
    fmt::Debug,
    io,
    str::FromStr,
};

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
    let n_m: Vec<usize> = read_vector();
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n_m[0] + 1];
    let mut reverse_graph: Vec<Vec<usize>> = vec![vec![]; n_m[0] + 1];
    let mut visited: Vec<bool> = vec![false; n_m[0] + 1];
    let mut reverse_visited: Vec<bool> = vec![false; n_m[0] + 1];
    let mut queue: VecDeque<usize> = VecDeque::new();
    for _ in 0..n_m[1] {
        let edge: Vec<usize> = read_vector();
        graph[edge[0]].push(edge[1]);
        reverse_graph[edge[1]].push(edge[0]);
    }
    queue.push_back(1);
    while let Some(node) = queue.pop_front() {
        if visited[node] == true {
            continue;
        }
        visited[node] = true;
        for &adj_node in &graph[node] {
            if visited[adj_node] == false {
                queue.push_back(adj_node);
            }
        }
    }
    for node in 1..=n_m[0] {
        if visited[node] == false {
            println!("NO");
            println!("1 {node}");
            return;
        }
    }

    queue.push_back(1);
    while let Some(node) = queue.pop_front() {
        if reverse_visited[node] == true {
            continue;
        }
        reverse_visited[node] = true;
        for &adj_node in &reverse_graph[node] {
            if reverse_visited[adj_node] == false {
                queue.push_back(adj_node);
            }
        }
    }
    for node in 1..=n_m[0] {
        if reverse_visited[node] == false {
            println!("NO");
            println!("{node} 1");
            return;
        }
    }
    println!("YES");
}
