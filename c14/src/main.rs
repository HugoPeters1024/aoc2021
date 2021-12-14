use std::io::{stdin, BufRead};
use sscanf::scanf;
use std::collections::HashMap;

type Rules = HashMap<(u8, u8), u8>;
type Histogram = HashMap<u8, usize>;
type State = HashMap<(u8,u8), usize>;

fn main() {
    let lines : Vec<String> = stdin().lock().lines().flatten().collect();
    let input : Vec<u8> = lines.get(0).unwrap().to_owned().bytes().collect();
    let mut state = State::new();
    for i in 0..input.len()-1 {
        addState(&mut state, (input[i], input[i+1]), 1);
    }

    let mut aggregate : HashMap<u8, usize> = HashMap::new();
    for c in input {
        *aggregate.entry(c).or_insert(0) += 1;
    }

    let mut rules : Rules = Rules::new();
    for line in lines.iter().skip(2) {
        let (lhs, rhs) = scanf!(line, "{} -> {}", String, String).unwrap();
        let lhs : Vec<u8> = lhs.bytes().collect();
        let rhs = rhs.bytes().collect::<Vec<u8>>()[0];
        let lhs1 = lhs[0];
        let lhs2 = lhs[1];
        rules.insert((lhs1,lhs2), rhs);
    }

    for iii in 0..40 {
        println!("{}: {:?}", iii, state);
        let mut applySet : Vec<(u8,u8,u8,usize)> = vec!();

        for ((lhs, rhs),n) in state.iter() {
            if let Some(c) = rules.get(&(*lhs,*rhs)) {
                applySet.push((*lhs, *rhs, *c, *n));
            }
        }

        for (l,r,c,n) in applySet.iter() {
            *aggregate.entry(*c).or_insert(0) += n;
            removeState(&mut state, (*l, *r), *n);
            addState(&mut state, (*l, *c),*n);
            addState(&mut state, (*c, *r),*n);
        }
    }

    let mut histo = Histogram::new();
    for ((l,r),n) in state.iter() {
        for c in [l,r] {
            if let Some(c) = histo.get_mut(c) {
                *c+=n;
            } else {
                histo.insert(c.to_owned(), *n);
            }
        }
    }

    let most_frequent = aggregate.values().max().unwrap().to_owned();
    let least_frequent = aggregate.values().min().unwrap().to_owned();

    println!("{:?}", aggregate);
    println!("{} - {} = {}", most_frequent, least_frequent, most_frequent - least_frequent);
}

fn addState(s: &mut State, item: (u8, u8), n: usize) {
    *s.entry(item).or_insert(0) += n;
}

fn removeState(s: &mut State, item: (u8, u8), n: usize) {
    *s.entry(item).or_insert(0) -= n;
}
