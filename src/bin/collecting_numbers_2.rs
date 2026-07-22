/*
 * Author: Cheerapa Bangaru Raju
 */

#![allow(unused)]

use core::time;
use std::{cmp::Ordering, collections::{BTreeSet, HashMap}, fmt::Debug, io, ptr::swap, str::FromStr};

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
    let temp_vector : Vec<i32> = read_vector();
    let n : i32 = temp_vector[0];
    let mut m : i32 = temp_vector[1];
    let mut v : Vec<i32> = read_vector();
    let mut my_map: HashMap<i32, i32> = HashMap::new();
    let mut index = 1;
    for ele in v.iter() { 
        my_map.insert(*ele, index);
        index+=1;
    }
    v.insert(0, 0);
    my_map.insert(0, 0);
    let mut rounds = 1;
    for ele in 1..=n { 
        let temp = ele-1;
        if my_map.get(&ele).unwrap() < my_map.get(&temp).unwrap(){
            rounds+=1;
        }
    }
    let mut swap_set: BTreeSet<(i32, i32)> = BTreeSet::new();
    while m!=0{
        m-=1;
        let temp_tuple : Vec<i32> = read_vector();
        let i_index_ele = v[temp_tuple[0] as usize];
        let j_index_ele = v[temp_tuple[1] as usize];
        
        if i_index_ele -1 >0 {
            swap_set.insert((i_index_ele-1, i_index_ele));
        }
        if i_index_ele + 1 <= n {
            swap_set.insert((i_index_ele, i_index_ele+1));
        }
        if j_index_ele - 1 > 0 { 
            swap_set.insert((j_index_ele-1, j_index_ele));
        }
        if j_index_ele + 1 <= n { 
            swap_set.insert((j_index_ele, j_index_ele+1));
        }
        //println!("{:?}",swap_set);
        for temp_swap_set in swap_set.iter(){
            if my_map.get(&temp_swap_set.0).unwrap() > my_map.get(&temp_swap_set.1).unwrap() { 
                rounds-=1;
            }
        }
        //println!("Before : {rounds}");
        my_map.insert(i_index_ele, temp_tuple[1]);
        my_map.insert(j_index_ele, temp_tuple[0]);
        v[temp_tuple[0] as usize] = j_index_ele;
        v[temp_tuple[1] as usize] = i_index_ele;
        //println!("{:?}", my_map);
        for temp_swap_set in swap_set.iter(){
            if my_map.get(&temp_swap_set.0).unwrap() > my_map.get(&temp_swap_set.1).unwrap() { 
                rounds+=1;
            }
        }
        swap_set.clear();
        println!("{rounds}");
        //println!("{:?}", v);
    }
}