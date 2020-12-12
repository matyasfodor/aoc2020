use clap::{App, Arg};
use itertools::Itertools;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let matches = App::new("AOC solution 9")
        .arg(Arg::with_name("test").short("t").long("test"))
        .get_matches();

    let preamble_length = if matches.is_present("test") { 5 } else { 25 };

    let path = if matches.is_present("test") {
        "test.txt"
    } else {
        "input.txt"
    };
    let file = File::open(path).expect("File not found");
    let reader = io::BufReader::new(file).lines();
    let mut generator = reader.map(|x| x.unwrap().parse::<usize>().unwrap());
    let mut preamble: Vec<usize> = vec![];

    for _ in 0..preamble_length {
        preamble.push(generator.next().unwrap());
    }

    let mut originals: VecDeque<usize> = preamble.iter().map(|x| x.to_owned()).collect();

    let mut found_elem: Option<usize> = None;
    for elem in generator {
        let found = originals
            .iter()
            .combinations(2)
            .map(|x| x[0] + x[1])
            .find(|x| elem == *x);
        if let None = found {
            found_elem = Some(elem);
            break;
        }
        originals.pop_front();
        originals.push_back(elem);
    }
    let first_result = found_elem.unwrap();
    println!("First {}", first_result);

    {
        let file = File::open(path).expect("File not found");
        let reader = io::BufReader::new(file).lines();
        let generator = reader.map(|x| x.unwrap().parse::<usize>().unwrap());

        let mut numbers: VecDeque<usize> = VecDeque::new();
        let mut sum = 0;
        let mut result: Option<usize> = None;

        for elem in generator {
            while sum + elem > first_result {
                let removed_elem = numbers.pop_front().expect("Queue shouldn't be empty");
                sum -= removed_elem;
            }
            if sum + elem == first_result {
                numbers.push_back(elem);
                let min = numbers.iter().min().unwrap();
                let max = numbers.iter().max().unwrap();
                result = Some(min + max);
                break;
            }
            numbers.push_back(elem);
            sum += elem;
        }
        println!("Second: {}", result.unwrap());
    }
}
