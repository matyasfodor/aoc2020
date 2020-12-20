use clap::{App, Arg};
use ndarray::{array, concatenate, Array, Array2, Axis};
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let matches = App::new("AOC solution 8")
        .arg(Arg::with_name("test").short("t").long("test"))
        .arg(Arg::with_name("second").short("s").long("second"))
        .get_matches();

    let path = if matches.is_present("test") {
        "test.txt"
    } else {
        "input.txt"
    };

    let file = File::open(path).expect("File not found");
    let mut reader_vec: Vec<Vec<usize>> = vec![];

    io::BufReader::new(file).lines().for_each(|line| {
        let line_contents = line.unwrap();
        let line_vec = line_contents
            .chars()
            .map(|c| match c {
                'L' => 0,
                '.' => 1,
                _ => panic!("Nok"),
            })
            .collect();
        reader_vec.push(line_vec);
    });

    // let smth = Array::from(reader_vec.iter());

    let mut floor: Array2<usize> = Array2::zeros((0, reader_vec[0].len()));

    for line in reader_vec.iter() {
        let copied_line = line.clone();
        let new_line = Array::from(copied_line);
        let formatted = new_line.into_shape((1, 10)).unwrap();
        floor = concatenate!(Axis(0), floor, formatted);
    }

    // for (i, mut row) in floor.axis_iter_mut(Axis(0)).enumerate() {
    //     // Perform calculations and assign to `row`; this is a trivial example:
    //     println!("Row: {}", row);
    //     row.fill(0);
    //     // row.set(5, 15);

    //     // let new_row = Array::from(reader_vec[0].copy());
    //     row.assign(&Array::from(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]));
    // }
    // let a2 = array![[1, 2], [3, 4]];
    println!("Hello, world! {:?}", floor);
}
