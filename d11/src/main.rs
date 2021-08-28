// use arr_macro::arr;
use clap::{App, Arg};
use ndarray::{stack, Array, Array2, Axis};
// use std::cmp::max;
// use std::collections::HashMap;
use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};

// fn gen_directions() -> [(isize, isize); 8] {
//     let it = (-1..2).cartesian_product(-1..2);
//     let mut ret: [(isize, isize); 8] = Default::default();
//     // let ret = arr![(0, 0), 8];
//     let mut cntr = 0;
//     for (left, right) in it {
//         if left == 0 && right == 0 {
//             continue;
//         }
//         ret[cntr] = (left, right);
//         cntr += 1;
//     }
//     ret
// }

// fn gen_directions_iter() -> impl Iterator<Item = (isize, isize)> + 'static {
fn gen_directions_iter() -> impl Iterator<Item = (isize, isize)> + 'static {
    let ret = (-1..2)
        .cartesian_product(-1..2)
        .filter(|(left, right)| *left != 0 || *right != 0);
    ret
}

fn neighbor_counter(arr: &Array2<usize>, second: bool) -> Array2<usize> {
    let shape = arr.shape();
    let mut ret = Array::zeros((shape[0], shape[1]));

    for x in 0..(shape[0] as isize) {
        for y in 0..(shape[1] as isize) {
            let mut counter = 0;
            if second {
                // let mut diagonals = (Some((x,y)), Some((x,y)), Some((x,y)), Some((x,y)));
                // let NW = Some((x,y));
                // let NE = Some((x,y));
                // let SE = Some((x,y));
                // let SW = Some((x,y));
                // diagonals.0 = None;
                // let had_change = true;
                // while had_change {

                // }
                // let mut NW = false;
                // let mut NE = false;
                // let mut SE = false;
                // let mut SW = false;
                // for increment in 0 as isize..max(shape[0] as isize, shape[1] as isize) {
                //     // if x - increment > 0 and y
                //     if let Some(value) = arr.get((
                //         max(0, x - increment) as usize,
                //         max(0, y - increment) as usize,
                //     )) {
                //         if NW && value == &1 {
                //             NW = true;
                //         }
                //     }
                //     if let Some(value) =
                //         arr.get(((x + increment) as usize, max(0, y - increment) as usize))
                //     {
                //         if NE && value == &1 {
                //             NE = true;
                //         }
                //     }
                //     if let Some(value) =
                //         arr.get(((x + increment) as usize, (y + increment) as usize))
                //     {
                //         if SE && value == &1 {
                //             SE = true;
                //         }
                //     }
                //     if let Some(value) = arr.get((
                //         max(0, x - increment) as usize as usize,
                //         (y + increment) as usize,
                //     )) {
                //         if SW && value == &1 {
                //             SW = true;
                //         }
                //     }

                //     let res = (vec![NW, NE, SE, SW])
                //         .iter()
                //         .fold(0, |acc, x| if *x { acc + 1 } else { acc });
                //     println!("Res {}", res);
                // }
                let it = (-1..1).cartesian_product(-1..1);
                for (left, right) in it {
                    if left == 0 && right == 0 {
                        continue;
                    }
                }
            } else {
                for (left, right) in gen_directions_iter() {
                    counter += match arr.get(((x + left) as usize, (y + right) as usize)) {
                        Some(r) => *r,
                        None => 0,
                    }
                }
            }
            *ret.get_mut((x as usize, y as usize)).unwrap() = counter;
        }
    }
    ret
}

fn progress(occupancy: &Array2<usize>, floor: &Array2<usize>, second: bool) -> Array2<usize> {
    let neighbors = neighbor_counter(occupancy, second);
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
    fn test_neighbor_counter_first() {
        assert_eq!(
            super::neighbor_counter(&ndarray::arr2(&[[1, 2], [3, 4]]), false),
            ndarray::arr2(&[[9, 8], [7, 6]])
        );
        assert_eq!(
            super::neighbor_counter(&ndarray::arr2(&[[0, 1, 0], [1, 1, 1], [0, 1, 0]]), false),
            ndarray::arr2(&[[3, 3, 3], [3, 4, 3], [3, 3, 3]])
        );
    }

    #[test]
    fn test_neighbor_counter_second() {
        assert_eq!(
            super::neighbor_counter(
                &ndarray::arr2(&[
                    [1, 0, 0], //
                    [0, 0, 0], //
                    [1, 0, 0]
                ]),
                true
            ),
            ndarray::arr2(&[[3, 3, 3], [3, 4, 3], [3, 3, 3]])
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
