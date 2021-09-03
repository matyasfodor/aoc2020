use clap::{App, Arg};
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let matches = App::new("AOC solution 12")
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

    let mut input_iter = io::BufReader::new(file).lines();

    let first_line = input_iter.next().unwrap().unwrap();
    let second_line = input_iter.next().unwrap().unwrap();

    let eta = first_line.parse::<isize>().unwrap();
    let bus_ids = second_line
        .split(",")
        .filter(|substr| *substr != "x")
        .map(|substr| substr.parse::<isize>().unwrap())
        .collect::<Vec<isize>>();

    println!("Parsed ETS {}, bus_ids {:?}", eta, bus_ids);
    let mut min_minutes_to_wait = std::isize::MAX;
    let mut min_bus_id = -1;
    bus_ids.iter().for_each(|bus_id| {
        let mins_to_wait = bus_id - (eta % bus_id);
        if mins_to_wait < min_minutes_to_wait {
            min_bus_id = *bus_id;
            min_minutes_to_wait = mins_to_wait;
        }
    });

    println!(
        "You have to wait for {}, {} mins which gives {} as the answer",
        min_bus_id,
        min_minutes_to_wait,
        min_bus_id * min_minutes_to_wait
    );
}
