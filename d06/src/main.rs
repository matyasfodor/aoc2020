use clap::{App, Arg};
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::FromIterator;

struct ReduceState {
    chars: Option<HashSet<char>>,
    count: usize,
}

fn main() {
    let matches = App::new("AOC solution 6")
        .arg(Arg::with_name("test").short("t").long("test"))
        .arg(Arg::with_name("second").short("s").long("second"))
        .get_matches();

    let path = if matches.is_present("test") {
        "test.txt"
    } else {
        "input.txt"
    };
    let file = File::open(path).expect("File not found");
    let res = io::BufReader::new(file).lines().fold(
        ReduceState {
            chars: None,
            count: 0,
        },
        |acc, line| {
            let unwrapped_line = line.unwrap();
            match (unwrapped_line.as_str(), acc.chars) {
                ("", Some(chars)) => ReduceState {
                    chars: None,
                    count: acc.count + chars.iter().count(),
                },
                (line_contents, None) => ReduceState {
                    chars: Some(HashSet::from_iter(line_contents.chars())),
                    count: acc.count,
                },
                (line_contents, Some(chars)) => {
                    let new_hash: HashSet<char> = HashSet::from_iter(line_contents.chars());
                    let new_acc = if matches.is_present("second") {
                        HashSet::from_iter(chars.intersection(&new_hash).map(|x| x.to_owned()))
                    } else {
                        HashSet::from_iter(chars.union(&new_hash).map(|x| x.to_owned()))
                    };
                    ReduceState {
                        chars: Some(new_acc),
                        count: acc.count,
                    }
                }
            }
        },
    );
    let custom_item_count = res.count + res.chars.unwrap().iter().count();

    println!("Found {} items", custom_item_count)
}
