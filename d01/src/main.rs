use clap::{App, Arg};
use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

// Taken from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
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
    let num_previous_elements = if matches.is_present("second") { 2 } else { 1 };
    let target = 2020;
    let mut vec: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(line_content) = line {
                let price = line_content
                    .trim()
                    .parse::<i32>()
                    .expect("expected a number");
                // If the 's' flag is not passed, it uses combinations of 1
                // (which is the same as a plain iterator)
                // If the 's'  flag is passed, it takes combinations of 2 from the vector
                for prev_price in vec.iter().combinations(num_previous_elements) {
                    // combinations return an array of reference objects, they need to be de-referenced before summed up
                    if prev_price.iter().map(|x| (*x)).sum::<i32>() == target - price {
                        println!(
                            "Found: {} {:?}, their product is {}",
                            price,
                            prev_price,
                            price * prev_price.iter().map(|x| (*x)).product::<i32>()
                        );
                    }
                }
                vec.push(price);
            }
        }
    }
}
