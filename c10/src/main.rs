use std::{io::{self, BufRead}, fmt::Display, str::FromStr, error::Error};
use std::collections::VecDeque;
use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();
    let lines: Vec<Vec<u8>> = stdin.lock().lines().flatten().map(|x| x.bytes().collect()).collect();

    let mut openers : HashSet<u8> = HashSet::new();
    openers.insert(b'(');
    openers.insert(b'[');
    openers.insert(b'{');
    openers.insert(b'<');

    let mut closers : HashSet<u8> = HashSet::new();
    closers.insert(b')');
    closers.insert(b']');
    closers.insert(b'}');
    closers.insert(b'>');

    let mut uncorrupted : Vec<Vec<u8>> = vec!();

    let mut score = 0;
    'outer: for line in lines {
        let mut stack : VecDeque<u8> = VecDeque::new();
        for c in line.iter() {
            if openers.contains(c) {
                stack.push_back(*c);
            } else {
                let front = stack.pop_back().unwrap();
                if *c != closer(&front) {
                    println!("Expected {}, but got {}", closer(&front), c);
                    score += points(c);
                    continue 'outer;
                }
            }
        }
        uncorrupted.push(line.clone());
    }
    println!("{}", score);

    println!("{:?}", uncorrupted.len());

    let mut scores : Vec<i64> = Vec::new();

    for line in uncorrupted.iter() {
        let mut stack: VecDeque<u8> = VecDeque::new();
        for c in line.iter() {
            if openers.contains(c) {
                stack.push_back(*c);
            } else {
                let _ = stack.pop_back();
            }
        }

        let mut score = 0;
        while !stack.is_empty() {
            let front = stack.pop_back().unwrap();
            score = score * 5 + points2(&closer(&front));
        }
        scores.push(score);
    }

    scores.sort();
    println!("{:?}", scores[(scores.len() / 2) as usize]);
}

fn closer(x: &u8) -> u8 {
    match x {
        b'(' => b')',
        b'[' => b']',
        b'{' => b'}',
        b'<' => b'>',
        _ => panic!(),
    }
}

fn points(x: &u8) -> i32 {
    match x {
        b')' => 3,
        b']' => 57,
        b'}' => 1197,
        b'>' => 25137,
        _ => panic!(),
    }
}

fn points2(x: &u8) -> i64 {
    match x {
        b')' => 1,
        b']' => 2,
        b'}' => 3,
        b'>' => 4,
        _ => panic!(),
    }
}
