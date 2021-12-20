use bitvec::order::{Lsb0, Msb0};
use bitvec::slice::Iter;
use bitvec::vec::BitVec;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Peekable;

fn main() {
    let path = "INPUT16.txt";
    let input = File::open(path).expect("file not found!");
    let buffered = BufReader::new(input);
    let input = buffered.lines().last().unwrap().unwrap();
    let mut bits: BitVec<Msb0, u8> = BitVec::from_vec(hex::decode(input).expect("decoding failed"));
    let mut parserInput = Vec::new();
    for x in bits {
        parserInput.push(x);
    }
    let a = parsePacket(&parserInput[..]).0;
    println!("{:?}", a);
}
fn parsePacket(mut input: &[bool]) -> (u64, &[bool]) {
    let mut sum = 0;
    let mut version: u8 = 0;
    for i in &input[0..3] {
        version = version << 1;
        version |= *i as u8;
    }
    let mut operator = 0;
    for i in &input[3..6] {
        operator = operator << 1;
        operator |= *i as u8;
    }
    let mut value = 0;
    let mut children: Vec<u64> = Vec::new();
    let mut i = 6;
    match operator {
        4 => {
            while input[i] {
                for v in &input[i + 1..i + 5] {
                    value = value << 1;
                    value |= *v as u64;
                }
                i += 5;
            }
            for v in &input[i + 1..i + 5] {
                value = value << 1;
                value |= *v as u64;
            }
            i += 5;
            input = &input[i..];
            (value, input)
        }

        _ => {
            if input[i] {
                let mut numberOfPackets = 0;
                for v in &input[i + 1..i + 12] {
                    numberOfPackets = numberOfPackets << 1;
                    numberOfPackets |= *v as u32;
                }
                input = &input[i + 12..];
                while children.len() < numberOfPackets as usize {
                    let res = parsePacket(input);
                    children.push(res.0);
                    input = res.1;
                }
            } else {
                let mut length = 0;
                for v in &input[i + 1..i + 16] {
                    length = length << 1;
                    length |= *v as usize;
                }
                i += 16;
                let mut temp_input = &input[i..i + length];
                input = &input[i + length..];
                println!("{:?}", temp_input);
                while temp_input.len() > 0 {
                    let res = parsePacket(temp_input);
                    children.push(res.0);
                    temp_input = res.1;
                }
            }
            match operator {
                0 => (
                    children.iter().fold(0, |sum, value| sum + value) as u64,
                    input,
                ),
                1 => (
                    children.iter().fold(1, |product, value| product * value) as u64,
                    input,
                ),
                2 => (*children.iter().min().unwrap(), input),
                3 => (*children.iter().max().unwrap(), input),
                5 => ((children[0] > children[1]) as u64, input),
                6 => ((children[0] < children[1]) as u64, input),
                7 => ((children[0] == children[1]) as u64, input),
                _ => (0, input),
            }
        }
    }
}
