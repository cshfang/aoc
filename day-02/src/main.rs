// Rust
use std::collections::HashMap;
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

    let mut total = 0;
    let mut available_cubes = HashMap::new();
    available_cubes.insert("red", 12);
    available_cubes.insert("green", 13);
    available_cubes.insert("blue", 14);

    for line in reader.lines() {
        let line = line?;

        let line_split: Vec<&str> = line.split(":").collect();
        let game = line_split[0];
        let mut rounds = line_split[1].split(";");
        if !rounds.any(|round| {
            let mut found = false;
            let combos = round.split(",");
            combos.for_each(|combo| {
                let cubes: Vec<&str> = combo.trim().split(" ").collect();
                let number_cubes: i32 = cubes[0].parse().unwrap();
                let color = cubes[1];
                if !found {
                    found = number_cubes > *available_cubes.get(color).unwrap() as i32;
                }
            });
            found
        }) {
            let id_str: String = game.trim().chars().skip(5).collect();
            let id: i32 = id_str.parse().unwrap();
            total += id
        }
    }

    println!("Part 1: {total}");

    Ok(())
}

fn part2(dataset: &str) -> std::io::Result<()> {
    let file = File::open(format!("{dataset}"))?;
    let reader = BufReader::new(file);

    let mut total = 0;

    for line in reader.lines() {
        let line = line?;

        let line_split: Vec<&str> = line.split(":").collect();
        let rounds = line_split[1].split(";");
        let mut min_cubes = HashMap::new();
        min_cubes.insert("red", 0);
        min_cubes.insert("green", 0);
        min_cubes.insert("blue", 0);
        rounds.for_each(|round| {
            let combos = round.split(",");
            combos.for_each(|combo| {
                let cubes: Vec<&str> = combo.trim().split(" ").collect();
                let number_cubes: i32 = cubes[0].parse().unwrap();
                let color = cubes[1];
                if number_cubes > *min_cubes.get(color).unwrap() as i32 {
                    min_cubes.insert(color, number_cubes);
                }
            });
        });

        let power = min_cubes.get("red").unwrap()
            * (min_cubes.get("green").unwrap())
            * (min_cubes.get("blue").unwrap());
        total += power;
    }

    println!("Part 2: {total}");

    Ok(())
}
