use std::io::{BufRead};
use sscanf::scanf;

/*
3 bits: packet version
3 bits: type ID
    typeID 4 is a literal value, padded with leading zeroes until multiple of 4
    each group of 4 prefixed with 1, expect the last being prefixed with 0
    (so actually groups of 5 bits)

    other types are an operator
    two modes encoded in the length type ID, which is the bit after the header

    0 => next 15 bits are a number representing total bit length of sub-packets
    1 => next 11 bits are a number representing the number of sub-packets

    Then the subpacket follows
*/

#[derive(Debug)]
enum Msg {
    Literal(i64,i64,i64),
    Operator(i64,i64,bool,Vec<Msg>),
}

fn main() {
    let lines : Vec<String> = std::io::stdin().lock().lines().flatten().collect();
    let input = hex_to_bin(lines.first().unwrap().clone());
    println!("input {}", input);
    let (msg, _) = parse_msg(input.clone());
    println!("{:?}", msg);
    println!("version sum {:?}", sum_typeid(&msg));
    println!("complex sum {:?}", sum_2(&msg));
}

fn hex_to_bin(inp: String) -> String {
    let bytes = inp.bytes().map(|v| {
        match v {
            b'0' => "0000",
            b'1' => "0001",
            b'2' => "0010",
            b'3' => "0011",
            b'4' => "0100",
            b'5' => "0101",
            b'6' => "0110",
            b'7' => "0111",
            b'8' => "1000",
            b'9' => "1001",
            b'A' => "1010",
            b'B' => "1011",
            b'C' => "1100",
            b'D' => "1101",
            b'E' => "1110",
            b'F' => "1111",
            _   => panic!(),
        }
    }).collect();
    bytes
}

fn bin_to_int(inp: String) -> i64 {
    let mut acc = 0;
    for b in inp.bytes() {
        acc = acc * 2 + (b - 0x30) as i64;
    }

    acc
}

fn parse_msg(bits: String) -> (Msg, String) {
    println!(">>>>>>>>>>>>>>>>>>>>>");
    let mut bits = bits.clone();

    let (version, rest) = bits.split_at(3);
    let version = bin_to_int(version.to_string());

    let (type_id, rest) = rest.split_at(3);
    let type_id = bin_to_int(type_id.to_string());

    bits = rest.to_string();

    println!("version {:?}", version);
    println!("type_id {:?}", type_id);

    if type_id == 4 {
        // Literal case
        let mut chunks : Vec<String> = Vec::new();
        loop {
            let wtf = bits.to_owned();
            let (chunk, rest) = wtf.split_at(5);
            let data : String = String::from_utf8(chunk.bytes().skip(1).collect()).expect("yieks");
            chunks.push(data);
            bits = rest.to_string();
            if chunk.bytes().nth(0).unwrap() == b'0' {
                break;
            }
        }

        let value = bin_to_int(chunks.iter().flat_map(|s| s.chars()).collect());
        println!("Literal value is {}", value);
        println!("<<<<<<<<<<<<<<<<<<<<");
        (Msg::Literal(version, type_id, value), bits)
    } else {

        // Operator case
        let (length_type, bits) = bits.split_at(1);
        let length_type = length_type.bytes().nth(0).unwrap() == b'1';

        if !length_type {
            // 0 case
            println!("length type is 0");
            let (total_length, bits) = bits.split_at(15);
            let total_length = bin_to_int(total_length.to_string());
            println!("Going to parse the next {} bits", total_length);
            let mut total_parsed = 0;
            let mut subs : Vec<Msg> = Vec::new();
            let mut bits = bits.to_string();
            while total_parsed < total_length {
                let old_len = bits.len();
                let (sub, rest) = parse_msg(bits.to_string());
                total_parsed += (old_len - rest.len()) as i64;
                println!("Parsed {}/{} bits", total_parsed, total_length);
                subs.push(sub);
                bits = rest.to_string();
            }

            (Msg::Operator(version, type_id, length_type, subs), bits)

        } else {
            // 1 case
            println!("length type is 1");
            let (total_subs, bits) = bits.split_at(11);
            let total_subs = bin_to_int(total_subs.to_string());
            println!("Going to parse the next {} packets", total_subs);
            
            let mut subs : Vec<Msg> = Vec::new();
            let mut bits = bits.to_string();
            while subs.len() < total_subs as usize {
                let (sub, rest) = parse_msg(bits.to_string());
                subs.push(sub);
                println!("Parsed {}/{} packets", subs.len(), total_subs);
                bits = rest.to_string();
            }
            (Msg::Operator(version, type_id, length_type, subs), bits)
        }
    }
}

fn sum_typeid(tree: &Msg) -> i64 {
    match tree {
        Msg::Literal(v,_,_) => *v,
        Msg::Operator(v,_,_,xs) => xs.iter().map(|x| sum_typeid(x)).sum::<i64>() + *v
    }
}

fn sum_2(tree: &Msg) -> i64 {
    match tree {
        Msg::Literal(_,_,v) => *v,
        Msg::Operator(_,0,_,xs) => xs.iter().map(|x| sum_2(x)).sum(),
        Msg::Operator(_,1,_,xs) => xs.iter().map(|x| sum_2(x)).product(),
        Msg::Operator(_,2,_,xs) => xs.iter().map(|x| sum_2(x)).min().unwrap(),
        Msg::Operator(_,3,_,xs) => xs.iter().map(|x| sum_2(x)).max().unwrap(),
        Msg::Operator(_,5,_,xs) => {
            let vs : Vec<i64> = xs.iter().map(|x| sum_2(x)).collect();
            if vs[0] > vs[1] {1} else {0}
        }
        Msg::Operator(_,6,_,xs) => {
            let vs : Vec<i64> = xs.iter().map(|x| sum_2(x)).collect();
            if vs[0] < vs[1] {1} else {0}
        }
        Msg::Operator(_,7,_,xs) => {
            let vs : Vec<i64> = xs.iter().map(|x| sum_2(x)).collect();
            if vs[0] == vs[1] {1} else {0}
        }

        _ => panic!(),
    }
}
