use std::io::BufRead;

struct Image {
    w: usize,
    h: usize,
    data: Vec<u8>,
}

fn main() {
    let lines: Vec<String> = std::io::stdin().lock().lines().flatten().collect();
    let program: Vec<u8> = lines[0].bytes().map(|b| pxl(b)).collect();

    let small_image_data : Vec<Vec<u8>> = lines.iter().skip(2).map(|s| s.bytes().map(|b| pxl(b)).collect()).collect();

    let sw = small_image_data[0].len();
    let sh = small_image_data.len();
    println!("orignal image: {}x{}", sw, sh);

    let w = 520;
    let h = 520;
    let mut image = Image {
        w,h, data: vec![0;w*h],
    };

    let sx = w/2-sw/2;
    let sy = h/2-sh/2;
    for y in 0..sw {
        for x in 0..sh {
            image.data[image.w * (y+sy) + (x+sx)] = small_image_data[y][x];
        }
    }


    for _ in 0..50 {
        image = iterate(&program, &image);
    }
    
    let sum = cnt_image(&image);
    print_img(&image);
    println!("sum {}", sum);
}

fn pxl(x: u8) -> u8 {
    match x {
        b'.' => 0,
        b'#' => 1,
        _ => panic!("{} is in there", x),
    }
}

fn iterate(program: &Vec<u8>, i: &Image) -> Image {
    let mut ret = Image {
        w: i.w, h: i.h, data: vec![0; i.w*i.h],
    };

    for y in 1..i.h-1 {
        for x in 1..i.w-1 {
            let idx = i.w * y + x;
            ret.data[idx] = kernel(program, i, x, y);
        }
    }

    ret
}

fn kernel(program: &Vec<u8>, i: &Image, x: usize, y: usize) -> u8 {
    program[neighbour_number(i,x,y) as usize]
}

fn neighbour_number(i: &Image, x: usize, y: usize) -> i32 {
    let mut ret = 0;
    for y in y-1..y+2 {
        for x in x-1..x+2 {
            ret = (ret << 1) | (i.data[i.w*y+x] as i32);
        }
    }

    ret
}

fn cnt_image(i: &Image) -> i32 {
    let mut ret = 0;
    for y in 50..i.w-50 {
        for x in 50..i.h-50 {
            ret += i.data[i.w*y+x] as i32;
        }
    }
    ret
}

fn rev_pxl(x: u8) -> char {
    match x {
        0 => '.',
        1 => '#',
        _ => panic!(),
    }
}

fn print_img(i: &Image) {
    for y in 0..i.w {
        for x in 0..i.h {
            print!("{}", rev_pxl(i.data[i.w * y + x]));
        }
        println!();
    }
}

