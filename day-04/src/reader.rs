use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_by_line(dataset: &str) -> std::io::Result<Vec<String>> {
    let file = File::open(format!("{dataset}"))?;
    let reader = BufReader::new(file);

    let mut vector: Vec<String> = Vec::new();

    for line in reader.lines() {
        vector.push(line?);
    }

    Ok(vector)
}
