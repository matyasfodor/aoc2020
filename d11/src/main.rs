use clap::{App, Arg};
use ndarray::{stack, Array, Array2, Axis};
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

fn progress(occupancy: &Array2<usize>, floor: &Array2<usize>) -> Array2<usize> {
    let neighbors = neighbor_counter(occupancy);
    let concatted = stack![Axis(0), *occupancy, *floor, neighbors];

    let ret = concatted.map_axis(Axis(0), |x| {
        let occupancy = x[0];
        let floor = x[1];
        let neighbors = x[2];
        if floor == 1 {
            0
        } else {
            if occupancy == 1 && 4 <= neighbors {
                0
            } else if occupancy == 0 && neighbors == 0 {
                1
            } else {
                occupancy
            }
        }
    });
    ret
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
    let shape = floor.shape();
    let mut prev = Array2::zeros((shape[0], shape[1]));
    let mut counter = 0;
    let loop_counter = loop {
        let next = progress(&prev, &floor);
        if next == prev {
            break counter;
        } else {
            counter += 1;
            prev = next;
        }
    };
    println!("Stable state after {}", loop_counter);
    println!("There are  {} occupied", prev.sum());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_neighbor_counter() {
        assert_eq!(
            super::neighbor_counter(&ndarray::arr2(&[[1, 2], [3, 4]])),
            ndarray::arr2(&[[9, 8], [7, 6]])
        );
        assert_eq!(
            super::neighbor_counter(&ndarray::arr2(&[[0, 1, 0], [1, 1, 1], [0, 1, 0]])),
            ndarray::arr2(&[[3, 3, 3], [3, 4, 3], [3, 3, 3]])
        );
    }

    #[test]

    fn test_progress() {
        assert_eq!(
            super::progress(
                &ndarray::arr2(&[[0, 0, 0], [0, 0, 0], [0, 0, 0]]),
                &ndarray::Array2::zeros((3, 3))
            ),
            ndarray::Array2::ones((3, 3))
        );
        assert_eq!(
            super::progress(
                &ndarray::Array2::ones((3, 3)),
                &ndarray::Array2::zeros((3, 3))
            ),
            ndarray::arr2(&[[1, 0, 1], [0, 0, 0], [1, 0, 1]]),
        );
        assert_eq!(
            super::progress(
                &ndarray::arr2(&[[0, 0, 0], [0, 0, 0], [0, 0, 0]]),
                &ndarray::arr2(&[[1, 0, 0], [0, 1, 0], [0, 0, 1]]),
            ),
            ndarray::arr2(&[[0, 1, 1], [1, 0, 1], [1, 1, 0]]),
        );
    }
}
