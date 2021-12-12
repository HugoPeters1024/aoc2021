use std::{io::{self, BufRead}, fmt::Display, str::FromStr, error::Error, collections::hash_set};
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
extern crate sscanf;
use sscanf::{scanf};
use common_macros::hash_set;

#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Debug)]
#[derive(Clone)]
enum Cave {
    Large(String),
    Small(String)
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().flatten().collect();

    let mut graph : Vec<(Cave, Cave)> = Vec::new();

    for line in lines {
        let (s, e) = scanf!(line, "{}-{}", String, String).unwrap();
        let (cs, ce) = (parseCave(s), parseCave(e));
        graph.push((cs.clone(), ce.clone()));
        graph.push((ce.clone(), cs.clone()));
    }

    let mut paths : Vec<Vec<Cave>> = Vec::new();
    let mut work : VecDeque<Vec<Cave>> = VecDeque::new();
    work.push_front(vec!{Cave::Small(String::from("start"))});

    while !work.is_empty() {
        let path = work.pop_back().unwrap();
        let options : Vec<&Cave> = graph.iter().filter(|(s,_)| s == path.last().unwrap()).map(|(_,e)| e).collect();
        for option in options {
            if let Cave::Small(name) = option {
                if *name == String::from("end") {
                    let mut fp = path.clone();
                    fp.push(option.clone());
                    paths.push(fp);
                    continue;
                } else if *name == String::from("start") {
                    continue;
                }


                if path.contains(option) && pathFull(&path) {
                    continue;
                }
            }

            let mut newpath = path.clone();
            newpath.push(option.clone());
            work.push_front(newpath);
        }
    }

    
    println!("{:?}", paths.len());
}

fn pathFull(inp: &Vec<Cave>) -> bool {
    let mut set : HashSet<String>  =HashSet::new();
    let mut count = 0;
    for cave in inp.iter() {
        if let Cave::Small(name) = cave {
            set.insert(name.to_owned());
            count += 1;
        }
    }

    set.len() != count
}

fn parseCave(inp: String) -> Cave {
    if inp.to_uppercase() == inp {
        Cave::Large(inp)
    } else {
        Cave::Small(inp)
    }
}
