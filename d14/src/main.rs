use clap::{App, Arg};
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Instruction {
    MemoryWrite(usize, usize),
    Mask(String),
}

// Based on https://stackoverflow.com/a/47990/2419215
fn flip_bit(number: usize, pos: usize, value: usize) -> usize {
    let mut ret = number;
    match value {
        0 => {
            // Clearing the bit
            ret &= !(1 << pos);
        }
        1 => {
            // Setting the bit
            ret |= 1 << pos;
        }
        _ => {
            panic!("This should not happen, received {}", value);
        }
    };
    ret
}

fn first(instructions: &Vec<Instruction>) {
    // println!("Instructions: {:?}", *instructions);
    // println!("This is first");
    // println!("Binary 1: {:b}", 1 << 4)
    let mut memory = HashMap::new();
    // let mut mask: Instruction = instructions[0];
    let mut current_mask: Option<String> = None;
    instructions
        .iter()
        .for_each(|instruction| match instruction {
            Instruction::Mask(mask) => {
                current_mask = Some(mask.to_string());
            }
            Instruction::MemoryWrite(address, original_value) => {
                let mut value = *original_value;
                let unwrapped_mask = current_mask.as_ref().unwrap();
                unwrapped_mask
                    .chars()
                    .rev()
                    .enumerate()
                    .for_each(|(index, charvalue)| {
                        if charvalue == 'X' {
                            return;
                        }
                        let numeric_value = charvalue.to_string().parse::<usize>().unwrap();
                        value = flip_bit(value, index, numeric_value);
                    });

                memory.insert(address, value);
            }
        });
    println!("Memory sum: {}", memory.values().sum::<usize>());
}

fn second(_instructions: &Vec<Instruction>) {
    println!("This is second");
}

fn main() {
    let matches = App::new("AOC solution 12")
        .arg(Arg::with_name("test").short("t").long("test"))
        .arg(Arg::with_name("second").short("s").long("second"))
        .get_matches();

    let path = if matches.is_present("test") {
        "test.txt"
    } else {
        "input.txt"
    };

    let is_second = matches.is_present("second");

    let file = File::open(path).expect("File not found");

    let line_matcher = Regex::new(r"^mem\[(?P<memaddress>\d+)\] $").unwrap();

    let instructions: Vec<Instruction> = io::BufReader::new(file)
        .lines()
        .map(|line| {
            let unwrapped = line.unwrap();
            let split_vals: Vec<&str> = unwrapped.split("=").collect();
            if split_vals[0] == "mask " {
                Instruction::Mask(split_vals[1].trim().to_string())
            } else {
                let caps = line_matcher.captures(&split_vals[0]).unwrap();
                let memaddress = caps
                    .name("memaddress")
                    .map_or(0, |m| m.as_str().parse::<usize>().unwrap());
                let value = split_vals[1].trim().parse::<usize>().unwrap();
                Instruction::MemoryWrite(memaddress, value)
            }
        })
        .collect();

    if !is_second {
        first(&instructions);
    } else {
        second(&instructions);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_convert_to_binary_1() {
        assert_eq!(super::convert_to_binary(1), "1");
    }

    #[test]
    fn test_convert_to_binary_2() {
        assert_eq!(super::convert_to_binary(2), "10");
    }

    #[test]
    fn test_convert_to_binary_3() {
        assert_eq!(super::convert_to_binary(382), "101111110");
    }

    #[test]
    fn test_flip_bit_0() {
        assert_eq!(super::flip_bit(1, 0, 0), 0);
    }

    #[test]
    fn test_flip_bit_1() {
        assert_eq!(super::flip_bit(11, 1, 0), 9);
    }

    #[test]
    fn test_flip_bit_2() {
        assert_eq!(super::flip_bit(9, 6, 1), 73);
    }
}
