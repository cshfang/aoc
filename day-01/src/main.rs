// Rust
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // let _ = part1("data/sample1");
    let _ = part1("data/input");

    // let _ = part2("data/sample2");
    let _ = part2("data/input");
}

fn part1(dataset: &str) -> std::io::Result<()> {
    let file = File::open(format!("{dataset}"))?;
    let reader = BufReader::new(file);

    let mut total: i32 = 0;

    for line in reader.lines() {
        let line = line?;
        let mut first: i32 = -1;
        let mut second: i32 = -1;
        line.chars().for_each(|x| {
            if x.is_numeric() {
                if first < 0 {
                    first = x.to_digit(10).expect("should be a number") as i32;
                }
                second = x.to_digit(10).expect("should be a number") as i32;
            }
        });
        total += first * 10 + second
    }

    println!("Part 1: {total}");

    Ok(())
}

fn part2(dataset: &str) -> std::io::Result<()> {
    let file = File::open(format!("{dataset}"))?;
    let reader = BufReader::new(file);

    let mut total: i32 = 0;
    let patterns = [
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    for line in reader.lines() {
        let line = line?;
        let mut index1: i32 = -1;
        let mut index2: i32 = -1;
        let mut first: i32 = -1;
        let mut second: i32 = -1;

        for pattern in patterns.iter() {
            let index = line.find(pattern.0);
            let rindex = line.rfind(pattern.0);
            if !index.is_none() {
                let unwrapped = index.unwrap() as i32;
                if index1 == -1 || index1 > unwrapped {
                    index1 = unwrapped;
                    first = pattern.1;
                }
            }
            if !rindex.is_none() {
                let unwrapped = rindex.unwrap() as i32;
                if index2 == -1 || index2 < unwrapped {
                    index2 = unwrapped;
                    second = pattern.1;
                }
            }
        }
        total += first * 10 + second
    }

    println!("Part 2: {total}");

    Ok(())
}
