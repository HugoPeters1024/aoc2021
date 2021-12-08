use std::{fs::File};
use std::io::{BufReader, BufRead};
extern crate queues;
use queues::*;

fn main() 
{
    let f = File::open("input.txt").unwrap();
    let lines = BufReader::new(f).lines()
                                 .map(|l| l.unwrap())
                                 .map(|l| l.parse::<i32>().unwrap());

    let mut rolling : Queue<i32> = Queue::new();
    let mut newList : Vec<i32> = Vec::new();
    for nr in lines {
        rolling.add(nr);
        if rolling.size() == 3 {
            let i0 = rolling.remove().unwrap();
            let i1 = rolling.remove().unwrap();
            let i2 = rolling.remove().unwrap();
            newList.push(i0 + i1 + i2);
            rolling.add(i1);
            rolling.add(i2);

        }
    }

    let mut prev : Option<i32> = Option::None;
    let mut count = 0;
    for nr in newList {
        count += match prev {
            None => 0,
            Some(prev) => if nr > prev {1} else {0},
        };
        prev = Some(nr);
    }
    println!("{}", count);
}
