use std::{io::{self, BufRead}, fmt::Display, str::FromStr, error::Error};
use parse_int::parse;
use std::collections::HashSet;

type Basin = HashSet<(usize, usize)>;

struct Level {
    data: Vec<u8>,
    width : usize,
    height: usize,
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().flatten().collect();

    let width = lines[0].len();
    let height = lines.len();
    let mut level = Level {
        width,
        height,
        data: vec![0; width*height],
    };

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.bytes().enumerate() {
            level.data[y*width+x] = (c as i32 - 0x30) as u8;
        }
    }

    let mut count = 0;


    for y in 0..height {
        for x in 0..width {
            let me = try_index(&level, x, y).unwrap();
            if neightbours(&level, x, y).iter().all(|o| *o > me) {
                count += (me + 1) as usize;
            }
        }
    }

    let mut basins: Vec<Basin> = vec!();
    let mut seen : Basin = HashSet::new();
    for y in 0..height {
        for x in 0..width {
            if !seen.contains(&(x,y)) {
                let mut basin : Basin = HashSet::new();
                find_basin(&level, x, y, &mut basin);
                if basin.len() == 0 { continue; }
                basins.push(basin.clone());
                seen.extend(basin.iter());
            }
        }
    }

    basins.sort_by(|a, b| b.len().partial_cmp(&a.len()).unwrap());
    let b1 = basins[0].len();
    let b2 = basins[1].len();
    let b3 = basins[2].len();

    println!("{}x{}x{}={}", b1, b2, b3, b1 * b2* b3);
}

fn find_basin(level: &Level, x: usize, y: usize, basin : &mut Basin) {
    if basin.contains(&(x,y)) { return; }
    let me = try_index(level, x, y).unwrap();
    if me != 9 {
        basin.insert((x,y));
        if x > 0 { find_basin(level, x-1, y, basin); }
        if y > 0 { find_basin(level, x, y-1, basin); }
        if x+1 < level.width { find_basin(level, x+1, y, basin); }
        if y+1 < level.height { find_basin(level, x, y+1, basin); }
    }
}

fn try_index(level: &Level, x: usize, y: usize) -> Option<u8> {
    if x < level.width && y < level.height {
        Some(level.data[(y*level.width+x) as usize])
    } else {
        None
    }
}

fn neightbours(level: &Level, x: usize, y: usize) -> Vec<u8> {
    vec!(try_index(level, x+1, y),
         try_index(level, x, y+1),
         if x > 0 { try_index(level, x-1, y) } else { None },
         if y > 0 {try_index(level, x, y-1)} else {None}).iter().filter_map(|v| *v).collect()
}
