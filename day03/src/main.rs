use std::{
    error,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    first();

    second();
}

fn first() {
    if let Ok(lines) = read_lines("./input") {
        let trees = slope(&lines, 3, 1);
        println!("{}", trees);
    }
}

fn second() {
    if let Ok(lines) = read_lines("./input") {
        let mut trees = slope(&lines, 1, 1);
        trees *= slope(&lines, 3, 1);
        trees *= slope(&lines, 5, 1);
        trees *= slope(&lines, 7, 1);
        trees *= slope(&lines, 1, 2);
        println!("{}", trees);
    }
}

fn slope(lines: &Vec<String>, dx: usize, dy: usize) -> usize {
    let mut x = 0;
    let mut y = 0;

    let mut trees: usize = 0;
    while y < lines.len() {
        if lines[y].chars().nth(x).unwrap() == '#' {
            trees += 1;
        }
        x += dx;
        x %= lines[0].len();
        y += dy;
    }

    return trees;
}

fn read_lines<P>(filename: P) -> Result<Vec<String>, Box<dyn error::Error>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;

    Ok(io::BufReader::new(file)
        .lines()
        .into_iter()
        .map(|x| x.unwrap())
        .map(|d| d.trim().to_owned())
        .collect())
}
