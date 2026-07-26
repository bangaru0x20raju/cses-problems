/*
 * Author: Cheerapa Bangaru Raju
 */

#![allow(unused)]

use std::{cmp::min, fmt::Debug, io, ptr::read, str::FromStr};

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

fn main() {
    let n_m_q : Vec<usize> = read_vector();
    let mut graph : Vec<Vec<u64>> = vec![vec![u64::MAX; n_m_q[0]+1]; n_m_q[0]+1];
    for i in 0..n_m_q[1]{
        let edge : Vec<usize> = read_vector();
        graph[edge[0]][edge[1]] = min(graph[edge[0]][edge[1]], edge[2] as u64);
        graph[edge[1]][edge[0]] = min(graph[edge[1]][edge[0]], edge[2] as u64);
    }
    
    for i in 1..=n_m_q[0]{
        for j in 1..=n_m_q[0]{
            if i == j {
                graph[i][j] = 0;
            }
        }
    }
    
    for k in 1..=n_m_q[0]{
        for i in 1..=n_m_q[0]{
            for j in 1..=n_m_q[0]{
                if graph[i][k]!= u64::MAX && graph[k][j] !=u64::MAX{
                    graph[i][j] = min(graph[i][j], graph[i][k]+graph[k][j]);
                }
            }
        }
    }

    for q in 0..n_m_q[2]{
        let query : Vec<usize> = read_vector();
        let distance: i64 =  if graph[query[0]][query[1]] != u64::MAX { graph[query[0]][query[1]] as i64} else {-1};
        println!("{distance}");
    }
}