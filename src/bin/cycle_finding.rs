/*
 * Author: Cheerapa Bangaru Raju
 */

#![allow(unused)]

use std::{fmt::Debug, io, str::FromStr};

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

struct Edge { 
    source : usize, 
    dest : usize, 
    weight : i64
}

fn main() {
    let n_m : Vec<usize> = read_vector();
    let mut edges : Vec<Edge> = vec![];
    for i in 0..n_m[1]{
        let edge : Vec<i64> = read_vector();
        edges.push(Edge { source: edge[0] as usize, dest: edge[1] as usize, weight: edge[2] });
    }
    let mut distances : Vec<i64> = vec![0; n_m[0]+1];
    distances[1] = 0;
    let mut parent : Vec<usize> = vec![0; n_m[0]+1];
    for i in 1..=n_m[0]{
        parent[i] = i;
    }
    for i in 1..n_m[0]{
        for edge in &edges{
            if distances[edge.dest] > distances[edge.source]+ edge.weight{
                distances[edge.dest] = distances[edge.source]+ edge.weight;
                parent[edge.dest] = edge.source;
            }
        }
    }
    for edge in &edges{
        if distances[edge.dest] > distances[edge.source]+ edge.weight{
            println!("YES");
            let mut node = edge.dest;
            for _ in 0..n_m[0]{
                node = parent[node];
            }
            let cycle_node = node;
            let mut path : Vec<usize> = vec![cycle_node];
            let mut curr = parent[cycle_node];
            while curr != cycle_node{
                path.push(curr);
                curr = parent[curr];
            }
            path.push(cycle_node);
            path.reverse();
            let result : Vec<String> = path.iter().map(|x| x.to_string()).collect();
            println!("{}", result.join(" "));
            return;
        }
    }
    println!("NO");
}