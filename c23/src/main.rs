use std::{io::BufRead, fmt::Display};
use sscanf::scanf;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use priority_queue::PriorityQueue;
use std::cmp::min;

#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
enum AmphiKind {A(), B(), C(), D() }
type Amphi = (AmphiKind, i32, i32);

#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
struct State {
    energy_spent: i64,
    amphis: [Amphi; 8],
}

impl State {
    fn get_dict(&self) -> HashMap<(i32, i32), AmphiKind> {
        self.amphis.iter().map(|(k,x,y)| ((*x,*y), *k)).collect()
    }

    fn is_free(&self, x: i32, y: i32) -> bool {
        for (_, xo, yo) in self.amphis {
            if x == xo && y == yo {
                return false;
            }
        }
        true
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let dict = self.get_dict();
        for y in 0..3 {
            for x in 0..11 {
                if let Some(k) = dict.get(&(x,y)) {
                    write!(f, "{}", kind_to_char(k)).unwrap();
                } else {
                    write!(f, " ").unwrap();
                }
            }
            writeln!(f).unwrap();
        }
        writeln!(f, "Energy spent: {}", self.energy_spent)
    }
}

fn main() {
    let state =  parse();

    let mut discovery: PriorityQueue<State, i64> = PriorityQueue::new();
    let mut visited: HashMap<[Amphi;8], i64> = HashMap::new();

    discovery.push(state, 0);
    let mut best: i64 = 100000000000;
    while let Some((state, _)) = discovery.pop() {
        for state in succs(&state) {
            if let Some(e) = visited.get(&state.amphis) {
                // Beter solution already known, continue
                if *e <= state.energy_spent { continue; }
            }
            *visited.entry(state.amphis).or_insert(0) = state.energy_spent;

            if is_correct(&state.amphis) {
                println!("Solution:");
                println!("{}", state);
                best = min(best, state.energy_spent);
            }

            discovery.push(state, -state.energy_spent);
        }
    }

    println!("best solution: {}", best);
}

fn is_correct(s: &[Amphi;8]) -> bool {
    for (k, x, y) in s.into_iter() {
        match (x,y) {
            (2,1) => if *k != AmphiKind::A() { return false; },
            (2,2) => if *k != AmphiKind::A() { return false; },
            (4,1) => if *k != AmphiKind::B() { return false; },
            (4,2) => if *k != AmphiKind::B() { return false; },
            (6,1) => if *k != AmphiKind::C() { return false; },
            (6,2) => if *k != AmphiKind::C() { return false; },
            (8,1) => if *k != AmphiKind::D() { return false; },
            (8,2) => if *k != AmphiKind::D() { return false; },
            _ => { return false },
        }

    }
    true
}

fn valid_end(s: &State, k: AmphiKind, x: i32, y: i32) -> bool {
    let (rx1, ry1, rx2, ry2) = match k {
        AmphiKind::A() => (2,1,2,2),
        AmphiKind::B() => (4,1,4,2),
        AmphiKind::C() => (6,1,6,2),
        AmphiKind::D() => (8,1,8,2),
    };

    if x == rx2 && y == ry2 {
        true
    } else if x == rx1 && y == ry1 {
        if let Some(ko) = s.get_dict().get(&(x,2)) {
            k == *ko
        } else {
            false
        }
    } else {
        false
    }
}


fn succs(s: &State) -> Vec<State> {
    let mut ret = Vec::new();


    for (i, (k,x,y)) in s.amphis.into_iter().enumerate() {
        // Accept only going to a room or going to the hallway
        let accept = if y > 0 {
            // you started in a room, just don't got anywhere forbidden
            |s: &State, k: AmphiKind, x: i32, y: i32| valid_end(s, k, x, y) || (x != 2 && x != 4 && x!=6 && x!=8)
        } else {
            // you started in the hallway, move to your room
            |s: &State, k: AmphiKind, x: i32, y: i32| valid_end(s, k, x, y)
        };


        if valid_end(s, k, x, y) { continue; }

        let mut discovery: VecDeque<State> = VecDeque::new();
        let mut visisted: HashSet<(i32, i32)> = HashSet::new();

        discovery.push_back(s.clone());

        while let Some(s) = discovery.pop_front() {
            let (k,x,y) = s.amphis[i];
            if visisted.contains(&(x,y)) { continue; }
            visisted.insert((x,y));
            for (x,y) in coord_succs(x, y) {
                // Check if it is free
                if s.is_free(x,y) {
                    let mut ns = s.clone();
                    ns.amphis[i] = (k,x,y);
                    ns.energy_spent += cost(k);
                    discovery.push_back(ns);

                    // Check the accepting conditions
                    if accept(&ns,k, x,y) {
                        ret.push(ns);
                    }
                }
            }
        }
    }

    ret
}

fn coord_succs(x: i32, y: i32) -> Vec<(i32, i32)> {
    let mut ret = Vec::new();
    if y > 0 {
        ret.push((x, y-1));
    } else {
        if x > 0 { ret.push((x-1, 0)) }
        if x < 12 { ret.push((x+1, 0)) }
    }

    if [2,4,6,8].contains(&x) && y < 2 {
        ret.push((x,y+1));
    }
    ret
}




fn cost(k: AmphiKind) -> i64 {
    match k {
        AmphiKind::A() => 1,
        AmphiKind::B() => 10,
        AmphiKind::C() => 100,
        AmphiKind::D() => 1000,
    }
}

fn parse() -> State {
    let lines: Vec<String> = std::io::stdin().lock().lines().flatten().collect();
    let (a1,b1,c1,d1) = scanf!(lines[2], "###{}#{}#{}#{}###", char, char, char, char).unwrap();
    let (a2,b2,c2,d2) = scanf!(lines[3], "  #{}#{}#{}#{}#", char, char, char, char).unwrap();

    let amphis = [
        (char_to_kind(a1), 2,1),
        (char_to_kind(a2), 2,2),
        (char_to_kind(b1), 4,1),
        (char_to_kind(b2), 4,2),
        (char_to_kind(c1), 6,1),
        (char_to_kind(c2), 6,2),
        (char_to_kind(d1), 8,1),
        (char_to_kind(d2), 8,2),
    ];

    State {
        energy_spent: 0,
        amphis,
//        nr_moves: [0; 4],
    }
}

fn char_to_kind(c: char) -> AmphiKind {
    match c {
        'A' => AmphiKind::A(),
        'B' => AmphiKind::B(),
        'C' => AmphiKind::C(),
        'D' => AmphiKind::D(),
        _ => panic!(),
    }
}

fn kind_to_char(k: &AmphiKind) -> char {
    match k {
        AmphiKind::A() => 'A',
        AmphiKind::B() => 'B',
        AmphiKind::C() => 'C',
        AmphiKind::D() => 'D',
    }
}
