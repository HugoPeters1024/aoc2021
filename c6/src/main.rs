use std::{io::{self, BufRead}, fmt::Display};
extern crate parse_int;
use parse_int::parse;

fn main() {
    let stdin = io::stdin();
    let mut inp = String::from("");
    io::stdin().read_line(&mut inp).unwrap();
    let input : Vec<i32> = inp.split(',').map(|v| parse::<i32>(v).unwrap()).collect();

    let mut state = vec![0; 9];
    for nr in input {
        state[nr as usize] += 1;
    }

    for day in 0..257 {
        let l : i64 = state.iter().sum();
        println!("day {}: {}", day, l);

        let to_spawn = state[0];
        for i in 1..9 {
            state[i-1] = state[i];
        }

        state[6] += to_spawn;
        state[8] = to_spawn;
    }
}

fn main_part1() {
    let stdin = io::stdin();
    let mut inp = String::from("");
    io::stdin().read_line(&mut inp).unwrap();
    let mut state : Vec<i32> = inp.split(',').map(|v| parse::<i32>(v).unwrap()).collect();


    for day in 0..257 {
        println!("day {}: {:?}", day, state.len());
        let mut spawn : Vec<i32> = vec!();
        for fish in state.iter_mut() {
            if *fish == 0 {
                *fish = 6;
                spawn.push(8)
            } else {
                *fish -= 1;
            }

        }
        state.append(&mut spawn);
    }
}

