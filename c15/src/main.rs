use std::io::{stdin, BufRead};
use std::collections::HashSet;
use std::cmp::{max};
extern crate priority_queue;
use priority_queue::{*};

type Node = (usize, usize, i64);
type Q = PriorityQueue<Node, i64>;

#[derive(Debug)]
struct Level {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

fn main() {
    let lines : Vec<Vec<u8>> = stdin().lock().lines().flatten().map(|v| v.bytes().map(|c| c-0x30).collect()).collect();
    let original_data : Vec<u8> = lines.to_owned().into_iter().flatten().collect();


    let w = lines[0].len();
    let h = lines.len();

    let mut level = Level {
        width: w*5,
        height: h*5,
        data: vec![0; 5*5*w*h],
    };

    for ny in 0..5 {
        for nx in 0..5 {
            for y in 0..h {
                for x in 0..w {
                    let inc = (nx + ny) as u8;
                    let tx = nx * w + x;
                    let ty = ny * h + y;
                    let mut val = original_data[y*w+x] + inc; 
                    while val > 9 {
                        val -= 9;
                    }
                    level.data[ty*level.width+tx] = val;
                }
            }
        }
    }

    // We use the inverse of risk with some arbitray high number as a priority value
    let MAX_RISK : i64 = 10000000000;

    let mut queue = Q::new(); 
    queue.push((0,0,MAX_RISK), MAX_RISK);

    let mut visited : HashSet<(usize, usize)> = HashSet::new();

    while !queue.is_empty() {
        let ((x,y,_), priority) = queue.pop().unwrap();

        if visited.contains(&(x,y)) {
            continue;
        }
        visited.insert((x,y));

        // Convert back to actual risk
        let total_risk = MAX_RISK - priority;

        if x == level.width-1 && y == level.height-1 {
            println!("final risk {}", total_risk);
        }

        for (x,y) in neighbours(&level,x,y) {
            let new_risk = total_risk + (level.data[y*level.width+x] as i64);
            let new_priority = MAX_RISK - new_risk;
            queue.push((x,y,new_priority), new_priority);
        }
    }
}

fn neighbours(l: &Level, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut ret = vec!();
    if x + 1 < l.width { ret.push((x+1, y)); }
    if y + 1 < l.height { ret.push((x, y+1)); }
    if x > 0 { ret.push((x-1, y)); }
    if y > 0 { ret.push((x, y-1)); }
    ret
}


