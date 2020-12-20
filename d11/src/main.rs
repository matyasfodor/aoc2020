use clap::{App, Arg};
use ndarray::{Array, Array2, Axis};
use std::fs::File;
use std::io::{self, BufRead};

fn neighbor_counter(arr: &Array2<usize>) -> Array2<usize> {
    let shape = arr.shape();
    let mut ret = Array::zeros((shape[0], shape[1]));

    for x in 0..(shape[0] as isize) {
        for y in 0..(shape[1] as isize) {
            let mut counter = 0;
            for i in (x as isize - 1)..(x as isize + 2) {
                if i < 0 || i >= (shape[0] as isize) {
                    continue;
                }
                for j in (y - 1)..(y + 2) {
                    if (x as isize == i && y == j) || j < 0 || j >= (shape[1] as isize) {
                        continue;
                    }
                    counter += arr.get((i as usize, j as usize)).unwrap()
                }
            }
            *ret.get_mut((x as usize, y as usize)).unwrap() = counter;
        }
    }
    ret
}

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

    let mut floor: Array2<usize> = Array2::zeros((reader_vec.len(), reader_vec[0].len()));

    for (i, mut row) in floor.axis_iter_mut(Axis(0)).enumerate() {
        let new_line = Array::from(reader_vec[i].clone());
        row.assign(&new_line);
    }
    println!("Hello, world! {:?}", floor);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_fun() {
        assert_eq!(
            super::neighbor_counter(&ndarray::arr2(&[[1, 2], [3, 4]])),
            ndarray::arr2(&[[9, 8], [7, 6]])
        );
        assert_eq!(
            super::neighbor_counter(&ndarray::arr2(&[[0, 1, 0], [1, 1, 1], [0, 1, 0]])),
            ndarray::arr2(&[[3, 3, 3], [3, 4, 3], [3, 3, 3]])
        );
    }
}
