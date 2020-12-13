use clap::{App, Arg};
use counter::Counter;
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
    // println!("Diffs {:?}", diffs);

    let counted_diffs = diffs.iter().collect::<Counter<_>>();
    let res =
        counted_diffs.get(&1).expect("No 1s found") * counted_diffs.get(&3).expect("No 3s found");
    println!("First {:?}", res);
}
