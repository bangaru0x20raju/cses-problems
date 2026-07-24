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

const ROW_MOVES : [i16;4] = [-1, 1, 0, 0];
const COL_MOVES : [i16;4] = [0, 0, 1, -1];


fn traverse(i : usize, j: usize, n : usize, m: usize, grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>){
    visited[i][j] = true;
    for k in 0..4{
        if (i == 0 && ROW_MOVES[k] == -1) || (i == n-1 && ROW_MOVES[k] == 1) {
            continue;
        }
        if (j == 0 && COL_MOVES[k] == -1) || (j == m-1 && COL_MOVES[k] == 1) {
            continue;
        }
        let new_row = (i as i16 + ROW_MOVES[k]) as usize;
        let new_col = (j as i16 + COL_MOVES[k]) as usize;
        if visited[new_row][new_col] == false && grid[new_row][new_col] == '.' {
            traverse(new_row, new_col, n, m, grid, visited);
        }
    }
}

fn main() {
    let n_and_m : Vec<usize> = read_vector();
    let mut grid : Vec<Vec<char>> = Vec::new();
    for i in 0..n_and_m[0] { 
        let line = read_string();
        grid.push(line.chars().collect());
    }
    let mut visited : Vec<Vec<bool>> = vec![vec![false;n_and_m[1]]; n_and_m[0]];
    let mut ans: u32 = 0;
    for i in 0..n_and_m[0]{
        for j in 0..n_and_m[1]{
            if visited[i][j] == false && grid[i][j] == '.'{
                traverse(i, j, n_and_m[0], n_and_m[1], &grid, &mut visited);
                ans+=1;
            }
        }
    }
    println!("{ans}");
}