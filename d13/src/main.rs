use clap::{App, Arg};
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct BusInfo<T> {
    offset: T,
    id: T,
}

fn first(eta: isize, bus_info: &Vec<BusInfo<isize>>) {
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

// Credits go to https://dev.to/benwtrent/comment/1962m
fn second(bus_info: &Vec<BusInfo<isize>>) {
    let mut bus_copy = bus_info.to_vec();
    bus_copy.sort_by_key(|a| a.offset);

    let head = bus_copy.first().unwrap();

    let mut increment = head.id;
    let mut timestamp = 0;
    for BusInfo{offset, id } in bus_copy[1..].iter() {
        println!("Finding timestamp for {} with offset {}. timestamp is {}", id, offset, timestamp);
        while (timestamp + offset) % id != 0 {
            timestamp += increment;
        }
        increment *= id
    }
    println!("Tiemstamp {}", timestamp);
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
