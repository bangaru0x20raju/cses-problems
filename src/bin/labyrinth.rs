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

const ROW_MOVES : [i16;4] = [-1, 1, 0, 0];
const COL_MOVES : [i16;4] = [0, 0, 1, -1];
const DIRECTIONS : [char;4] = ['U','D','R', 'L'];

struct Step{
    row : usize,
    col : usize,
    steps : usize
}

fn main() {
    let n_and_m : Vec<usize> = read_vector();
    let mut grid : Vec<Vec<char>> = Vec::new();
    for i in 0..n_and_m[0] { 
        let line = read_string();
        grid.push(line.chars().collect());
    }
    let mut visited : Vec<Vec<bool>> = vec![vec![false;n_and_m[1]]; n_and_m[0]];
    let mut parent_dir: Vec<Vec<char>> = vec![vec![' '; n_and_m[1]]; n_and_m[0]];
    let mut queue : VecDeque<Step> = VecDeque::new();
    let mut start_row : usize = 0;
    let mut start_col : usize = 0;
    for i in 0..n_and_m[0]{
        for j in 0..n_and_m[1]{
            if grid[i][j] == 'A'{
                queue.push_back(Step{
                    row : i, 
                    col : j, 
                    steps : 0
                });
                visited[i][j] = true;
                start_row = i;
                start_col = j;
            }
        }
        if queue.len() > 0 {
            break;
        }
    }
    let mut ans: Option<Step> = None;
    
    while queue.is_empty() == false{
        let top = queue.pop_front().unwrap();
        if grid[top.row][top.col] == 'B'{
            ans = Some(top);
            break;
        }
        for k in 0..4 { 
            if (top.row == 0 && ROW_MOVES[k] == -1) || (top.row == n_and_m[0] - 1 && ROW_MOVES[k] == 1){
                continue;
            }
            if (top.col == 0 && COL_MOVES[k] == -1) || (top.col == n_and_m[1] - 1 && COL_MOVES[k] == 1){
                continue;
            }
            let new_row = (top.row as i16 + ROW_MOVES[k]) as usize;
            let new_col = (top.col as i16 + COL_MOVES[k]) as usize;
            if visited[new_row][new_col] == false && (grid[new_row][new_col] == '.' || grid[new_row][new_col] == 'B'){
                visited[new_row][new_col] = true;
                parent_dir[new_row][new_col] = DIRECTIONS[k];
                queue.push_back(Step{
                    row : new_row,
                    col : new_col,
                    steps : top.steps + 1
                });
            }
        }
    }
    if let Some(dest) = ans {
        let mut path: Vec<char> = Vec::new();
        let mut curr_row = dest.row;
        let mut curr_col = dest.col;
        while (curr_row, curr_col) != (start_row, start_col) {
            let dir = parent_dir[curr_row][curr_col];
            path.push(dir);
            match dir {
                'U' => curr_row += 1, // opposite of Up is Down
                'D' => curr_row -= 1, // opposite of Down is Up
                'R' => curr_col -= 1, // opposite of Right is Left
                'L' => curr_col += 1, // opposite of Left is Right
                _ => {}
            }
        }
        path.reverse();
        println!("YES");
        println!("{}", dest.steps);
        println!("{}", path.iter().collect::<String>());
    }else{
        println!("NO");
    }
}