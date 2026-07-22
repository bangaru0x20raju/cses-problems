/*
 * Author: Cheerapa Bangaru Raju
 */

#![allow(unused)]

use std::{cmp::max, fmt::Debug, io, process::id, str::FromStr};

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
#[derive(Debug)]
struct Project {
    start_day : u64,
    end_day : u64, 
    reward : u64
}

fn lower_bound(vec: &Vec<u64>, target : &u64) -> Option<usize>{
    let idx = vec.partition_point(|x| x<target);
    if idx > 0 {
        Some(idx-1)
    }else{
        None
    }
}

fn main() {
    let n : usize = read_number();
    let mut projects : Vec<Project> = Vec::new();
    for i in 0..n { 
        let temp: Vec<u64> = read_vector();
        projects.push(Project { start_day: temp[0], end_day: temp[1], reward: temp[2] });
    }
    
    projects.sort_by_key(|p| p.end_day);
    let mut end_days: Vec<u64> = Vec::new();
    for i in 0..n{
        end_days.push(projects[i].end_day);
    }
    let mut reward_till : Vec<u64> = Vec::new();
    reward_till.resize(n, 0);
    reward_till[0] = projects[0].reward;
    for i in 1..n {
        reward_till[i] = projects[i].reward;
        if let Some(idx) = lower_bound(&end_days, &projects[i].start_day){
            reward_till[i]+=reward_till[idx];
        }
        reward_till[i] = max(reward_till[i], reward_till[i-1]);
    }
    println!("{}", reward_till[n-1]);
}