use std::{
    error,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

struct Line {
    min: i32,
    max: i32,
    policy: String,
    password: String,
}

fn main() {
    first();
    second();
}

fn first() {
    let mut res = 0;
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            let count = line.password.matches(&line.policy).count();
            if line.min <= count as i32 && line.max >= count as i32 {
                res = res + 1;
            }
        }
        println!("{}", res);
    }
}

fn second() {
    let mut res = 0;
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            let mut n = 0;
            if line.password.chars().nth((line.min - 1) as usize).unwrap()
                == line.policy.chars().nth(0).unwrap()
            {
                n = n + 1;
            }

            if line.password.chars().nth((line.max - 1) as usize).unwrap()
                == line.policy.chars().nth(0).unwrap()
            {
                n = n + 1;
            }

            if n == 1 {
                res = res + 1;
            }
        }
        println!("{}", res);
    }
}

fn read_lines<P>(filename: P) -> Result<Vec<Line>, Box<dyn error::Error>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;

    Ok(io::BufReader::new(file)
        .lines()
        .into_iter()
        .map(|x| x.unwrap())
        .map(|d| d.trim().to_owned())
        .map(|s| {
            let parts = s.split(':').collect::<Vec<&str>>();
            let policy: Vec<&str> = parts[0].split(' ').collect();
            let password = parts[1].trim();
            let minmax: Vec<&str> = policy[0].split('-').collect();

            return Line {
                min: i32::from_str_radix(minmax[0], 10).unwrap(),
                max: i32::from_str_radix(minmax[1], 10).unwrap(),
                policy: String::from(policy[1]),
                password: String::from(password),
            };
        })
        .collect())
}
