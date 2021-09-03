use clap::{App, Arg};
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct BusInfo<T> {
    offset: T,
    id: T,
}

fn first(eta: isize, bus_info: &Vec<BusInfo<isize>>) {
    println!("Parsed ETS {}, bus_ids {:?}", eta, bus_info);
    let mut min_minutes_to_wait = std::isize::MAX;
    let mut min_bus_id = -1;
    bus_info.iter().for_each(|BusInfo { id, .. }| {
        let mins_to_wait = id - (eta % id);
        if mins_to_wait < min_minutes_to_wait {
            min_bus_id = *id;
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

fn second(_bus_info: &Vec<BusInfo<isize>>) {
    println!("Second");
}

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
    let bus_info = second_line
        .split(",")
        .enumerate()
        .fold(vec![], |mut acc, (index, value)| {
            if value != "x" {
                acc.push(BusInfo::<isize> {
                    offset: index as isize,
                    id: value.parse().unwrap(),
                })
            }
            acc
        });

    if !is_second {
        first(eta, &bus_info);
    } else {
        second(&bus_info);
    }
}
