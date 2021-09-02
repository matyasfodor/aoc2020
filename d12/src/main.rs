use clap::{App, Arg};
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn turn(current: Direction, left: bool) -> Direction {
    match (current, left) {
        (Direction::North, true) => Direction::West,
        (Direction::North, false) => Direction::East,
        (Direction::South, true) => Direction::East,
        (Direction::South, false) => Direction::West,
        (Direction::East, true) => Direction::North,
        (Direction::East, false) => Direction::South,
        (Direction::West, true) => Direction::South,
        (Direction::West, false) => Direction::North,
    }
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
    direction: Direction,
}

fn translate(pos: &Point<isize>, direction: &str, value: isize) -> Point<isize> {
    match direction {
        "N" => Point::<isize> {
            x: pos.x,
            y: pos.y + value,
            direction: pos.direction,
        },
        "S" => Point {
            x: pos.x,
            y: pos.y - value,
            direction: pos.direction,
        },
        "E" => Point {
            x: pos.x + value,
            y: pos.y,
            direction: pos.direction,
        },
        "W" => Point {
            x: pos.x - value,
            y: pos.y,
            direction: pos.direction,
        },
        _ => {
            panic!("This should not happen");
        }
    }
}

fn main() {
    let matches = App::new("AOC solution 11")
        .arg(Arg::with_name("test").short("t").long("test"))
        .arg(Arg::with_name("second").short("s").long("second"))
        .get_matches();

    let path = if matches.is_present("test") {
        "test.txt"
    } else {
        "input.txt"
    };

    // let second = matches.is_present("second");

    let file = File::open(path).expect("File not found");

    let mut current = Point {
        x: 0,
        y: 0,
        direction: Direction::East,
    };

    let line_matcher = Regex::new(r"^(?P<letter>[NSEWLRF])(?P<number>\d+)$").unwrap();

    let mut direction_to_string = HashMap::new();

    direction_to_string.insert(Direction::North, "N");
    direction_to_string.insert(Direction::South, "S");
    direction_to_string.insert(Direction::East, "E");
    direction_to_string.insert(Direction::West, "W");

    io::BufReader::new(file).lines().for_each(|line| {
        let unwrapped = line.unwrap();
        let caps = line_matcher.captures(&unwrapped).unwrap();

        let head = caps.name("letter").map_or("", |m| m.as_str());
        let num = caps
            .name("number")
            .map_or(0, |m| m.as_str().parse::<isize>().unwrap());

        // println!("## line {} {}", head, num);

        current = match head {
            "N" | "S" | "W" | "E" => translate(&current, head, num),
            "L" | "R" => {
                let repeat = num / 90;
                // println!("Turn {} times based on {}", repeat, num);
                let mut new_direction = current.direction;
                for _ in 0..repeat {
                    new_direction = turn(new_direction, head == "L");
                    // println!("New direction is {:?}", new_direction);
                }
                Point {
                    x: current.x,
                    y: current.y,
                    direction: new_direction,
                }
            }
            "F" => translate(&current, direction_to_string[&current.direction], num),
            _ => {
                panic!("This should not happen! Head is {}", head)
            }
        };
        // println!("Lines is {}, current is {:?}", unwrapped, current);
    });

    let manhattan_distance = current.x.abs() + current.y.abs();
    println!(
        "Manhattan distance from the source is {}, current is {:?}",
        manhattan_distance, current
    );
}
