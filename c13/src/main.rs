use std::{io::{self, BufRead}, fmt::Display, str::FromStr, error::Error};
use std::collections::HashSet;
use sscanf::scanf;

#[derive(Debug)]
enum Dim { 
    X(),
    Y() 
}

fn main() {
    let lines : Vec<String> = std::io::stdin().lock().lines().flatten().collect();
    let mut dots : Vec<(i32, i32)> = Vec::new();
    let mut folds : Vec<(Dim, i32)> = Vec::new();

    for line in lines {
        if let Some((x,y)) = sscanf::scanf!(line, "{},{}", i32, i32) {
            dots.push((x,y));
        } else if let Some((dim, c)) = sscanf::scanf!(line, "fold along {}={}", String, i32) {
            let dim = match dim.as_str() {
                "x" => Dim::X(),
                "y" => Dim::Y(),
                _   => panic!(),
            };

            folds.push((dim, c));
        }
    }


    for (dim, c) in folds.iter() {
        match dim {
            Dim::X() => {
                for (x,_) in dots.iter_mut() {
                    if *x > *c {
                        *x -= 2*(*x-*c);
                    }
                }
            }
            Dim::Y() => {
                for (_,y) in dots.iter_mut() {
                    if *y > *c {
                        *y -= 2*(*y-*c);
                    }
                }
            }
        }
    }

    let sx = 1+dots.iter().map(|(x,_)| *x).max().unwrap();
    let sy = 1+dots.iter().map(|(_,y)| *y).max().unwrap();
    let uniques : HashSet<(i32, i32)> = HashSet::from_iter(dots);

    for y in 0..sy {
        for x in 0..sx {
            if uniques.contains(&(x,y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    println!("{}", uniques.len());
}
