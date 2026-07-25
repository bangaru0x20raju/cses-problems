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

fn traverse(node : usize, team: i8, graph: &Vec<Vec<usize>>, teams : &mut Vec<i8>) -> bool{
    for next_node in &graph[node]{
        if teams[*next_node] !=0 && teams[*next_node] == team {
            return false;
        }
    }
    teams[node] = team;
    let mut can_we_assign = true;
    for next_node in &graph[node]{
        if teams[*next_node] == 0 {
            can_we_assign &= traverse(*next_node, team^3, graph, teams);
            if can_we_assign == false{
                return false;
            }
        }
    }
    return true;
}


fn main() {
    let n_and_m : Vec<usize> = read_vector();
    let mut graph: Vec<Vec<usize>> = vec![vec![];n_and_m[0]+1];
    for i in 0..n_and_m[1]{
        let edge : Vec<usize> = read_vector();
        graph[edge[0]].push(edge[1]);
        graph[edge[1]].push(edge[0]);
    }
    let mut teams : Vec<i8> = vec![0; n_and_m[0]+1];
    let mut can_we_assign = true;
    for node in 1..=n_and_m[0]{
        if teams[node] == 0{
            can_we_assign &= traverse(node, 1, &graph, &mut teams);            
        }
    }
    if can_we_assign{
        for node in 1..=n_and_m[0]{
            print!("{} ", teams[node]);
        }
        println!();
    }else{
        println!("IMPOSSIBLE");
    }
}