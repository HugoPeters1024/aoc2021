use std::{io::{self, BufRead}, fmt::Display};
use parse_int::parse;
use std::fmt;

#[derive(PartialEq)]
#[derive(Clone)]
struct Board {
    data : Vec<(i32, bool)>,
}

impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> { 
        for i in 0..5 {
            let to_print : Vec<(i32, bool)> = self.data.iter().cloned().skip(5*i).take(5).collect();
            for v in to_print {
                write!(f, "{:?} ", v).unwrap();
            }
            writeln!(f).unwrap();
        }

        writeln!(f)
    }
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().flatten().collect();
    let numbers : Vec<i32> = lines.first().unwrap().split(',').map(|v| parse::<i32>(v).unwrap()).collect();

    let mut boards: Vec<Board> = vec!();
    let mut offset = 2;
    while lines.len() >= 5 + offset {
        boards.push(read_board(&lines.iter().skip(offset).take(5).collect()));
        offset += 6
    }

    println!("nr boards: {}", boards.len());

    let mut count_win = 0;
    let mut has_won : Vec<bool> = vec![false; boards.len()];
    let mut last_won = 0;
    for lot in numbers {
        for (bi, board) in boards.iter_mut().enumerate() {
            if update_board(board, lot) {
                if !has_won[bi] {
                    has_won[bi] = true;
                    count_win+=1;
                    last_won = bi;
                }
            }
        }
        if count_win == boards.len() {
            println!("{}", get_score(&boards[last_won], lot));
            return;
        }
    }

}

fn read_board(inp: &Vec<&String>) -> Board {
    let mut data : Vec<i32> = vec!();
    for i in 0..5 {
        let mut numbers : Vec<i32> = inp[i].split(' ').filter(|v| v.len() > 0).map(|v| parse::<i32>(v).unwrap()).collect();
        data.append(&mut numbers);
    }

    Board {
        data: data.iter().map(|v| *v).zip(vec![false; 25]).collect(),
    }
}

fn update_board(board: &mut Board, lot: i32) -> bool {
    for (bv, bm) in board.data.iter_mut() {
        if *bv == lot { *bm = true; }
    }

    // check rows
    for y in 0..5 {
        let mut done = true;
        for x in 0..5 {
            let (_, m) = board.data[5*y+x];
            done &= m;
        }

        if done { return true; }
    }

    // check columns
    for x in 0..5 {
        let mut done = true;
        for y in 0..5 {
            let (_, m) = board.data[5*y+x];
            done &= m;
        }

        if done { return true; }
    }

    return false;
}

fn get_score(board: &Board, lot: i32) -> i32 {
    let mut sum = 0;
    for (bv, bm) in &board.data {
        if !bm { sum += bv; }
    }

    sum * lot
}
