use clap::{App, Arg};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::FromIterator;
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
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        let mut counts: HashMap<String, i32> = HashMap::new();
        // for elem in self.required_fields.iter() {
        //     counts.insert(elem.to_owned().to_owned(), 0);
        // }

        while let Some(Ok(line)) = self.lines.next() {
            if line.len() > 0 {
                for elem in line.split(" ").map(|entry| {
                    [
                        entry.split(":").next().expect("could not split by ':'"),
                        entry.split(":").next().expect("could not split by ':'"),
                    ]
                }) {
                    *counts.entry(elem[0].to_owned()).or_insert(0) += 1;
                }
            } else {
                break;
            }
        }
        if counts.iter().count() == 0 {
            return None;
        }

        let keys: HashSet<String> = HashSet::from_iter(counts.keys().map(|x| x.to_owned()));
        let expected_keys: HashSet<String> = self
            .required_fields
            .iter()
            .map(|x| x.to_string())
            .collect::<HashSet<_>>();
        let diff_size = expected_keys.difference(&keys).count();
        return Some(diff_size == 0);
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
    let num_pass = passport_generator.filter(|x| *x).count();
    println!("Num passing {}", num_pass);
}
