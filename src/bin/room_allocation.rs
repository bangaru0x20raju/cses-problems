/*
 * Author: Cheerapa Bangaru Raju
 */

#![allow(unused)]

use std::{collections::{BTreeSet, VecDeque}, fmt::Debug, io, str::FromStr};

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
    let n : usize = read_number();
    let mut timings : BTreeSet<(u64, i8, usize)> = BTreeSet::new();
    for i in 0..n{
        let range : Vec<u64> = read_vector();
        timings.insert((range[0], 1, i));
        timings.insert((range[1], 2, i));
    }
    let mut get_room : Vec<u32> = Vec::new(); get_room.resize(n, 0);
    let mut rooms : VecDeque<u32> = VecDeque::new();
    let mut max_rooms: u32 = 0;
    for interval_time in timings.into_iter(){
        if interval_time.1 == 1 { 
            if rooms.is_empty(){
                max_rooms+=1;
                get_room[interval_time.2] = max_rooms;
            }else{
                get_room[interval_time.2] = rooms.pop_front().unwrap();
            }
        }else{
            rooms.push_back(get_room[interval_time.2]);
        }
    }

    println!("{max_rooms}");
    for i in get_room{
        print!("{i} ");
    }
}