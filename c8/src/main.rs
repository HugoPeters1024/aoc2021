use std::{io::{self, BufRead}, fmt::Display, str::FromStr, error::Error};
extern crate scanf;
use scanf::scanf;
use std::collections::HashSet;

// bit encoded
type Pattern = u8;

// which bit maps to which other bit
type Mapping = [u8; 7];

#[derive(Debug)]
struct Case {
    patterns: [Pattern; 10],
    data: [Pattern; 4],
}

fn main() {
    let stdin = io::stdin();
    let cases: Vec<Case> = stdin.lock().lines().flatten().map(|l| parse_case(&l)).collect();

    let mut counter = 0;
    for case in cases.iter() {
        for item in case.data {
            let l = count(&item);
            if l == 2 || l == 3 || l == 4 || l == 7 {
                counter+=1;
            }
        }
    }
    println!("{}", counter);

    // Generate all unique mappings, ugh
    let mut allMappings : Vec<Mapping> = vec!();
    for x1 in 0..7 {
    for x2 in 0..7 {
    for x3 in 0..7 {
    for x4 in 0..7 {
    for x5 in 0..7 {
    for x6 in 0..7 {
    for x7 in 0..7 {
        let mut s = HashSet::new();
        s.insert(x1);
        s.insert(x2);
        s.insert(x3);
        s.insert(x4);
        s.insert(x5);
        s.insert(x6);
        s.insert(x7);
        if s.len() == 7 {
            let m = [x1,x2,x3,x4,x5,x6,x7];
            allMappings.push(m);
        }
    }}}}}}}

    println!("Generated all {} mappings", allMappings.len());

    let mut sum = 0;
    for case in cases.iter() {
        for mapping in allMappings.iter() {
            if mapping_valid(case, mapping) {
                // case solved
                let n1 = try_convert(case.data[0], mapping).unwrap();
                let n2 = try_convert(case.data[1], mapping).unwrap();
                let n3 = try_convert(case.data[2], mapping).unwrap();
                let n4 = try_convert(case.data[3], mapping).unwrap();
                let n = n1 * 1000 + n2 * 100 + n3 * 10 + n4;
                sum += n;
            }
        }
    }

    println!("{}", sum);
}

// Apply mapping to pattern
fn lookup(x: Pattern, m: &Mapping) -> u8 {
    let mut ret = 0;

    for i in 0..7 {
        // bit was enabled
        if (x & (1 << i)) != 0 {
            ret |= 1 << m[i as usize];
        }
    }
    ret
}

fn mapping_valid(case: &Case, mapping: &Mapping) -> bool {
    case.patterns.iter().filter_map(|x| try_convert(*x, mapping)).count() == 10 
}

fn try_convert(x: Pattern, m: &Mapping) -> Option<i32> {
    let c = lookup(x, m);
    if c == parse_pattern(String::from("abcefg")) { Some(0) }
    else if c == parse_pattern(String::from("cf")) { Some(1) }
    else if c == parse_pattern(String::from("acdeg")) { Some(2) }
    else if c == parse_pattern(String::from("acdfg")) { Some(3) }
    else if c == parse_pattern(String::from("bcdf")) { Some(4) }
    else if c == parse_pattern(String::from("abdfg")) { Some(5) }
    else if c == parse_pattern(String::from("abdefg")) { Some(6) }
    else if c == parse_pattern(String::from("acf")) { Some(7) }
    else if c == parse_pattern(String::from("abcdefg")) { Some(8) }
    else if c == parse_pattern(String::from("abcdfg")) { Some(9) }
    else { None }

}

fn parse_case(inp: &String) -> Case {
    let mut ret = Case {
        patterns: [0; 10],
        data: [0; 4],
    };

    let items : Vec<&str> = inp.split(' ').collect();
    for (p, item) in ret.patterns.iter_mut().zip(items.clone()) {
        *p = parse_pattern(String::from_str(item.clone()).unwrap());
    }

    for (i, d) in ret.data.iter_mut().enumerate() {
        *d = parse_pattern(String::from_str(items[i+11]).unwrap());
    }

    ret
}

fn parse_pattern(inp: String) -> Pattern {
    let mut ret = 0;
    for c in inp.bytes() {
        ret |= match c {
            b'a' => 1,
            b'b' => 2,
            b'c' => 4,
            b'd' => 8,
            b'e' => 16,
            b'f' => 32,
            b'g' => 64,
            _ => panic!(),
        }
    }
    ret
}

fn count(p: &Pattern) -> i32 {
    let mut mask = 1;
    let mut ret = 0;
    for _ in 0..7 {
        if p & mask != 0 {
            ret += 1;
        }
        mask *= 2;
    }
    ret

}
