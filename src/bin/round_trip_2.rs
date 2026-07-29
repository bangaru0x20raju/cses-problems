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

fn traverse(node : usize, graph: &Vec<Vec<usize>>, visited : &mut Vec<u8>, parent_node : &mut Vec<usize>) -> Option<(usize, usize)>{
    visited[node] = 1;
    for adj_node in &graph[node]{
        if visited[*adj_node] == 0{
            parent_node[*adj_node] = node;
            if let Some(cycle) = traverse(*adj_node, graph, visited, parent_node){
                return Some(cycle);
            }
        }else if visited[*adj_node] == 1{
            return Some((node, *adj_node));
        }
    }
    visited[node] = 2;
    None
}

fn main() {
    let n_m : Vec<usize> = read_vector();
    let mut graph : Vec<Vec<usize>> = vec![vec![]; n_m[0]+1];
    for i in 0..n_m[1]{
        let edge : Vec<usize> = read_vector();
        graph[edge[0]].push(edge[1]);
    }

    let mut parent_node: Vec<usize> = vec![0; n_m[0]+1];
    let mut visited : Vec<u8> = vec![0; n_m[0]+1];
    let mut has_cycle = false;
    for i in 1..=n_m[0]{
        if visited[i] == 0{
            if let Some(cycle) = traverse(i, &graph, &mut visited, &mut parent_node){
                let mut ans : VecDeque<usize> = VecDeque::new();
                ans.push_back(cycle.1);
                let mut curr_node = cycle.0;
                while curr_node!=cycle.1{
                    ans.push_front(curr_node);
                    curr_node = parent_node[curr_node];
                }
                ans.push_front(curr_node);
                println!("{}", ans.len());
                for ans_node in ans{
                    print!("{ans_node} ");
                }
                println!();
                has_cycle = true;
            }
        }
        if has_cycle {
            break;
        }
    }
    if !has_cycle{
        println!("IMPOSSIBLE");
    }
    
}