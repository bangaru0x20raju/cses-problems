/*
 * Author: Cheerapa Bangaru Raju
 */

#![allow(unused)]

use std::{collections::{BinaryHeap, VecDeque}, fmt::Debug, io, str::FromStr};

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
struct Path {
    dest: usize, 
    distance : u64
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance)
    }
}

fn main() {
    let n_m_k : Vec<usize> = read_vector();
    let mut graph : Vec<Vec<Path>> = vec![vec![]; n_m_k[0]+1];
    for i in 0..n_m_k[1]{
        let edge : Vec<u64> = read_vector();
        graph[edge[0] as usize].push(Path { dest: edge[1] as usize, distance: edge[2]});
    }
    let mut queue : BinaryHeap<Path> = BinaryHeap::new();
    let mut node_queue : Vec<BinaryHeap<u64>> = vec![BinaryHeap::new(); n_m_k[0]+1];
    queue.push(Path { dest: 1, distance: 0 });
    node_queue[1].push(0);

    while let Some(node) = queue.pop(){
        if node.distance > *node_queue[node.dest].peek().unwrap() {
            continue;
        }

        for adj_node in &graph[node.dest]{
            let next_node_distance = adj_node.distance + node.distance;
            if node_queue[adj_node.dest].len() < n_m_k[2]{
                node_queue[adj_node.dest].push(next_node_distance);
                queue.push(Path { dest: adj_node.dest, distance: next_node_distance });
            }else if *node_queue[adj_node.dest].peek().unwrap() > next_node_distance {
                node_queue[adj_node.dest].pop();
                node_queue[adj_node.dest].push(next_node_distance);
                queue.push(Path { dest: adj_node.dest, distance: next_node_distance });
            }
        }
    }

    let mut ans : VecDeque<u64> = VecDeque::new();
    while let Some(distance) = node_queue[n_m_k[0]].pop() { 
        ans.push_front(distance);
    }
    for distance in ans {
        print!("{distance} ");
    }
    println!();
}