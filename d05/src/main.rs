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

/**
 * Possible optimization: Simply map B, R -> 1; F, L -> 0
 * The produced binary stirng is the seat id
 */
fn boarding_pass_to_seat_id(boarding_pass: String) -> usize {
    let row_chars = &boarding_pass[..7];
    let column_chars = &boarding_pass[7..];
    let row_number = row_chars.chars().fold(0, row_reduce);
    let col_number = column_chars.chars().fold(0, col_reduce);

    let res = row_number * 8 + col_number;
    res
}

/**
 * Finding the missing seat id
 *
 * Finding the missing seat ID in O(N)
 * The key to find the missing seat ID with a single pass
 * is to keep track of the boundaries (min, max),
 * while xor-ing the seat ids (product).
 *
 * Finally a range from min - max xored together xored with the product
 * will result the missing seat ID.
 *
 * Pseudo solution:
 * (min..max+1).reduce(xor) xor product = missing_id
 */
struct MissingSeatReducerState {
    min: usize,
    max: usize,
    product: usize,
}

fn missing_set_reducer(
    acc: Option<MissingSeatReducerState>,
    seat_id: usize,
) -> Option<MissingSeatReducerState> {
    match acc {
        None => Some(MissingSeatReducerState {
            min: seat_id,
            max: seat_id,
            product: seat_id,
        }),
        Some(MissingSeatReducerState { min, max, product }) => Some(MissingSeatReducerState {
            min: min.min(seat_id),
            max: max.max(seat_id),
            product: product ^ seat_id,
        }),
    }
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
        let seat_ids_iter = lines.map(|line_result| match line_result {
            Ok(line) => boarding_pass_to_seat_id(line),
            _ => 0,
        });
        if matches.is_present("second") {
            if let Some(reducer_state) = seat_ids_iter.fold(None, missing_set_reducer) {
                let all = (reducer_state.min + 1..reducer_state.max + 1)
                    .fold(reducer_state.min, |a, b| a ^ b);
                let missing_seat_id = all ^ reducer_state.product;
                println!("Missing seat id {}", missing_seat_id);
            }
        } else {
            if let Some(max_id) = seat_ids_iter.max() {
                println!("Highest seta ID {}", max_id);
            }
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
