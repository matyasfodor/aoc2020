use clap::{App, Arg};
use itertools::Itertools;
use ndarray::{stack, Array, Array2, Axis};
use std::fs::File;
use std::io::{self, BufRead};

fn gen_directions_iter() -> impl Iterator<Item = (isize, isize)> + 'static {
    let ret = (-1..2)
        .cartesian_product(-1..2)
        .filter(|(left, right)| *left != 0 || *right != 0);
    ret
}

fn neighbor_counter<P>(arr: &Array2<usize>, floor: &Array2<usize>, predicate: P) -> Array2<usize>
where
    P: Fn(&Array2<usize>, &Array2<usize>, (isize, isize), (isize, isize)) -> bool,
{
    let shape = arr.shape();
    let mut ret = Array::zeros((shape[0], shape[1]));

    for x in 0..(shape[0] as isize) {
        for y in 0..(shape[1] as isize) {
            let mut counter = 0;
            for (left, right) in gen_directions_iter() {
                if predicate(arr, floor, (x, y), (left, right)) {
                    counter += 1;
                }
            }
            *ret.get_mut((x as usize, y as usize)).unwrap() = counter;
        }
    }
    ret
}

fn simple_count(
    arr: &Array2<usize>,
    _: &Array2<usize>,
    current: (isize, isize),
    direction: (isize, isize),
) -> bool {
    match arr.get((
        (current.0 + direction.0) as usize,
        (current.1 + direction.1) as usize,
    )) {
        Some(r) => *r == 1,
        None => false,
    }
}

fn second_count(
    arr: &Array2<usize>,
    floor: &Array2<usize>,
    current: (isize, isize),
    direction: (isize, isize),
) -> bool {
    let mut mut_current = (current.0 + direction.0, current.1 + direction.1);
    let mut finished = false;
    let mut has_neighbor = false;

    while !finished {
        // finished = match arr.get((mut_current.0 as usize, mut_current.1 as usize)) {
        //     Some(x) => {
        //         if *x == 1 {
        //             has_neighbor = true;
        //             true
        //         } else {
        //             false
        //         }
        //     }
        //     None => true,
        // };

        finished = match arr.get((mut_current.0 as usize, mut_current.1 as usize)) {
            Some(x) => {
                let floor_value = floor
                    .get((mut_current.0 as usize, mut_current.1 as usize))
                    .unwrap();

                // println!(
                //     "## Current {:?} Floor value {} occupancy value {}",
                //     mut_current, floor_value, x
                // );
                if *floor_value == 1 {
                    false
                } else {
                    if *x == 1 {
                        has_neighbor = true;
                        true
                    } else {
                        true
                    }
                }

                // match (floor_value, x) {
                //     (0, _) => false,
                //     (1, 1) => {
                //         has_neighbor = true;
                //         true
                //     }
                //     (1, 0) => false,
                //     _ => {
                //         panic!("Shouldn't have reach this!")
                //     }
                // }

                // if *x == 1 {
                //     has_neighbor = true;
                //     true
                // } else {
                //     false
                // }
            }
            None => true,
        };

        // let floor_value = floor.get((mut_current.0 as usize, mut_current.1 as usize));
        mut_current = (mut_current.0 + direction.0, mut_current.1 + direction.1);
    }
    has_neighbor
}

fn repr(occupancy: &Array2<usize>, floor: &Array2<usize>, disp_number: bool) {
    for ((x, y), value) in occupancy.indexed_iter() {
        if y == 0 {
            print!("\n");
        }
        let floor_val = floor.get((x, y)).unwrap();

        if *floor_val == 1 {
            print!(".")
        } else {
            if *value == 1 {
                if disp_number {
                    print!("{}", *value)
                } else {
                    print!("{}", "#")
                }
            } else {
                if disp_number {
                    print!("{}", *value)
                } else {
                    print!("{}", "L")
                }
            }
        }
    }
    print!("\n");
}

