use clap::{App, Arg};
use itertools::Itertools;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let matches = App::new("AOC solution 9")
        .arg(Arg::with_name("test").short("t").long("test"))
        .arg(Arg::with_name("second").short("s").long("second"))
        .get_matches();

    // TODO store in argument
    let preamble_length = 25;

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

    for elem in generator {
        let found = originals
            .iter()
            .combinations(2)
            .map(|x| x[0] + x[1])
            .find(|x| elem == *x);
        if let None = found {
            println!("Found {}", elem);
        }
        originals.pop_front();
        originals.push_back(elem);
    }
}
