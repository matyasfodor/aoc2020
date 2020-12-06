use clap::{App, Arg};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::FromIterator;
use std::path::Path;

// fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
// where
//     P: AsRef<Path>,
// {
//     let file = File::open(filename)?;
//     Ok(io::BufReader::new(file).lines())
// }

struct ReduceState {
    chars: HashSet<char>,
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
    // let acc: HashSet<char> = HashSet::new();
    let res = io::BufReader::new(file).lines().fold(
        ReduceState {
            chars: HashSet::new(),
            count: 0,
        },
        |acc, line| {
            let unwrapped_line = line.unwrap();
            match unwrapped_line.as_str() {
                "" => ReduceState {
                    chars: HashSet::new(),
                    count: acc.count + acc.chars.iter().count(),
                },
                line_contents => {
                    let new_hash: HashSet<char> = HashSet::from_iter(line_contents.chars());
                    let new_acc =
                        HashSet::from_iter(acc.chars.union(&new_hash).map(|x| x.to_owned()));
                    ReduceState {
                        chars: new_acc,
                        count: acc.count,
                    }
                }
            }
        },
    );
    let custom_item_count = res.count + res.chars.iter().count();

    println!("Found {} items", custom_item_count)
}
