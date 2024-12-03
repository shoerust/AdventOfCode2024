use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

pub fn solve() {
    let filename = "./inputs/day3.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut total: i64 = 0;
    let mut enable: bool = true;
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        if line.chars().count() > 0 {

            let re = Regex::new(r"(do\(\)|don't\(\))|mul\([0-9]+,[0-9]+\)").unwrap();

            if re.is_match(&*line) {
                println!("The line contains code matching the pattern.");

                for mat in re.find_iter(&*line) {
                    println!("Found match: {}", mat.as_str());
                    if mat.as_str() == "do()" {
                        enable = true;
                    } else if mat.as_str() == "don't()" {
                        enable = false;
                    } else if enable && mat.as_str().contains("mul") {
                        let number_string = mat.as_str().replace("mul(", "");
                        let number_string = number_string.replace(")", "");
                        let numbers: Vec<&str> = number_string.split(",").collect();
                        total += numbers[0].parse::<i64>().unwrap() * numbers[1].parse::<i64>().unwrap();
                    }
                }
            } else {
                println!("No matches found.");
            }
        }
    }

    println!("total: {}", total);
}