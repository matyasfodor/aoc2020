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
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Ship {
    position: Point,
    direction: Direction,
}

struct InputInstruction {
    direction: char,
    num: isize,
}

struct ShipWithWaypoint {
    position: Point,
    waypoint_position: Point,
}

fn translate(pos: &Point, direction: char, value: isize) -> Point {
    match direction {
        'N' => Point {
            x: pos.x,
            y: pos.y + value,
        },
        'S' => Point {
            x: pos.x,
            y: pos.y - value,
        },
        'E' => Point {
            x: pos.x + value,
            y: pos.y,
        },
        'W' => Point {
            x: pos.x - value,
            y: pos.y,
        },
        _ => {
            panic!("This should not happen");
        }
    }
}

fn first(instructions: &Vec<InputInstruction>) {
    let mut direction_to_string = HashMap::new();

    direction_to_string.insert(Direction::North, 'N');
    direction_to_string.insert(Direction::South, 'S');
    direction_to_string.insert(Direction::East, 'E');
    direction_to_string.insert(Direction::West, 'W');

    let end_point = instructions.iter().fold(
        Ship {
            position: Point { x: 0, y: 0 },
            direction: Direction::East,
        },
        |current, InputInstruction { direction, num }| match *direction {
            'N' | 'S' | 'W' | 'E' => Ship {
                position: translate(&current.position, *direction, *num),
                ..current
            },
            'L' | 'R' => {
                let repeat = num / 90;
                let mut new_direction = current.direction;
                for _ in 0..repeat {
                    new_direction = turn(new_direction, *direction == 'L');
                }
                Ship {
                    direction: new_direction,
                    ..current
                }
            }
            'F' => Ship {
                position: translate(
                    &current.position,
                    direction_to_string[&current.direction],
                    *num,
                ),
                ..current
            },
            _ => {
                panic!("This should not happen! Head is {}", *direction)
            }
        },
    );

    let manhattan_distance = end_point.position.x.abs() + end_point.position.y.abs();
    println!(
        "Manhattan distance from the source is {}",
        manhattan_distance
    );
}

fn second(instructions: &Vec<InputInstruction>) {
    let mut direction_to_string = HashMap::new();

    direction_to_string.insert(Direction::North, 'N');
    direction_to_string.insert(Direction::South, 'S');
    direction_to_string.insert(Direction::East, 'E');
    direction_to_string.insert(Direction::West, 'W');

    let end_point = instructions.iter().fold(
        ShipWithWaypoint {
            position: Point { x: 0, y: 0 },
            waypoint_position: Point { x: 10, y: 1 },
        },
        |current, InputInstruction { direction, num }| match *direction {
            'N' | 'S' | 'W' | 'E' => ShipWithWaypoint {
                waypoint_position: translate(&current.waypoint_position, *direction, *num),
                ..current
            },
            'L' | 'R' => {
                let repeat = num / 90;
                let mut waypoint_position = current.waypoint_position;
                let modifier = if *direction == 'L' { -1 } else { 1 };

                for _ in 0..repeat {
                    waypoint_position = Point {
                        x: modifier * waypoint_position.y,
                        y: modifier * -waypoint_position.x,
                    }
                }

                ShipWithWaypoint {
                    waypoint_position,
                    ..current
                }
            }
            'F' => ShipWithWaypoint {
                position: Point {
                    x: current.position.x + current.waypoint_position.x * num,
                    y: current.position.y + current.waypoint_position.y * num,
                },
                ..current
            },
            _ => {
                panic!("This should not happen! Head is {}", *direction)
            }
        },
    );

    let manhattan_distance = end_point.position.x.abs() + end_point.position.y.abs();
    println!(
        "Manhattan distance from the source is {}",
        manhattan_distance
    );
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

    let is_second = matches.is_present("second");

    let file = File::open(path).expect("File not found");

    let line_matcher = Regex::new(r"^(?P<letter>[NSEWLRF])(?P<number>\d+)$").unwrap();

    let instructions: Vec<InputInstruction> = io::BufReader::new(file)
        .lines()
        .map(|line| {
            let unwrapped = line.unwrap();
            let caps = line_matcher.captures(&unwrapped).unwrap();

            let head = caps.name("letter").map_or("", |m| m.as_str());
            let num = caps
                .name("number")
                .map_or(0, |m| m.as_str().parse::<isize>().unwrap());

            InputInstruction {
                direction: head.chars().nth(0).unwrap(),
                num,
            }
        })
        .collect();
    if !is_second {
        first(&instructions);
    } else {
        second(&instructions);
    }
}
