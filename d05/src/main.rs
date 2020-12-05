use clap::{App, Arg};
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

fn row_reduce(prev: usize, c: char) -> usize {
    prev * 2 + if c == 'F' { 0 } else { 1 }
}

fn col_reduce(prev: usize, c: char) -> usize {
    prev * 2 + if c == 'L' { 0 } else { 1 }
}

fn boarding_pass_to_seat_id(boarding_pass: String) -> usize {
    let row_chars = &boarding_pass[..7];
    let column_chars = &boarding_pass[7..];
    let row_number = row_chars.chars().fold(0, row_reduce);
    let col_number = column_chars.chars().fold(0, col_reduce);

    let res = row_number * 8 + col_number;
    res
}

fn main() {
    let matches = App::new("AOC solution 3")
        .arg(Arg::with_name("test").short("t").long("test"))
        .arg(Arg::with_name("second").short("s").long("second"))
        .get_matches();

    let path = if matches.is_present("test") {
        "test.txt"
    } else {
        "input.txt"
    };

    if let Ok(lines) = read_lines(path) {
        if let Some(max_id) = lines
            .map(|line_result| match line_result {
                Ok(line) => boarding_pass_to_seat_id(line),
                _ => 0,
            })
            .max()
        {
            println!("Highest seta ID {}", max_id);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_validate_hgt() {
        assert_eq!("FFFFFFF".chars().fold(0, super::row_reduce), 0);
        assert_eq!("BBBBBBB".chars().fold(0, super::row_reduce), 127);
        assert_eq!("FBFBBFF".chars().fold(0, super::row_reduce), 44);
    }
}
