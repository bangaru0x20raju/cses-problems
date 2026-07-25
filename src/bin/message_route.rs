/*
 * Author: Cheerapa Bangaru Raju
 */

#![allow(unused)]

use std::{collections::{VecDeque, vec_deque}, fmt::Debug, io, str::FromStr, vec};

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

struct Step{
    node : usize,
}

fn main() {
    let n_and_m : Vec<usize> = read_vector();
    let mut graph : Vec<Vec<usize>> = vec![vec![]; n_and_m[0]+1];
    let mut visited : Vec<bool> = vec![false;n_and_m[0]+1];
    let mut queue : VecDeque<Step> = VecDeque::new();
    for i in 0..n_and_m[1]{
        let edge : Vec<usize> = read_vector();
        graph[edge[0]].push(edge[1]);
        graph[edge[1]].push(edge[0]);
    }
    visited[1] = true;
    queue.push_back(Step{
        node : 1,
    });
    let mut dest_node: Option<Step> = None;
    let mut parent_node : Vec<usize> = vec![0;n_and_m[0]+1];
    while let Some(top) = queue.pop_front() { 
        if top.node == n_and_m[0]{
            dest_node = Some(top);
            break;
        }
        for next_node in &graph[top.node]{
            if visited[*next_node] == false{
                visited[*next_node] = true;
                queue.push_back(Step { node: *next_node});
                parent_node[*next_node] = top.node;
            }
        }
    }
    if let Some(node) = dest_node{
        let mut path : Vec<usize> = vec![];
        let mut curr_node = node.node;
        while curr_node!= 0{
            path.push(curr_node);
            curr_node = parent_node[curr_node];
        }
        println!("{}", path.len());
        path.reverse();
        for path_node in path{
            print!("{path_node} ");
        }
        println!();
    }else{
        println!("IMPOSSIBLE");
    }
}