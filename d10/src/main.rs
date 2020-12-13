use clap::{App, Arg};
use counter::Counter;
use sorted_vec::SortedVec;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter;

fn pairwise<I>(right: I) -> impl Iterator<Item = (I::Item, I::Item)>
where
    I: IntoIterator + Clone,
{
    let left = right.clone().into_iter();
    left.zip(right.into_iter().skip(1))
}

fn get_combinations(n: usize, upper_bound: usize) -> Vec<Vec<usize>> {
    if n == 0 {
        return vec![vec![]];
    }
    let ones: Vec<Vec<usize>> = get_combinations(n - 1, 1)
        .iter()
        .map(|x| vec![1].iter().cloned().chain(x.iter().cloned()).collect())
        .collect();

    let twos: Vec<Vec<usize>> = if n >= 2 && upper_bound >= 2 {
        get_combinations(n - 2, 2)
            .iter()
            .map(|x| vec![2].iter().cloned().chain(x.iter().cloned()).collect())
            .collect()
    } else {
        vec![]
    };

    let threes: Vec<Vec<usize>> = if n >= 3 && upper_bound >= 3 {
        get_combinations(n - 3, 3)
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
    println!("Diffs {:?}", diffs);

    let counted_diffs = diffs.iter().collect::<Counter<_>>();
    let res =
        counted_diffs.get(&1).expect("No 1s found") * counted_diffs.get(&3).expect("No 3s found");
    println!("First {:?}", res);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(super::get_combinations(1, 100), vec![vec![1]]);
        assert_eq!(super::get_combinations(2, 100), vec![vec![1, 1], vec![2]]);
        assert_eq!(
            super::get_combinations(3, 100),
            vec![vec![1, 1, 1], vec![2, 1], vec![3]]
        );
    }
}
