use clap::{App, Arg};
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Instruction {
    Nop,
    Acc(isize),
    Jmp(isize),
}

#[derive(Debug)]
enum EvaluationResult {
    LoopDetected { acc: isize, step: isize },
    Finished { acc: isize, step: isize },
    Error,
}

fn parse_line(line: &str) -> Instruction {
    match &line[..3] {
        "nop" => Instruction::Nop,
        "acc" => Instruction::Acc(line[4..].parse().expect("Could not parse acc")),
        "jmp" => Instruction::Jmp(line[4..].parse().expect("Could not parse jmp")),
        _ => panic!("Could not parse line"),
    }
}

fn first(instructions: &Vec<Instruction>) -> EvaluationResult {
    let mut index = 0;
    let mut visited_inidices: HashSet<usize> = HashSet::new();
    let mut count = 0;
    let result = loop {
        if index > instructions.len() {
            break EvaluationResult::Error;
        } else if index == instructions.len() {
            break EvaluationResult::Finished {
                acc: count,
                step: index as isize,
            };
        }
        let instruction = instructions.get(index).expect("Index out of bounds");
        match instruction {
            Instruction::Nop => {
                let next_index = index + 1;
                if visited_inidices.contains(&next_index) {
                    break EvaluationResult::LoopDetected {
                        acc: count,
                        step: index as isize,
                    };
                } else {
                    visited_inidices.insert(next_index);
                }
                index = next_index;
            }
            Instruction::Acc(acc_count) => {
                let next_index = index + 1;
                if visited_inidices.contains(&next_index) {
                    break EvaluationResult::LoopDetected {
                        acc: count,
                        step: index as isize,
                    };
                } else {
                    visited_inidices.insert(next_index);
                }
                index = next_index;
                count += acc_count;
            }
            Instruction::Jmp(jmp_count) => {
                let next_index = ((index as isize) + jmp_count) as usize;
                if visited_inidices.contains(&next_index) {
                    break EvaluationResult::LoopDetected {
                        acc: count,
                        step: index as isize,
                    };
                } else {
                    visited_inidices.insert(next_index);
                }
                index = next_index;
            }
        }
    };
    result
}

fn second(instructions: &Vec<Instruction>) {
    let mut index = 0;
    let mut visited_inidices: HashSet<usize> = HashSet::new();
    let mut count = 0;
    let result = loop {
        if index > instructions.len() {
            break None;
        }
        if index == instructions.len() {
            break Some(count);
        }
        let instruction = instructions.get(index).expect("Index out of bounds");
        match instruction {
            Instruction::Nop => {
                let next_index = index + 1;
                if visited_inidices.contains(&next_index) {
                    break None;
                } else {
                    visited_inidices.insert(next_index);
                }
                index = next_index;
            }
            Instruction::Acc(acc_count) => {
                let next_index = index + 1;
                if visited_inidices.contains(&next_index) {
                    break None;
                } else {
                    visited_inidices.insert(next_index);
                }
                index = next_index;
                count += acc_count;
            }
            Instruction::Jmp(jmp_count) => {
                let next_index = ((index as isize) + jmp_count) as usize;
                if visited_inidices.contains(&next_index) {
                    break None;
                } else {
                    visited_inidices.insert(next_index);
                }
                index = next_index;
            }
        }
    };
    println!("Final count {}", result.unwrap());
}

fn main() {
    let matches = App::new("AOC solution 7")
        .arg(Arg::with_name("test").short("t").long("test"))
        .arg(Arg::with_name("second").short("s").long("second"))
        .get_matches();

    let path = if matches.is_present("test") {
        "test.txt"
    } else {
        "input.txt"
    };
    let file = File::open(path).expect("File not found");
    let instructions: Vec<Instruction> = io::BufReader::new(file)
        .lines()
        .map(|x| parse_line(x.unwrap().as_str()))
        .collect();

    if matches.is_present("second") {
        second(&instructions);
    } else {
        match first(&instructions) {
            EvaluationResult::LoopDetected {
                acc: result,
                step: _,
            } => println!("Loop detected, acc: {}", result),
            smth => println!("Did not converge {:?}", smth),
        }
    }
}
