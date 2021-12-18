#![feature(box_patterns)]
use std::{io::BufRead, fmt::{Display, Formatter}};
use sscanf::scanf;
use itertools::iproduct;
use std::cmp::max;

#[derive(Clone)]
enum Snail {
    SLit(i32),
    SPair(Box<Snail>, Box<Snail>),
    SExplode(i32, i32),
    SCarryL(i32, Box<Snail>),
    SCarryR(i32, Box<Snail>),
}

impl Display for Snail {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        match self {
            Snail::SLit(v) => write!(f,"{}",v),
            Snail::SPair(l,r) => write!(f,"[{},{}]", l, r),
            Snail::SExplode(l,r) => write!(f,"${},{}$",l,r),
            Snail::SCarryL(i,v) => write!(f,"{}(+{}l)",v,i),
            Snail::SCarryR(i,v) => write!(f,"{}(+{}r)",v,i),
        }
    }
}

fn main() {
    let lines : Vec<Snail> = std::io::stdin().lock().lines().flatten().map(|l| parse(l)).collect();

    let mut state = lines[0].clone();

    for i in 1..lines.len() {
        println!("After addition: {}", state);
        state = Snail::SPair(Box::new(state), Box::new(lines[i].clone()));
        eval(&mut state);
    }

    println!("Final: {}", state);
    println!("Magnitude: {}", magnitude(&state));

    let mut max_mag = 0;
    for (l,r) in iproduct!(lines.clone(), lines.clone()) {
        let mut lr = Snail::SPair(Box::new(l.clone()), Box::new(r.clone()));
        let mut rl = Snail::SPair(Box::new(r.clone()), Box::new(l.clone()));
        eval(&mut lr);
        eval(&mut rl);
        max_mag = max(max_mag, magnitude(&lr));
        max_mag = max(max_mag, magnitude(&rl));
    }

    println!("Max sum: {}", max_mag);
}

fn eval(sn: &mut Snail) {
    loop {
        if !(reduce_explode(sn, 0) || reduce_split(sn)) {
            break;
        } 

        //println!("After reduction: {}", sn);

    }
}

fn parse(s : String) -> Snail {
    if let Some(i) = scanf!(s, "{}", i32) {
        Snail::SLit(i)
    } else if let Some(s) = scanf!(s, "[{}]", String) {
        let mut open = 0;
        let mut idx = 0;
        for (i, c) in s.bytes().enumerate() {
            match c {
                b'[' => open += 1,
                b']' => open -= 1,
                b',' => {
                    if open == 0 {
                        idx = i;
                    }
                }
                _ => {}
            }
        }

        let (l,r) = s.split_at(idx);
        let (_, r) = r.split_at(1);
        let l = parse(l.to_string());
        let r = parse(r.to_string());
        Snail::SPair(Box::new(l), Box::new(r))
    } else {
        panic!()
    }
}

fn reduce_explode(s : &mut Snail, depth: i32) -> bool {
    match s {
        Snail::SPair(l, r) => {
            if let box Snail::SExplode(lv, rv) = l.clone() {
                // can't add to the left, add carry object
                *l = Box::new(Snail::SCarryL(lv, Box::new(Snail::SLit(0))));
                *first_left(r) += rv;
                true
            } else if let box Snail::SExplode(lv, rv) = r.clone() {
                // can't add to the right, add carry object
                *r = Box::new(Snail::SCarryR(rv, Box::new(Snail::SLit(0))));
                *first_right(l) += lv;
                true
            } else if let box Snail::SCarryL(i, sn) = l.clone() {
                // Propagate
                *s = Snail::SCarryL(i, Box::new(Snail::SPair(sn, r.clone())));
                true
            } else if let box Snail::SCarryR(i, sn) = r.clone() {
                // Propagate
                *s = Snail::SCarryR(i, Box::new(Snail::SPair(l.clone(), sn)));
                true
            } else if let box Snail::SCarryL(i, sn) = r.clone() {
                *first_right(l) += i;
                *s = Snail::SPair(l.clone(), sn);
                true
            } else if let box Snail::SCarryR(i, sn) = l.clone() {
                *first_left(r) += i;
                *s = Snail::SPair(sn, r.clone());
                true
            } else if depth == 4 {
                let (lv, rv) = match (l,r) {
                    (box Snail::SLit(lv), box Snail::SLit(rv)) => (lv, rv),
                    _ => panic!(),
                };
                *s = Snail::SExplode(*lv, *rv);
                true
            } else {
                reduce_explode(l, depth+1) || reduce_explode(r, depth+1)
            }
        }
        // Could not apply, just unpack
        Snail::SCarryL(_, box sn) => { 
            *s = sn.clone(); 
            true
        }
        Snail::SCarryR(_, box sn) => { 
            *s = sn.clone(); 
            true
        }
        Snail::SLit(_) => false,
        _ => panic!(),
    }
}

fn reduce_split(s : &mut Snail) -> bool {
    match s {
        Snail::SLit(v) => {
            if *v > 9 {
                let (lv, rv) = div2(*v);
                *s = Snail::SPair(Box::new(Snail::SLit(lv)), Box::new(Snail::SLit(rv)));
                true
            } else {
                false
            }
        }
        Snail::SPair(l,r) => reduce_split(l) || reduce_split(r),
        _ => panic!(),
    }
}

fn magnitude(s: &Snail) -> i32 {
    match s {
        Snail::SLit(v) => v.clone(),
        Snail::SPair(l,r) => 3*magnitude(l) + 2*magnitude(r),
        _ => panic!(),
    }
}

fn first_left(s : &mut Snail) -> &mut i32 {
    match s {
        Snail::SLit(v) => v,
        Snail::SPair(l, _) => first_left(l),
        _ => panic!(),
    }
}

fn first_right(s : &mut Snail) -> &mut i32 {
    match s {
        Snail::SLit(v) => v,
        Snail::SPair(_, r) => first_right(r),
        _ => panic!(),
    }
}

fn div2(x: i32) -> (i32, i32) {
    let l = x / 2;
    let mut r = l;
    if l+r < x {
        r+=1;
    }
    (l,r)
}
