/*
 * Author: Cheerapa Bangaru Raju
 */

#![allow(unused)]

use std::{cmp::{max, min}, collections::{BinaryHeap, VecDeque}, fmt::Debug, io, str::FromStr};

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
    let n_m : Vec<usize> = read_vector();
    let n: usize = n_m[0];
    let m: usize = n_m[1];
    let mut graph: Vec<Vec<Path>> = vec![vec![]; n+1];
    let mut price : Vec<u64> = vec![u64::MAX; n+1];
    let mut paths : Vec<u32> = vec![0; n+1];
    let mut minimum_flights : Vec<u32> = vec![n as u32; n+1];
    let mut maximum_flights : Vec<u32> = vec![n as u32; n+1];
    for _ in 0..m{
        let edge : Vec<u64> = read_vector();
        graph[edge[0] as usize].push(Path { dest: edge[1] as usize, distance: edge[2] });
    }
    let mut queue : BinaryHeap<Path> = BinaryHeap::new();
    queue.push(Path { dest: 1, distance: 0 });
    price[1] = 0;
    paths[1] = 1;
    minimum_flights[1] = 0; 
    maximum_flights[1] = 0;
    while let Some(node) = queue.pop(){
        if node.distance > price[node.dest]{
            continue;
        }
        for adj_node in &graph[node.dest]{
            if price[adj_node.dest] > adj_node.distance + price[node.dest]{
                price[adj_node.dest] = adj_node.distance + price[node.dest];
                paths[adj_node.dest] = paths[node.dest];
                minimum_flights[adj_node.dest] = minimum_flights[node.dest] + 1;
                maximum_flights[adj_node.dest] = maximum_flights[node.dest] + 1;
                queue.push(Path { dest: adj_node.dest, distance: price[adj_node.dest] });
            } else if price[adj_node.dest] == adj_node.distance + price[node.dest]{
                paths[adj_node.dest] += paths[node.dest];
                paths[adj_node.dest] %= MOD;
                minimum_flights[adj_node.dest] = min(minimum_flights[adj_node.dest], minimum_flights[node.dest]+1);
                maximum_flights[adj_node.dest] = max(maximum_flights[adj_node.dest], maximum_flights[node.dest]+1);
            }
        }
    }

    println!("{} {} {} {}", price[n], paths[n], minimum_flights[n], maximum_flights[n]);
}