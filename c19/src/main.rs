use std::io::BufRead;
use sscanf::scanf;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;
extern crate itertools;
use itertools::{Itertools,iproduct};
use std::cmp::max;

type Coord = (i32,i32,i32);

fn parse_input() -> Vec<Vec<(i32,i32,i32)>> {
    let lines : Vec<String> = std::io::stdin().lock().lines().flatten().collect();
    let mut ret = Vec::new();

    let mut line_idx = 1;
    while line_idx < lines.len() {
        let mut l = Vec::new();
        while let Some((x,y,z)) = scanf!(lines[line_idx], "{},{},{}",i32,i32,i32) {
            l.push(( x,y,z ));
            line_idx+=1;
            if line_idx >= lines.len() {
                break;
            }
        }
        ret.push(l);
        line_idx += 2
    }

    ret
}

fn rotatex((x,y,z): Coord) -> Coord {
    (x,-z,y)
}

fn rotatey((x,y,z): Coord) -> Coord {
    (-z,y,x)
}

fn rotatez((x,y,z): Coord) -> Coord {
    (x,-z,y)
}

fn rotate(c: Coord, rot_id: i32) -> Coord {
    let nx = rot_id / 16;
    let ny = (rot_id / 4) % 4;
    let nz = rot_id % 4;
    let mut ret = c;
    for _ in 0..nx {
        ret = rotatex(ret)
    }
    for _ in 0..ny {
        ret = rotatey(ret);
    }
    for _ in 0..nz {
        ret = rotatez(ret);
    }
    ret
}

fn coord_min((lx,ly,lz) : Coord, (rx,ry,rz) : Coord) -> Coord {
    (lx-rx,ly-ry,lz-rz)
}

fn coord_add((lx,ly,lz) : Coord, (rx,ry,rz) : Coord) -> Coord {
    (lx+rx,ly+ry,lz+rz)
}


fn main() {
    let mut sensors = parse_input();

    // Consider all pairs of sensors
    let mut work: VecDeque<usize> = VecDeque::new();
    work.push_back(0);

    let mut at_origin: HashSet<usize> = HashSet::new();
    at_origin.insert(0);

    let mut positions: Vec<Coord> = Vec::new();

    while let Some(me_i) = work.pop_front() {
        for other_i in 0..sensors.len() {
            if other_i == me_i || at_origin.contains(&other_i) {
                continue;
            }

            let me = &sensors.clone()[me_i];
            let other = &mut sensors[other_i];

            for rot_id in 0..64 {
                let mut count: HashMap<Coord,i32> = HashMap::new();
                // Consider each pair of beacons
                for (me, other) in iproduct!(me.iter(), other.iter()) {
                    let offset = coord_min(*me, rotate(*other, rot_id));
                    *count.entry(offset).or_insert(0) += 1;
                }

                let (offset, max_overlap) = count.iter().max_by(|(_,l),(_,r)| l.cmp(r)).unwrap();

                if *max_overlap >= 12 {
                    println!("found {} overlapping beacons between {} and {} with offset {:?}, and rotation {}", max_overlap,me_i, other_i, offset, rot_id);
                    at_origin.insert(other_i);
                    work.push_back(other_i);
                    positions.push(*offset);
                    for p in other.iter_mut() {
                        *p = rotate(*p, rot_id);
                        *p = coord_add(*p, *offset);
                    }
                    break;
                }
            }
        }
    }

    let mut max_dis = 0;
    for (i, j) in positions.iter().tuple_combinations() {
        let (ix,iy,iz) = i;
        let (jx,jy,jz) = j;
        let dis = (ix-jx).abs() + (iy-jy).abs() + (iz-jz).abs();
        if dis > max_dis {
            println!("winner {:?} and {:?} = {}  ({} + {} + {})", i, j, dis, (ix-jx).abs(), (iy-jy).abs(), (iz-jz).abs());
            max_dis = dis;
        }
    }

    let all_beacons : HashSet<Coord> = sensors.to_owned().into_iter().flatten().collect();
    println!("total beacons: {}", all_beacons.len());
    println!("max dis: {}", max_dis);
}
