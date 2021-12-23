use std::{io::BufRead, collections::btree_set::Iter, ops::Range};
use sscanf::scanf;
use std::cmp::{min,max};
use itertools::iproduct;

type Cuboid = ((i64, i64), (i64, i64), (i64, i64));
type Step = (bool, Cuboid);

fn main() {
    let steps: Vec<Step> = std::io::stdin().lock().lines().flatten().map(|l| parse_step(&l)).collect();
    let mut universe: Vec<Step> = Vec::new();

    for (v, cube) in steps {
        let mut to_add: Vec<Step> = Vec::new();

        if v { to_add.push((v, cube)); }

        for (v_other, cube_other) in &universe {
            if let Some(c) = cuboid_overlap(cube, *cube_other) {
                if *v_other {
                    println!("overlap detected: {:?}", c);
                    to_add.push((false, c));
                } else {
                    to_add.push((true, c));
                }
            }
        }
        universe.extend(to_add);
    }


    let count: i64 = universe.iter().map(|(v, cube)| if *v { cuboid_volume(cube) } else { -cuboid_volume(cube) }).sum();
    println!("count: {}", count);
}

fn overlap_volume(((x, xe), (y, ye), (z, ze)): Cuboid, ((a, ae), (b, be), (c, ce)): Cuboid) -> i64 {
    max(min(ae, xe) - max(a,x), 0) * max(min(be, ye) - max(b, y), 0) * max(min(ce, ze) - max(c,z), 0)
}

fn cuboid_overlap(((x, xe), (y, ye), (z, ze)): Cuboid, ((a, ae), (b, be), (c, ce)): Cuboid) -> Option<Cuboid> {
    let i = max(a,x); 
    let ie = min(ae, xe);
    let j = max(b, y);
    let je = min(be, ye);
    let k = max(c,z);
    let ke = min(ce, ze);
    if ie > i && je > j && ke > k {
        Some(((i, ie), (j, je), (k, ke)))
    } else {
        None
    }
}


fn cuboid_volume(((x,xe), (y, ye), (z, ze)): &Cuboid) -> i64 {
    (xe-x)*(ye-y)*(ze-z)
}

fn parse_step(inp: &String) -> Step {
    if let Some((xs, xe, ys, ye, zs, ze)) = scanf!(inp, "on x={}..{},y={}..{},z={}..{}", i64, i64, i64, i64, i64, i64) {
        (true, ((xs, xe+1), (ys, ye+1), (zs, ze+1)))
    } else if let Some((xs, xe, ys, ye, zs, ze)) = scanf!(inp, "off x={}..{},y={}..{},z={}..{}", i64, i64, i64, i64, i64, i64) {
        (false, ((xs, xe+1), (ys, ye+1), (zs, ze+1)))
    } else {
        panic!();
    }
}