fn progress(occupancy: &Array2<usize>, floor: &Array2<usize>, second: bool) -> Array2<usize> {
    let neighbors = neighbor_counter(
        occupancy,
        floor,
        if second { second_count } else { simple_count },
    );
    let concatted = stack![Axis(0), *occupancy, *floor, neighbors];

    // repr(&neighbors, floor, true);
    let ret = concatted.map_axis(Axis(0), |x| {
        let occupancy = x[0];
        let floor = x[1];
        let neighbors = x[2];
        if floor == 1 {
            0
        } else {
            if occupancy == 1 && if second { 5 } else { 4 } <= neighbors {
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

    let second = matches.is_present("second");

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
        let next = progress(&prev, &floor, second);
        // repr(&next, &floor, false);
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
    fn test_second_count_1() {
        assert_eq!(
            super::second_count(&ndarray::arr2(&[[0, 0, 0]]), (0, 0), (0, 1)),
            false
        );
    }

    #[test]
    fn test_second_count_2() {
        assert_eq!(
            super::second_count(&ndarray::arr2(&[[0, 1, 0]]), (0, 0), (0, 1)),
            true
        );
    }

    #[test]
    fn test_second_count_3() {
        assert_eq!(
            super::second_count(&ndarray::arr2(&[[1, 0, 0]]), (0, 1), (0, 1)),
            false
        );
    }

    #[test]
    fn test_second_count_4() {
        assert_eq!(
            super::second_count(&ndarray::arr2(&[[1, 0, 0]]), (0, 1), (0, -1)),
            true
        );
    }

    #[test]
    fn test_neighbor_counter_first_1() {
        assert_eq!(
            super::neighbor_counter(&ndarray::arr2(&[[0, 1], [0, 0]]), super::simple_count),
            ndarray::arr2(&[[1, 0], [1, 1]])
        );
    }

    #[test]
    fn test_neighbor_counter_first_2() {
        assert_eq!(
            super::neighbor_counter(
                &ndarray::arr2(&[[0, 1, 0], [1, 1, 1], [0, 1, 0]]),
                super::simple_count
            ),
            ndarray::arr2(&[[3, 3, 3], [3, 4, 3], [3, 3, 3]])
        );
    }

    #[test]
    fn test_neighbor_counter_second_1() {
        assert_eq!(
            super::neighbor_counter(
                &ndarray::arr2(&[
                    [1, 0, 0], //
                    [0, 0, 0], //
                    [1, 0, 0]
                ]),
                super::second_count
            ),
            ndarray::arr2(&[[1, 1, 2], [2, 2, 0], [1, 1, 2]])
        );
    }

    #[test]

    fn test_progress() {
        assert_eq!(
            super::progress(
                &ndarray::arr2(&[[0, 0, 0], [0, 0, 0], [0, 0, 0]]),
                &ndarray::Array2::zeros((3, 3)),
                false
            ),
            ndarray::Array2::ones((3, 3))
        );
        assert_eq!(
            super::progress(
                &ndarray::Array2::ones((3, 3)),
                &ndarray::Array2::zeros((3, 3)),
                false
            ),
            ndarray::arr2(&[[1, 0, 1], [0, 0, 0], [1, 0, 1]]),
        );
        assert_eq!(
            super::progress(
                &ndarray::arr2(&[[0, 0, 0], [0, 0, 0], [0, 0, 0]]),
                &ndarray::arr2(&[[1, 0, 0], [0, 1, 0], [0, 0, 1]]),
                false
            ),
            ndarray::arr2(&[[0, 1, 1], [1, 0, 1], [1, 1, 0]]),
        );
    }

    // #[test]
    // fn test_gen_directions() {
    //     assert_eq!(
    //         super::gen_directions(),
    //         [
    //             (-1, -1),
    //             (-1, 0),
    //             (-1, 1), //
    //             (0, -1),
    //             (0, 1), //
    //             (1, -1),
    //             (1, 0),
    //             (1, 1), //
    //         ]
    //     )
    // }

    #[test]
    fn test_gen_directions_iter() {
        assert_eq!(
            super::gen_directions_iter().collect::<Vec<(isize, isize)>>(),
            vec![
                (-1, -1),
                (-1, 0),
                (-1, 1), //
                (0, -1),
                (0, 1), //
                (1, -1),
                (1, 0),
                (1, 1), //
            ]
        )
    }
}
