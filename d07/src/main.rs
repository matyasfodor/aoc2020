use clap::{App, Arg};
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::FromIterator;

#[derive(PartialEq, Debug)]
struct ParsedInfo {
    name: String,
    children: Vec<CountedEntry>,
}

#[derive(Debug, Hash, Eq)]
struct CountedEntry {
    name: String,
    count: usize,
}

impl std::cmp::PartialEq for CountedEntry {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

fn extract_info(line: &str) -> ParsedInfo {
    let main_re = Regex::new(r"(?P<name>[a-z]* [a-z]*) bags contain (?P<rest>.*)\.").unwrap();
    let capture = main_re
        .captures_iter(line)
        .next()
        .expect("Expected to match");
    let children = if &capture["rest"] == "no other bags" {
        Vec::new()
    } else {
        let children_re =
            Regex::new(r"(?P<count>\d+) (?P<name>[a-z]+ [a-z]+) bag(s)?(, )?").unwrap();

        children_re
            .captures_iter(&capture["rest"])
            .map(|x| CountedEntry {
                name: x["name"].to_string(),
                count: x["count"].parse().unwrap(),
            })
            .collect()
    };
    ParsedInfo {
        name: (&capture["name"]).to_string(),
        children,
    }
}

fn reducer(
    mut acc: HashMap<String, Vec<CountedEntry>>,
    line: String,
) -> HashMap<String, Vec<CountedEntry>> {
    let ParsedInfo { name, children } = extract_info(line.as_str());
    for child in children.iter() {
        acc.entry(child.name.to_string())
            .or_insert(vec![])
            .push(CountedEntry {
                name: name.to_owned(),
                count: child.count,
            });
    }
    acc
}

fn child_reducer(
    mut acc: HashMap<String, Vec<CountedEntry>>,
    line: String,
) -> HashMap<String, Vec<CountedEntry>> {
    let ParsedInfo { name, children } = extract_info(line.as_str());
    acc.insert(name, children);
    acc
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
    let acc: HashMap<String, Vec<CountedEntry>> = HashMap::new();
    let res = io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .fold(acc, reducer);

    let mut to_visit: HashSet<String> =
        HashSet::from_iter(res["shiny gold"].iter().map(|x| x.name.to_owned()));
    let mut visited: HashSet<String> = HashSet::new();

    while to_visit.len() > 0 {
        let elt = to_visit.iter().next().cloned().unwrap();
        let value = to_visit.take(&elt).unwrap();

        if let Some(entry) = res.get(&value) {
            for to_visit_element in entry.iter() {
                to_visit.insert(to_visit_element.name.to_string());
            }
        }
        visited.insert(value);
    }
    println!("Parents {}", visited.len());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_extract_info_simple() {
        assert_eq!(
            super::extract_info("dotted black bags contain no other bags."),
            super::ParsedInfo {
                name: "dotted black".to_owned(),
                children: Vec::new(),
            }
        );
    }

    #[test]
    fn test_extract_info_complex() {
        assert_eq!(
            super::extract_info(
                "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags."
            ),
            super::ParsedInfo {
                name: "vibrant plum".to_owned(),
                children: vec![
                    super::CountedEntry {
                        name: "faded blue".to_string(),
                        count: 5,
                    },
                    super::CountedEntry {
                        name: "dotted black".to_string(),
                        count: 6,
                    }
                ],
            }
        )
    }
}
