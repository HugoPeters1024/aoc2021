use core::panic;
use std::convert::Infallible;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;
use std::string::ParseError;
extern crate sscanf;
use sscanf::scanf;

enum Dir {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl FromStr for Dir {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((cmd, val)) = scanf!(s, "{} {}", String, i32) {
            match (cmd.as_str(), val) {
                ("forward", i) => Ok(Dir::Forward(i)),
                ("down"   , i) => Ok(Dir::Down(i)),
                ("up"     , i) => Ok(Dir::Up(i)),
                _              => panic!(),
            }
        } else {
            panic!()
        }
    }
}

fn main() {
    let cmds = read_lines("input.txt").unwrap().map(|l| Dir::from_str(l.unwrap().as_str()).unwrap());
    let (h,d,a) = cmds.fold((0,0,0), |(h,d,a), cmd| -> (i32, i32, i32) {
        match cmd {
            Dir::Forward(i) => (h+i, d+i*a, a),
            Dir::Down(i)    => (h, d, a+i),
            Dir::Up(i)      => (h, d, a-i),
        }
    });
    println!("{}", h*d);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
