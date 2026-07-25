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


fn traverse(node : usize, prev_node : usize, graph: &Vec<Vec<usize>>, parent_node: &mut Vec<usize>) -> Option<(usize,usize)>{
    for next_node in &graph[node]{
        if *next_node != prev_node {
            if parent_node[*next_node]!=0{
                return Some((node, *next_node));
            }
            else{
                parent_node[*next_node] = node;
                if let Some(cycle) = traverse(*next_node, node, graph, parent_node){
                    return Some(cycle);
                }
            }
        }
    }
    return None;
}

fn main() {
    let n_and_m : Vec<usize> = read_vector();
    let mut graph : Vec<Vec<usize>> = vec![vec![];n_and_m[0]+1];
    let mut parent_node : Vec<usize> = vec![0; n_and_m[0]+1];

    for i in 0..n_and_m[1]{
        let edge : Vec<usize> = read_vector();
        graph[edge[0]].push(edge[1]);
        graph[edge[1]].push(edge[0]);
    }
    
    for i in 1..=n_and_m[0]{
        if parent_node[i] == 0{
            parent_node[i] = i;
            if let Some(cycle) = traverse(i, i, &graph, &mut parent_node){
                let mut path : Vec<usize> = vec![];
                let mut curr_node = cycle.0;
                while(curr_node != cycle.1){
                    path.push(curr_node);
                    curr_node = parent_node[curr_node];
                }
                path.push(cycle.1);
                path.push(cycle.0);
                println!("{}", path.len());
                for node in path{
                    print!("{node} ");
                }
                println!();
                return;
            }
        }
    }
    println!("IMPOSSIBLE");
}