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

fn monster_spread(row : usize, col : usize, n : usize, m : usize, monster_loc : &mut Vec<Vec<bool>>, monster_last_locs : &mut Vec<(usize, usize)>, graph: &Vec<Vec<char>>){
    for k in 0..4{
        if (row == 0 && ROW_MOVES[k] == -1) || ( row == n - 1 && ROW_MOVES[k] == 1){
            continue;
        }
        if (col == 0 && COL_MOVES[k] == -1) || (col == m - 1 && COL_MOVES[k] == 1){
            continue;
        }
        let new_row = (row as i16 + ROW_MOVES[k]) as usize;
        let new_col = (col as i16 + COL_MOVES[k]) as usize;
        if graph[new_row][new_col] == '.' && monster_loc[new_row][new_col] == false {
            monster_loc[new_row][new_col] = true;
            monster_last_locs.push((new_row, new_col));
        }
    }
}

fn main() {
    let n_and_m : Vec<usize> = read_vector();
    let mut graph: Vec<Vec<char>> = vec![];
    for i in 0..n_and_m[0]{
        let line = read_string();
        graph.push(line.chars().collect());
    }
    let mut start_row : usize = 0;
    let mut start_col : usize = 0;
    let mut monster_locations : Vec<Vec<bool>> = vec![vec![false;n_and_m[1]];n_and_m[0]];
    let mut monster_last_locs : Vec<(usize, usize)> = vec![];
    for i in 0..n_and_m[0]{
        for j in 0..n_and_m[1]{
            if graph[i][j] == 'M'{
                monster_locations[i][j] = true;
                monster_spread(i, j, n_and_m[0], n_and_m[1], &mut monster_locations, &mut monster_last_locs, &graph);
            }else if graph[i][j] == 'A'{
                start_row = i;
                start_col = j;
            }
        }
    }
    if start_row == 0 || start_row == n_and_m[0] - 1 || start_col == 0 || start_col == n_and_m[1] - 1{
        println!("YES");
        println!("0");
        return;
    }
    let mut queue : VecDeque<(usize, usize, usize)> = VecDeque::new();
    let mut found_path = false;
    let mut ans : Option<(usize, usize, usize)> = None;
    let mut parent_dir : Vec<Vec<char>> = vec![vec![' '; n_and_m[1]]; n_and_m[0]];
    let mut last_step : usize = 0;
    queue.push_back((start_row, start_col, 0));
    let mut visited : Vec<Vec<bool>> = vec![vec![false;n_and_m[1]]; n_and_m[0]];
    visited[start_row][start_col] = true;
    while let Some(curr_loc) = queue.pop_front(){
        if last_step != curr_loc.2{
            let mut temp_monster_last_loc: Vec<(usize, usize)> = vec![];
            for monster_last_loc in monster_last_locs{
                monster_spread(monster_last_loc.0, monster_last_loc.1, n_and_m[0], n_and_m[1], &mut monster_locations, &mut temp_monster_last_loc, &graph);
            }
            monster_last_locs = temp_monster_last_loc;
            last_step = curr_loc.2;
        }
        for k in 0..4{
            let new_row = (curr_loc.0 as i16 + ROW_MOVES[k]) as usize;
            let new_col = (curr_loc.1 as i16 + COL_MOVES[k]) as usize;
            if graph[new_row][new_col] == '.' && monster_locations[new_row][new_col] == false && visited[new_row][new_col] == false{
                visited[new_row][new_col] = true;
                if new_row == 0 || new_row == n_and_m[0] - 1 || new_col == 0 || new_col == n_and_m[1] - 1{
                    found_path = true;
                    ans = Some((new_row, new_col, curr_loc.2 + 1));
                    parent_dir[new_row][new_col] = DIRECTIONS[k];
                    break;
                } else{
                    queue.push_back((new_row, new_col, curr_loc.2 + 1));
                    parent_dir[new_row][new_col] = DIRECTIONS[k];
                }
            }
        }
        if found_path{
            break;
        }
    }
    if let Some(dest) = ans {
        let mut path: Vec<char> = Vec::new();
        let mut curr_row = dest.0;
        let mut curr_col = dest.1;
        while (curr_row, curr_col) != (start_row, start_col) {
            let dir = parent_dir[curr_row][curr_col];
            path.push(dir);
            match dir {
                'U' => curr_row += 1,
                'D' => curr_row -= 1,
                'R' => curr_col -= 1,
                'L' => curr_col += 1,
                _ => {}
            }
        }
        path.reverse();
        println!("YES");
        println!("{}", dest.2);
        println!("{}", path.iter().collect::<String>());
    }else{
        println!("NO");
    }
}