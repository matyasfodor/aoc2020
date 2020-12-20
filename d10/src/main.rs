use clap::{App, Arg};
use counter::Counter;
use factorial::Factorial;
use sorted_vec::SortedVec;
use std::fs::File;
use std::io::{self, BufRead};

fn pairwise<I>(right: I) -> impl Iterator<Item = (I::Item, I::Item)>
where
    I: IntoIterator + Clone,
{
    let left = right.clone().into_iter();
    left.zip(right.into_iter().skip(1))
}

fn get_combinations(n: &usize, upper_bound: usize) -> Vec<Vec<usize>> {
    if *n == 0 {
        return vec![vec![]];
    }
    let ones: Vec<Vec<usize>> = get_combinations(&(n - &1), 1)
        .iter()
        .map(|x| vec![1].iter().cloned().chain(x.iter().cloned()).collect())
        .collect();

    let twos: Vec<Vec<usize>> = if *n >= 2 && upper_bound >= 2 {
        get_combinations(&(n - &2), 2)
            .iter()
            .map(|x| vec![2].iter().cloned().chain(x.iter().cloned()).collect())
            .collect()
    } else {
        vec![]
    };

    let threes: Vec<Vec<usize>> = if *n >= 3 && upper_bound >= 3 {
        get_combinations(&(n - &3), 3)
            .iter()
            .map(|x| vec![3].iter().cloned().chain(x.iter().cloned()).collect())
            .collect()
    } else {
        vec![]
    };
    ones.into_iter()
        .chain(twos.into_iter())
        .chain(threes.into_iter())
        .collect()
}

fn number_of_variations(n: &usize) -> usize {
    get_combinations(n, 100)
        .iter()
        .map(|x| {
            let counted = x.iter().collect::<Counter<_>>();
            let divisor: usize = counted.iter().map(|x| x.1.factorial()).product();
            x.len().factorial() / divisor
        })
        .sum()
}

fn get_consecutive_blocks(arr: &Vec<usize>, n: &usize) -> Vec<usize> {
    let (mut blocks, next) = arr.iter().fold((vec![], 0), |mut acc, next| {
        if next != n {
            if acc.1 > 0 {
                acc.0.push(acc.1);
            }
            acc.1 = 0;
        } else {
            acc.1 += 1;
        }
        acc
    });
    if next > 0 {
        blocks.push(next);
    }
    blocks
}

fn main() {
    let matches = App::new("AOC solution 10")
        .arg(Arg::with_name("test").short("t").long("test"))
        .arg(Arg::with_name("second").short("s").long("second"))
        .get_matches();

    let path = if matches.is_present("test") {
        "test.txt"
    } else {
        "input.txt"
    };

    let file = File::open(path).expect("File not found");
    let mut adapters: SortedVec<usize> = SortedVec::from_unsorted(
        io::BufReader::new(file)
            .lines()
            .map(|x| {
                let res: usize = x.unwrap().parse().unwrap();
                res
            })
            .collect(),
    );
    adapters.insert(0);
    adapters.insert(adapters.last().unwrap() + 3);
    let diffs: Vec<usize> = pairwise(adapters.iter()).map(|x| x.1 - x.0).collect();

    let counted_diffs = diffs.iter().collect::<Counter<_>>();
    let res =
        counted_diffs.get(&1).expect("No 1s found") * counted_diffs.get(&3).expect("No 3s found");
    println!("First {:?}", res);

    let consecutive_1_blocks: Vec<usize> = get_consecutive_blocks(&diffs, &1);
    let res: usize = consecutive_1_blocks
        .iter()
        .map(number_of_variations)
        .product();
    println!("Second: {}", res);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(super::get_combinations(&1, 100), vec![vec![1]]);
        assert_eq!(super::get_combinations(&2, 100), vec![vec![1, 1], vec![2]]);
        assert_eq!(
            super::get_combinations(&3, 100),
            vec![vec![1, 1, 1], vec![2, 1], vec![3]]
        );
    }
    #[test]
    fn test_number_of_variations() {
        assert_eq!(super::number_of_variations(&1), 1);
        assert_eq!(super::number_of_variations(&2), 2);
        assert_eq!(super::number_of_variations(&3), 4);
        assert_eq!(super::number_of_variations(&4), 7);
    }
    #[test]

    fn test_get_consecutive_blocks() {
        assert_eq!(
            super::get_consecutive_blocks(&vec![1, 1, 1, 3, 1, 1, 3, 1, 3, 3, 1, 1, 1, 1], &1),
            vec![3, 2, 1, 4]
        );
    }
}
