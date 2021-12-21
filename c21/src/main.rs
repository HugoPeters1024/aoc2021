use std::io::BufRead;
use sscanf::scanf;
use std::collections::HashMap;
use std::cmp::max;
use itertools::iproduct;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
struct State {
    player: u8,
    pos_0: i32,
    pos_1: i32,
    score_0: i32,
    score_1: i32,
}

fn main() {
    let lines: Vec<String> = std::io::stdin().lock().lines().flatten().collect();

    let mut state = State {
        player: 1,
        pos_0: 0,
        pos_1: 0,
        score_0: 0,
        score_1: 0,
    };

    state.pos_0 = scanf!(lines[0], "Player 1 starting position: {}", i32).unwrap() - 1;
    state.pos_1 = scanf!(lines[1], "Player 2 starting position: {}", i32).unwrap() - 1;

    println!("start[0]: {}", state.pos_0+1);
    println!("start[1]: {}", state.pos_1+1);

    let mut states: HashMap<State, usize> = HashMap::new();
    states.insert(state, 1);

    let mut wins : [usize; 2] = [0; 2];

    while !states.is_empty() {
        let mut newstates: HashMap<State, usize> = HashMap::new();
        for (state, occ) in states.iter() {
            if state.player == 0 && state.score_0 >= 21 { wins[0] += occ; continue; }
            if state.player == 1 && state.score_1 >= 21 { wins[1] += occ; continue; }

            for (v1, v2, v3) in iproduct!(1..4, 1..4, 1..4) {
                let mut state = state.clone();
                state.player = (state.player+1)%2;
                if state.player == 0 {
                    state.pos_0 = (state.pos_0+v1+v2+v3)%10;
                    state.score_0 += state.pos_0 + 1;
                } else { 
                    state.pos_1 = (state.pos_1+v1+v2+v3)%10;
                    state.score_1 += state.pos_1 + 1;
                }
                
                *newstates.entry(state).or_insert(0) += occ;
            }
        }

        states = newstates;
    }

    println!("wins[0]: {}, wins[1]: {}, answer: {}", wins[0], wins[1], max(wins[0], wins[1]));
}
