use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    let filename = "./inputs/day1.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut columnOne = Vec::new();
    let mut columnTwo = Vec::new();
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        if line.chars().count() > 0 {
            let parts: Vec<&str> = line.split_whitespace().collect();

            if parts.len() == 2 {
                let first: i64 = parts[0].parse::<i64>().unwrap(); // Parse the first column
                let second: i64 = parts[1].parse::<i64>().unwrap(); // Parse the second column
                columnOne.push(first);
                columnTwo.push(second);
                println!("{} {}", first, second);
            } else {
                eprintln!("Skipping malformed line: {}", line);
            }
        }
    }
    columnOne.sort();
    columnTwo.sort();

    let mut distance: i64 = 0;
    for (index, num) in columnOne.iter().enumerate() {
        distance += (num - columnTwo[index]).abs();
        println!("Index: {}, Value: {}", index, num);
    }

    println!("distance: {}", distance);


}