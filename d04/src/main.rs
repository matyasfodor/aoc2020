use clap::{App, Arg};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct PassportGenerator<'a> {
    lines: std::io::Lines<std::io::BufReader<std::fs::File>>,
    required_fields: Vec<&'a str>,
}

impl<'a> PassportGenerator<'a> {
    fn new(path: &str, required_fields: Vec<&'a str>) -> PassportGenerator<'a> {
        PassportGenerator {
            lines: read_lines(path).expect("File not found"),
            required_fields: required_fields,
        }
    }
}

impl<'a> Iterator for PassportGenerator<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        let mut counts: HashMap<&str, i32> = HashMap::new();
        for elem in self.required_fields.iter() {
            counts.insert(elem, 0);
        }

        while let Some(Ok(line)) = self.lines.next() {
            if line.len() > 0 {
                for elem in line
                    .split(" ")
                    .map(|entry| entry.split(":").next().expect("could not split by ':'"))
                {
                    *counts.entry(elem).or_insert(0) += 1;
                }
            }
        }
        Some("")
    }
}

fn main() {
    let matches = App::new("AOC solution 4")
        .arg(Arg::with_name("test").short("t").long("test"))
        .arg(Arg::with_name("second").short("s").long("second"))
        .get_matches();

    let path = if matches.is_present("test") {
        "test.txt"
    } else {
        "input.txt"
    };
    let passport_generator =
        PassportGenerator::new(path, vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]);
    println!("Hello, world!");
}
