/*
 * Author: Cheerapa Bangaru Raju
 */

#![allow(unused)]

use std::{fmt::Debug, io, str::FromStr, vec};

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

fn traverse(node : usize, graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>){
    visited[node] = true;
    for next_node in &graph[node]{
        if visited[*next_node] == false{
            traverse(*next_node, graph, visited);
        }
    }
}

fn main() {
    let n_and_m : Vec<usize> = read_vector();
    let mut graph : Vec<Vec<usize>> = vec![vec![];n_and_m[0]+1];
    let mut visited : Vec<bool> = vec![false;n_and_m[0]+1];
    for i in 0..n_and_m[1] { 
        let edge : Vec<usize> = read_vector();
        graph[edge[0]].push(edge[1]);
        graph[edge[1]].push(edge[0]);
    }
    let mut ans = -1;
    let mut new_roads : Vec<usize> = vec![];
    for node in 1..=n_and_m[0]{
        if visited[node] == false{
            traverse(node, &graph, &mut visited);
            ans+=1;
            new_roads.push(node);
        }
    }
    println!("{ans}");
    for i in 1..new_roads.len(){
        println!("{} {}", new_roads[i-1], new_roads[i]);
    }
}