use itertools::Itertools;
use std::error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input") {
        let combinations = lines.into_iter().combinations(3);
        for combination in combinations {
            if combination.iter().sum::<i32>() == 2020 {
                println!("{}", combination.iter().product::<i32>());
            }
        }
    }
}

fn read_lines<P>(filename: P) -> Result<Vec<i32>, Box<dyn error::Error>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;

    Ok(io::BufReader::new(file)
        .lines()
        .into_iter()
        .map(|x| x.unwrap())
        .map(|d| d.trim().to_owned())
        .map(|f| i32::from_str_radix(&f, 10).unwrap())
        .collect())
}
