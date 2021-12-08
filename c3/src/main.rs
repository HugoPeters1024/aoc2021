use std::io::{self, BufRead};

fn main() {
    println!("Hello, world!");

    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().flatten().collect();

    let bitarrays: Vec<Vec<u8>> = lines.iter().map(|x| x.as_bytes().to_vec().iter().map(|c| match *c as char {
        '1' => 1,
        '0' => 0,
        _   => panic!(),
    }).collect()).collect();

    //let mut epsilon: Vec<u8> = vec![0; 12];
    //for b in 0..12 {
    //    let (zeroes, ones) = bitarrays.iter().fold((0,0), |(zeroes, ones),l| {
    //        if l[b] == '1' as u8 {
    //            (zeroes, ones+1)
    //        } else if l[b] == '0' as u8 {
    //            (zeroes+1, ones)
    //        } else {
    //            panic!();
    //        }
    //    });

    //    let bit = if zeroes > ones {0} else {1};
    //    epsilon[b] = bit;
    //}

    //println!("{}", toDecimal(&epsilon) * toDecimal(&invert(&epsilon)));
    println!("---- part 2 ------");

    let mut oxygenList = bitarrays.clone();
    let mut co2List = bitarrays.clone();
    // Find oxygen
    for b in 0..12 {
        if oxygenList.len() > 1 {
            let mut keepList : Vec<Vec<u8>> = vec![vec![0;0];0];

            let (z, o) = freq(&oxygenList, b);
            let to_delete = if z > o {1} else {0};

            for item in &oxygenList {
                if item[b] != to_delete {
                    keepList.push(item.clone());
                }
            }

            oxygenList = keepList;
        }

        if co2List.len() > 1 {
            let mut keepList : Vec<Vec<u8>> = vec![vec![0;0];0];

            let (z, o) = freq(&co2List, b);
            let to_delete = if z > o {0} else {1};

            for item in &co2List {
                if item[b] != to_delete {
                    keepList.push(item.clone());
                }
            }

            co2List = keepList;
        }
    }

    println!("{}", toDecimal(&oxygenList[0]) * toDecimal(&co2List[0]));
}

fn freq(inp: &Vec<Vec<u8>>, pos: usize) -> (i32, i32) {
    let (mut z, mut o) = (0,0);
    for val in inp {
        match val[pos] {
            0 => z+=1,
            1 => o+=1,
            _ => ()
        }
    }
    (z, o)
}

fn invert(inp: Vec<u8>) -> Vec<u8> {
    inp.iter().map(|v| -> u8 {
        match v {
            0 => 1,
            1 => 0,
            x => *x,
        }
    }).collect()
}

fn toDecimal(inp: &Vec<u8>) -> i32 {
    let mut acc = 0;
    for b in inp {
        acc = 2 * acc + *b as i32
    }
    ac
}
