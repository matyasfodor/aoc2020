use clap::{App, Arg};
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

#[derive(Debug)]
enum EvaluationResult {
    LoopDetected { acc: isize, step: usize },
    Finished { acc: isize, step: usize },
    Error,
}

fn parse_line(line: &str) -> Instruction {
    let value = line[4..].parse().expect("Could not parse instuction value");
    match &line[..3] {
        "nop" => Instruction::Nop(value),
        "acc" => Instruction::Acc(value),
        "jmp" => Instruction::Jmp(value),
        _ => panic!("Could not parse line"),
    }
}

fn first(instructions: &Vec<Instruction>, replace_index: Option<usize>) -> EvaluationResult {
    let mut index = 0;
    let mut visited_inidices: HashSet<usize> = HashSet::new();
    let mut count = 0;
    let mut step = 0;
    let result = loop {
        if index > instructions.len() {
            break EvaluationResult::Error;
        } else if index == instructions.len() {
            break EvaluationResult::Finished { acc: count, step };
        }
        let mut instruction = *instructions.get(index).expect("Index out of bounds");
        if let Some(actual_repalce_index) = replace_index {
            if actual_repalce_index == step {
                let new_instruction = match instruction {
                    Instruction::Nop(number) => Instruction::Jmp(number),
                    Instruction::Jmp(number) => Instruction::Nop(number),
                    // stays the same
                    Instruction::Acc(number) => Instruction::Acc(number),
                };
                instruction = new_instruction;
            }
        }
        match instruction {
            Instruction::Nop(_) => {
                let next_index = index + 1;
                if visited_inidices.contains(&next_index) {
                    break EvaluationResult::LoopDetected { acc: count, step };
                } else {
                    visited_inidices.insert(next_index);
                }
                index = next_index;
            }
            Instruction::Acc(acc_count) => {
                let next_index = index + 1;
                if visited_inidices.contains(&next_index) {
                    break EvaluationResult::LoopDetected { acc: count, step };
                } else {
                    visited_inidices.insert(next_index);
                }
                index = next_index;
                count += acc_count;
            }
            Instruction::Jmp(jmp_count) => {
                let next_index = ((index as isize) + jmp_count) as usize;
                if visited_inidices.contains(&next_index) {
                    break EvaluationResult::LoopDetected { acc: count, step };
                } else {
                    visited_inidices.insert(next_index);
                }
                index = next_index;
            }
        }
        step += 1;
    };
    result
}

fn second(instructions: &Vec<Instruction>) -> isize {
    let max_steps = match first(&instructions, None) {
        EvaluationResult::LoopDetected { acc: _, step } => step,
        smth => panic!("Did not converge {:?}", smth),
    };
    let mut res: Option<isize> = None;
    for step in (0..max_steps).rev() {
        let loop_result = match first(&instructions, Some(step as usize)) {
            EvaluationResult::Finished { acc, step: _ } => Some(acc),
            _ => None,
        };
        if let Some(_) = loop_result {
            res = loop_result;
            break;
        }
    }
    res.expect("Loop did not find element to change")
}

fn main() {
    let matches = App::new("AOC solution 8")
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
        let res = second(&instructions);
        println!("Finished with {}", res);
    } else {
        match first(&instructions, None) {
            EvaluationResult::LoopDetected {
                acc: result,
                step: _,
            } => println!("Loop detected, acc: {}", result),
            smth => panic!("Did not converge {:?}", smth),
        }
    }
}
