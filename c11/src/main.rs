use std::{io::{self, BufRead}, fmt::Display, str::FromStr, error::Error};
use std::collections::HashSet;


struct Level {
    data: Vec<u8>,
    width : usize,
    height: usize,
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<Vec<u8>> = stdin.lock().lines().flatten().map(|v| v.bytes().collect()).collect();

    let w = lines[0].len();
    let h = lines.len();

    let mut level = Level {
        data: vec![0; w*h],
        width: w,
        height: h,
    };

    for y in 0..h {
        for x in 0..w {
            level.data[y*w+x] = lines[y][x] - 0x30;
        }
    }

    let mut total = 0;
    for i in 0..10000000 {
        for y in 0..level.height {
            for x in 0..level.width {
                level.data[y*level.width+x] += 1;
            }
        }

        let mut has_flashed : HashSet<(usize, usize)> = HashSet::new();
        let mut changed = true;
        while changed {
            changed = false;
            for y in 0..level.height {
                for x in 0..level.width {
                    if level.data[y*level.width+x] > 9 && !has_flashed.contains(&(x,y)) {
                        changed = true;
                        total += 1;
                        has_flashed.insert((x,y));
                        for n in 0..8 {
                            if let Some(idx) = index(&level, x, y, n) {
                                level.data[idx] += 1;
                            }
                        }
                    }
                }
            }
        }

        if has_flashed.len() == w*h {
            println!("all flashed at step {}", i);
            return;
        }

        for (x,y) in has_flashed {
            level.data[y*level.width+x] = 0;
        }
    }

    println!("{}", total);
}

fn index(level: &Level, x: usize, y: usize, n: i32) -> Option<usize> {
    if let Some((x, y)) = match n {
        0 => Some((x+1,y)),
        1 => Some((x+1,y+1)),
        2 => Some((x,y+1)),
        3 => if x > 0 { Some((x-1,y+1)) } else { None },
        4 => if x > 0 { Some((x-1,y)) } else { None },
        5 => if x > 0 && y > 0 { Some((x-1,y-1)) } else { None },
        6 => if y > 0 { Some((x,y-1)) } else { None },
        7 => if y > 0 { Some((x+1,y-1)) } else { None },
        _ => panic!(),
    } {
        if x < level.width && y < level.height {
            Some(y*level.width+x)
        } else {
            None
        }
    } else {
        None
    }
}
