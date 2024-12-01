use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

pub fn solve() {
    let filename = "./inputs/day1.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut frequency = HashMap::new();
    let mut column_two = Vec::new();
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        if line.chars().count() > 0 {
            let parts: Vec<&str> = line.split_whitespace().collect();

            if parts.len() == 2 {
                let first: i64 = parts[0].parse::<i64>().unwrap(); // Parse the first column
                let second: i64 = parts[1].parse::<i64>().unwrap(); // Parse the second column
                frequency.insert(first, 0);
                column_two.push(second);
                println!("{} {}", first, second);
            } else {
                eprintln!("Skipping malformed line: {}", line);
            }
        }
    }

    let mut total: i64 = 0;
    for num in &column_two {
        if frequency.contains_key(num) {
            frequency.insert(num.abs(), frequency.get(num).unwrap() + 1i64);
        }
    }

    for (key, value) in &frequency {
        total += key * value;
    }

    println!("total: {}", total);


}