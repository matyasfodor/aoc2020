use clap::{App, Arg};
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
        .get_matches();

    let path = if matches.is_present("test") {
        "test.txt"
    } else {
        "input.txt"
    };
    let target = 2020;
    let mut vec: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(line_content) = line {
                let price = line_content
                    .trim()
                    .parse::<i32>()
                    .expect("expected a number");
                // println!("With text:\n{} {}", price, cnt);
                for prev_price in vec.iter() {
                    if price + prev_price == target {
                        println!(
                            "Found: {} {}, their product is {}",
                            price,
                            prev_price,
                            price * prev_price
                        );
                    }
                }
                vec.push(price);
            }
        }
    }
}
