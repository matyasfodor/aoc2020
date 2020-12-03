use clap::{App, Arg};
use std::fs::File;
use std::io::{self, BufRead};
use std::ops;
use std::path::Path;

// use std::str::Chars;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Coordinates {
    x: usize,
    y: usize,
}

impl ops::Add<&Coordinates> for Coordinates {
    type Output = Coordinates;

    fn add(self, _rhs: &Coordinates) -> Coordinates {
        Coordinates {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl std::fmt::Display for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// struct Cursor {
//     width: usize,
//     step: Coordinates,
//     coords: Coordinates,
// }

// impl Cursor {

// }

fn main() {
    let matches = App::new("AOC solution 1")
        .arg(Arg::with_name("test").short("t").long("test"))
        .arg(Arg::with_name("second").short("s").long("second"))
        .get_matches();

    let path = if matches.is_present("test") {
        "test.txt"
    } else {
        "input.txt"
    };

    let step = Coordinates { x: 3, y: 1 };
    let mut current_coords = Coordinates { x: 0, y: 0 };

    let mut tree_count = 0;
    let mut lines = read_lines(path).expect("File does not exist");
    let mut line_content = lines.next().unwrap().unwrap();
    let len = line_content.len();
    let mut ok = true;

    while ok {
        if line_content.chars().nth(current_coords.x % len).unwrap() == '#' {
            println!("Tree found");
            tree_count += 1;
        }
        current_coords = current_coords + &step;
        while current_coords.y > 0 {
            println!("Line digested");
            println!("line content           {}", line_content);
            // line_content = ;
            // let next_result = lines.next();
            // next_result
            if let Some(Ok(new_line_content)) = lines.next() {
                line_content = new_line_content;
            } else {
                ok = false;
                break;
            }
            current_coords.y -= 1;
        }
        // current_coords.x = current_coords.x % len;
        println!("Current coords {:}", current_coords);
    }
    // for line in lines {
    // if let Ok(line_content) = line {
    // if current_coords.y > 0 {
    //     println!("Line digested");
    //     println!("line content           {}", line_content);
    //     current_coords.y -= 1;
    // }
    // Could be stored as result
    // if line_content.chars().nth(current_coords.x % len).unwrap() == '#' {
    //     println!("Tree found");
    //     tree_count += 1;
    // }
    // current_coords = current_coords + &step;
    // while current_coords.y > 0 {
    //     println!("Line digested");
    //     println!("line content           {}", line_content);
    //     line_content = lines.next();
    //     current_coords.y -= 1;
    // }
    // current_coords.x = current_coords.x % len;
    // println!("Current coords {:}", current_coords);
    // }
    // }
    println!("There are {} trees encountered alltogether", tree_count);
}
