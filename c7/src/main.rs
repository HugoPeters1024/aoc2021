use std::{io::{self, BufRead}, fmt::Display};
extern crate parse_int;
use parse_int::parse;

fn main() {
    let stdin = io::stdin();
    let mut inp = String::from("");
    io::stdin().read_line(&mut inp).unwrap();
    let input : Vec<i32> = inp.split(',').map(|v| parse::<i32>(v).unwrap()).collect();
    let s : i32 = 1 + *input.iter().max().unwrap();

    let mut costs : Vec<i32> = vec![0; s as usize];

    for (tp, cost) in costs.iter_mut().enumerate() {
        for crab in input.iter() {
            *cost += c((tp as i32 - crab).abs());
        }
    }

    println!("{:?}", costs.iter().min());
}

fn c(n : i32) -> i32 {
    (n * n + n) / 2
}

