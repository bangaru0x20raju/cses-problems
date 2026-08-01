/*
 * Author: Cheerapa Bangaru Raju
 */

#![allow(unused)]

use std::io::{self, Read, Write, BufWriter};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();

    let stride = n + 1;
    let mut up = vec![0usize; 30 * stride];

    for j in 1..=n {
        up[j] = iter.next().unwrap().parse().unwrap();
    }

    for i in 1..30 {
        let prev = (i - 1) * stride;
        let cur = i * stride;
        for j in 1..=n {
            up[cur + j] = up[prev + up[prev + j]];
        }
    }

    for _ in 0..q {
        let mut x: usize = iter.next().unwrap().parse().unwrap();
        let k: u32 = iter.next().unwrap().parse().unwrap();
        for i in 0..30 {
            if k & (1 << i) > 0 {
                x = up[i * stride + x];
            }
        }
        writeln!(out, "{x}").unwrap();
    }
}