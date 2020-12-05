use clap::{App, Arg};
use regex::Regex;
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

type ValidatorCallback = fn(value: &str) -> bool;

fn validate_byr(value: &str) -> bool {
    match value.parse::<usize>() {
        Ok(parsed_value) => 1920 <= parsed_value && parsed_value <= 2002,
        _ => false,
    }
}

fn validate_iyr(value: &str) -> bool {
    match value.parse::<usize>() {
        Ok(parsed_value) => 2010 <= parsed_value && parsed_value <= 2020,
        _ => false,
    }
}

fn validate_eyr(value: &str) -> bool {
    match value.parse::<usize>() {
        Ok(parsed_value) => 2020 <= parsed_value && parsed_value <= 2030,
        _ => false,
    }
}

fn validate_hgt(value: &str) -> bool {
    let len = value.len();
    let num = &value[..len - 2];
    let ending = &value[len - 2..];
    match num.parse::<usize>() {
        Ok(parsed_value) => match ending {
            "cm" => 150 <= parsed_value && parsed_value <= 193,
            "in" => 59 <= parsed_value && parsed_value <= 76,
            _ => false,
        },
        _ => false,
    }
}

fn validate_hcl(value: &str) -> bool {
    let re = Regex::new(r"^#([a-fA-F0-9]{6})$").expect("Failed to create");
    re.is_match(&value)
}

fn validate_ecl(value: &str) -> bool {
    let re = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").expect("Failed to create");
    re.is_match(&value)
}

fn validate_pid(value: &str) -> bool {
    let re = Regex::new(r"^(\d){9}$").expect("Failed to create");
    re.is_match(&value)
}

struct PassportGenerator {
    lines: std::io::Lines<std::io::BufReader<std::fs::File>>,
    validate: bool,
    validators: HashMap<String, ValidatorCallback>,
}

impl PassportGenerator {
    fn new(path: &str, validate: bool) -> PassportGenerator {
        let mut validators: HashMap<String, ValidatorCallback> = HashMap::new();
        validators.insert("byr".to_owned(), validate_byr);
        validators.insert("iyr".to_owned(), validate_iyr);
        validators.insert("eyr".to_owned(), validate_eyr);
        validators.insert("hgt".to_owned(), validate_hgt);
        validators.insert("hcl".to_owned(), validate_hcl);
        validators.insert("ecl".to_owned(), validate_ecl);
        validators.insert("pid".to_owned(), validate_pid);
        PassportGenerator {
            lines: read_lines(path).expect("File not found"),
            validators,
            validate,
        }
    }
}

impl<'a> Iterator for PassportGenerator {
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        let mut counts: HashMap<String, usize> = HashMap::new();
        let mut valid = false;
        while let Some(Ok(line)) = self.lines.next() {
            if line.len() > 0 {
                valid = true;
                let owned_line = line.to_owned();
                for elem in owned_line.split(" ") {
                    let mut split_elem = elem.clone().split(":");
                    let key = split_elem.next().unwrap().clone().to_string();
                    let value = split_elem.next().unwrap();
                    if let Some(validator) = self.validators.get(&key) {
                        let validation_result = validator(&value);
                        if !self.validate || validation_result {
                            *counts.entry(key.to_owned()).or_insert(0) += 1;
                        }
                    }
                }
            } else {
                break;
            }
        }
        if !valid {
            return None;
        }

        let keys: HashSet<String> = HashSet::from_iter(counts.keys().map(|x| x.to_string()));
        let expected_keys: HashSet<String> = self.validators.keys().map(|x| x.to_owned()).collect();

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
    let validate = matches.is_present("second");
    let passport_generator = PassportGenerator::new(path, validate);
    let num_pass = passport_generator.filter(|x| *x).count();
    println!("Num passing {}", num_pass);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_validate_hgt() {
        assert_eq!(super::validate_hgt("asdasd"), 4);
    }
}
