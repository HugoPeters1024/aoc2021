use std::{io::{self, BufRead}, fmt::Display};
use std::cmp::{min,max};

#[derive(Debug)]
struct Line {
    start : (i32, i32),
    end: (i32, i32),
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<Line> = stdin.lock().lines().flatten()
        .map(|x| parse_line(&x))
        //.filter(|l| l.start.0 == l.end.0 || l.start.1 == l.end.1)
        .collect();
    let sx = 1+lines.iter().map(|l| max(l.start.0, l.end.0)).max().unwrap();
    let sy = 1+lines.iter().map(|l| max(l.start.1, l.end.1)).max().unwrap();
    let grid_size = sx * sy;
    let mut grid : Vec<i32> = vec![0; grid_size as usize];
    println!("grid size: {}x{}={}", sx, sy, grid.len());
    for line in lines {
        println!("{:?}", line);
        let (mut x, mut y) = line.start;
        let dirx = sign(line.end.0 - line.start.0);
        let diry = sign(line.end.1 - line.start.1);
        println!("dir=({},{})", dirx, diry);
        loop {
            let should_break = x == line.end.0 && y == line.end.1;
            grid[(sx*y+x) as usize] += 1;
            x += dirx;
            y += diry;
            println!("pos: {},{}", x, y);
            if should_break { break; }
        }
    }

    let answer = grid.iter().filter(|&v| *v>=2).collect::<Vec<&i32>>().len();
    for y in 0..sy {
        for x in 0..sx {
            print!("{} ", grid[(sx*y+x) as usize]);
        }
        println!();
    }
    println!("{:?}", answer);
}

fn sign(v: i32) -> i32 {
    match v {
        0 => 0,
        v => if v < 0 {-1} else {1},
    }
}

fn parse_line(line: &String) -> Line {
    let (mut x0, mut y0, mut x1, mut y1) = (0,0,0,0);
    if let Ok(_) = scanf::sscanf!(line, "{},{} -> {},{}", x0, y0, x1, y1) {
        Line {
            start: (x0, y0),
            end: (x1, y1),
        }
    } else {
        println!("cannot parse {}", line);
        panic!();
    }
}
