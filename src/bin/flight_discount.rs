/*
 * Author: Cheerapa Bangaru Raju
 */

#![allow(unused)]

use std::{cmp::min, collections::BinaryHeap, fmt::Debug, io, str::FromStr};

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
struct Route{
    dest : usize, 
    cost : u64,
    used : usize
}


impl PartialOrd for Route {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Route{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

fn main() {
    let n_m : Vec<usize> = read_vector();
    let mut graph : Vec<Vec<Route>> = vec![vec![]; n_m[0]+1];
    for i in 0..n_m[1]{
        let edge : Vec<u64> = read_vector();
        graph[edge[0] as usize].push(Route{
            dest: edge[1] as usize,
            cost : edge[2], 
            used : 0
        });
    }
    let mut distance : Vec<Vec<u64>> = vec![vec![u64::MAX;2]; n_m[0]+1];
    let mut parent : Vec<usize> = vec![0; n_m[0]+1];
    let mut costs : Vec<u64> = vec![0; n_m[0]+1];
    let mut queue : BinaryHeap<Route> = BinaryHeap::new();
    distance[1][0] = 0;
    distance[1][1] = 0;
    queue.push(Route{
        dest: 1,
        cost : 0,
        used : 0
    });
    while let Some(node) = queue.pop(){
        if node.cost > distance[node.dest][node.used]{
            continue;
        }
        for adj_node in &graph[node.dest]{
            if node.used == 0{
                if distance[adj_node.dest][0] > distance[node.dest][0] + adj_node.cost{
                    distance[adj_node.dest][0] = distance[node.dest][0] + adj_node.cost;
                    queue.push(Route { dest: adj_node.dest, cost: distance[adj_node.dest][0], used: 0 });
                }
                if distance[adj_node.dest][1] > distance[node.dest][0] + adj_node.cost/2 {
                    distance[adj_node.dest][1] = distance[node.dest][0] + adj_node.cost/2;
                    queue.push(Route { dest: adj_node.dest, cost: distance[adj_node.dest][1], used: 1 });
                }
            }else if distance[adj_node.dest][1] > distance[node.dest][1] + adj_node.cost{
                distance[adj_node.dest][1] = distance[node.dest][1] + adj_node.cost;
                queue.push(Route { dest: adj_node.dest, cost: distance[adj_node.dest][1], used: 1 });
            }
        }
    }
    
    println!("{}", min(distance[n_m[0]][0],distance[n_m[0]][1]) );
}