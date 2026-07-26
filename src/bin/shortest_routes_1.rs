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
#[derive(Clone, PartialEq, Eq)]
struct Connection{
    dest : usize, 
    cost : u64
}
impl PartialOrd for Connection {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}

impl Ord for Connection {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return other.cost.cmp(&self.cost);
    }
}

fn main() {
    let n_and_m : Vec<usize> = read_vector();
    let mut graph : Vec<Vec<Connection>> = vec![vec![]; n_and_m[0]+1];
    for i in 0..n_and_m[1]{
        let edge : Vec<u64> = read_vector();
        graph[edge[0] as usize].push(Connection{
            dest: edge[1] as usize,
            cost : edge[2]
        });
        // graph[edge[1] as usize].push(Connection { dest: edge[0] as usize, cost: edge[2] });
    }
    let mut distances : Vec<u64> = vec![u64::MAX; n_and_m[0]+1];
    distances[1] = 0;
    let mut queue : BinaryHeap<Connection> = BinaryHeap::new();
    queue.push(Connection { dest: 1, cost: 0 });
    while let Some(node) = queue.pop() {
        if node.cost > distances[node.dest]{
            continue;
        }
        for adj_node in &graph[node.dest]{
            if distances[adj_node.dest] > node.cost + adj_node.cost{
                distances[adj_node.dest] = node.cost + adj_node.cost;
                queue.push(Connection { dest: adj_node.dest, cost: distances[adj_node.dest] });
            }
        }
    }
    for i in 1..=n_and_m[0]{
        print!("{} ", distances[i]);
    }
    println!();
}