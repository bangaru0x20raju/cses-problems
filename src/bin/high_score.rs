/*
 * Author: Cheerapa Bangaru Raju
 */

#![allow(unused)]

use std::{cmp::max, collections::HashSet, fmt::Debug, i64, io, str::FromStr};

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

fn traverse(node : usize, n : usize, graph: &Vec<Vec<(usize)>>, visited: &mut Vec<bool>) -> bool{
    if node == n{
        return true;
    }
    visited[node] = true;
    let mut score : i64 = i64::MIN;
    let mut can_reach = false;
    for next_node in &graph[node]{
        if visited[*next_node] == false {
            if traverse(*next_node, n, graph, visited){
                return true;
            }
        }
    }
    return false;
}


struct Edge{
    source : usize, 
    dest : usize, 
    weight : i64
}

fn main() {
    let n_m : Vec<usize> = read_vector();
    let mut edges : Vec<Edge> = vec![];
    let mut graph : Vec<Vec<usize>> = vec![vec![]; n_m[0]+1];
    for i in 0..n_m[1]{
        let edge : Vec<i64> = read_vector();
        edges.push(Edge{
            source : edge[0] as usize, 
            dest : edge[1] as usize, 
            weight : edge[2]
        });
        graph[edge[0] as usize].push(edge[1] as usize);
    }

    let mut distance : Vec<i64> = vec![i64::MIN; n_m[0]+1];
    for edge in &edges{
        if edge.source == 1 {
            distance[edge.dest] = edge.weight;
        }
    }
    distance[1] = 0;
    for i in 1..n_m[1]{
        for edge in &edges{
            if distance[edge.source]!= i64::MIN && distance[edge.dest] < distance[edge.source] + edge.weight{
                distance[edge.dest] = distance[edge.source] + edge.weight;
            }
        }
    }
    let mut has_cycle = false;
    let mut visited : Vec<bool> = vec![false;n_m[0]+1];
    for i in 0..n_m[1]{
        for edge in &edges{
            if distance[edge.source]!= i64::MIN && distance[edge.dest] < distance[edge.source] + edge.weight{
                has_cycle = traverse(edge.dest, n_m[0], &graph, &mut visited);
            }
            if has_cycle {
                break;
            }
        }
        if has_cycle{
            break;
        }
    }
    if has_cycle{
        println!("-1");
    }else{
        println!("{}",distance[n_m[0]]);
    }
}