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

fn dfs1(node : usize, graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>, stack : &mut Vec<usize>){
    visited[node] = true;
    for &adj_node in &graph[node]{
        if visited[adj_node] == false{
            dfs1(adj_node, graph, visited, stack);
        }
    }
    stack.push(node);
}

fn dfs2(node : usize, graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>, ssc_count : usize, kingdom : &mut Vec<usize>){
    kingdom[node] = ssc_count;
    visited[node] = true;
    for &adj_node in &graph[node]{
        if visited[adj_node] == false{
            dfs2(adj_node, graph, visited, ssc_count, kingdom);
        }
    }
}

fn main() {
    let n_m : Vec<usize> = read_vector();
    let n : usize = n_m[0];
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n + 1];
    let mut reverse_graph: Vec<Vec<usize>> = vec![vec![]; n + 1];
    let mut visited: Vec<bool> = vec![false; n + 1];
    let mut reverse_visited: Vec<bool> = vec![false; n + 1];
    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut stack : Vec<usize> = vec![];
    let mut ssc_count : usize = 0;
    let mut kingdom : Vec<usize> = vec![0; n+1];
    for _ in 0..n_m[1]{
        let edge : Vec<usize> = read_vector();
        graph[edge[0]].push(edge[1]);
        reverse_graph[edge[1]].push(edge[0]);
    }
    
    for node in 1..=n { 
        if visited[node] == false{
            dfs1(node, &graph, &mut visited, &mut stack);
        }
    }
    while let Some(node) = stack.pop(){
        if reverse_visited[node] == true { 
            continue;
        }
        ssc_count += 1;
        dfs2(node, &reverse_graph, &mut reverse_visited, ssc_count, &mut kingdom);
    }

    println!("{ssc_count}");
    for k in 1..=n{
        print!("{} ", kingdom[k]);
    }
    println!();
}