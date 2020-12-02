use clap::{App, Arg};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Taken from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct ParseResult {
    policy: (usize, usize),
    letter: char,
    password: String,
}

fn parse(line: String) -> ParseResult {
    let vec: Vec<&str> = line.split(" ").collect();
    // try remove
    let policy = vec[0];
    let letter = vec[1].chars().collect::<Vec<char>>()[0];
    let password = vec[2];

    let mut policy_vec: Vec<usize> = policy
        .split("-")
        .map(|x| x.parse::<usize>().expect("expected a number"))
        .collect();
    let policy_first = policy_vec.remove(0);
    let policy_second = policy_vec.remove(0);

    ParseResult {
        policy: (policy_first, policy_second),
        letter: letter,
        password: password.to_owned(),
    }
}

fn valid(line: String) -> bool {
    let parse_results = parse(line);

    let mut count = 0;
    for char in parse_results.password.chars() {
        if char == parse_results.letter {
            count += 1;
        }
    }
    return parse_results.policy.0 <= count && count <= parse_results.policy.1;
}

fn valid2(line: String) -> bool {
    let parse_results = parse(line);
    let password = parse_results.password;
    let first_occurrence = password
        .chars()
        .nth(parse_results.policy.0 - 1)
        .expect("Index out of range");
    let second_occurrence = password
        .chars()
        .nth(parse_results.policy.1 - 1)
        .expect("Index out of range");

    return (first_occurrence == parse_results.letter)
        ^ (second_occurrence == parse_results.letter);
}

fn main() {
    let matches = App::new("AOC solution 1")
        .arg(Arg::with_name("test").short("t").long("test"))
        .arg(Arg::with_name("second").short("s").long("second"))
        .get_matches();

    let path = if matches.is_present("test") {
        "test.txt"
    } else {
        "input.txt"
    };

    let mut count = 0;
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(line_content) = line {
                let function = if matches.is_present("second") {
                    valid2
                } else {
                    valid
                };
                if function(line_content) {
                    count += 1;
                }
            }
        }
    }

    println!("Valid lines: {}", count);
}
