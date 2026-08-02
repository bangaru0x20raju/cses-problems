/*
 * Author: Cheerapa Bangaru Raju
 */

#![allow(unused)]

use std::{cmp::max, fmt::Debug, io, str::FromStr};

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

struct DisjointSetUnion{
    parent : Vec<usize>,
    size : Vec<usize>,
    no_of_sets : usize,
    max_size : usize
}

impl DisjointSetUnion {
    pub fn new(n : usize) -> Self{
        let mut parent : Vec<usize> = vec![0; n+1];
        for i in 1..=n {
            parent[i] = i;
        }
        Self { parent: parent, size: vec![1; n+1], no_of_sets : n, max_size : 1}
    }

    pub fn find_set(&mut self, node : usize) -> usize{
        if self.parent[node] == node {
            node
        }else{
            self.parent[node] = self.find_set(self.parent[node]);
            self.parent[node]
        }
    }

    pub fn make_set(&mut self, node_1 : usize, node_2 : usize) -> bool{
        let parent_1 : usize = self.find_set(node_1);
        let parent_2 : usize = self.find_set(node_2);
        if parent_1 == parent_2{
            return false;
        }
        if self.size[parent_1] >= self.size[parent_2]{
            self.parent[parent_2] = parent_1;
            self.size[parent_1] += self.size[parent_2];
            self.max_size = max(self.max_size, self.size[parent_1]);
        }else{
            self.parent[parent_1] = parent_2;
            self.size[parent_2] += self.size[parent_1];
            self.max_size = max(self.max_size, self.size[parent_2]);
        }
        self.no_of_sets -= 1;
        true
    }
}


fn main() {

    let n_m : Vec<usize> = read_vector();
    let mut dsu_obj : DisjointSetUnion = DisjointSetUnion::new(n_m[0]);

    for _ in 0..n_m[1] {
        let edge : Vec<usize> = read_vector();
        dsu_obj.make_set(edge[0], edge[1]);
        println!("{} {}", dsu_obj.no_of_sets, dsu_obj.max_size);
    }
    
}