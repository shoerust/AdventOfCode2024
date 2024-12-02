use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    let filename = "./inputs/day2.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut total: i64 = 0;
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        if line.chars().count() > 0 {
            let levels: Vec<&str> = line.split_whitespace().collect();
            let int_levels: Vec<i64> = levels
                .iter()
                .filter_map(|&s| s.parse::<i64>().ok())
                .collect();
            let mut safe= check_safety(&int_levels);
            if !safe {
                for index in 0..levels.len() {
                    let mut cloned_levels = int_levels.clone();
                    cloned_levels.remove(index);
                    safe = check_safety(&cloned_levels);
                    if safe {
                        break;
                    }
                }
            }
            if safe {
                total += 1;
            }
        }
    }

    println!("total: {}", total);
}

fn check_safety(levels: &Vec<i64>) -> bool {
    let mut prev: i64 = -1;
    let mut safe: bool = true;
    let mut increasing: bool = false;
    for (index, level) in levels.iter().enumerate() {
        let current = level.abs();
        if index == 1 {
            if current > prev {
                increasing = true;
            }
        }
        if index > 0 {
            if increasing && (current <= prev) {
                safe = false;
                break;
            }
            if !increasing && (current >= prev) {
                safe = false;
                break;
            }
            if (prev - current).abs() > 3 {
                safe = false;
                break;
            }
        }
        prev = current;
    }

    safe
}