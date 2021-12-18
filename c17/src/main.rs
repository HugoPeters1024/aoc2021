use std::io::{BufRead};
use sscanf::scanf;
use itertools::iproduct;
use std::cmp::max;

#[derive(Debug)]
struct Area {
    xmin : i32,
    xmax : i32,
    ymin : i32,
    ymax : i32,
}

fn main() {
    let line = std::io::stdin().lock().lines().flatten().nth(0).unwrap();
    let (xmin, xmax, ymin, ymax) = scanf!(line, "target area: x={}..{}, y={}..{}", i32, i32, i32, i32).unwrap();
    let area = Area { xmin, xmax, ymin, ymax };
    println!("{:?}", area);

    let mut gmaxy = -10000;
    let mut valid_count = 0;
    for (ivx, ivy) in iproduct!(0..400, -150..550) {
        let mut vx = ivx;
        let mut vy = ivy;
        let mut x = 0;
        let mut y = 0;
        let mut maxy = 0;
        let mut valid = false;
        for i in 0..500 {
            x += vx;
            y += vy;
            maxy = max(maxy, y);

            if in_area(&area, x,y) {
                valid = true;
            }

            vx = max(0, vx-1);
            vy = vy - 1;
        }
        if valid {
            gmaxy = max(gmaxy, maxy);
            valid_count+=1;
        }
    }

    println!("max y: {}", gmaxy);
    println!("total_valid: {}", valid_count);
}

fn in_area(area: &Area, x: i32, y: i32) -> bool {
    x >= area.xmin && x <= area.xmax && y >= area.ymin && y <= area.ymax
}
