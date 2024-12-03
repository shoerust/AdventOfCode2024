use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

pub fn solve() {
    let filename = "./inputs/day3.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut total: i64 = 0;
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        if line.chars().count() > 0 {
            let pattern = r"mul\([0-9]+,[0-9]+\)";

            let re = Regex::new(pattern).unwrap();

            let matches: Vec<&str> = re.find_iter(line.as_str()).map(|m| m.as_str()).collect();

            for match_ in matches {
                let number_string = match_.replace("mul(", "");
                let number_string = number_string.replace(")", "");
                let numbers: Vec<&str> = number_string.split(",").collect();
                total += numbers[0].parse::<i64>().unwrap() * numbers[1].parse::<i64>().unwrap();
            }
        }
    }

    println!("total: {}", total);
}