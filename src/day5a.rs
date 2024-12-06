use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    let filename = "./inputs/day5.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut rules: HashMap<String, Vec<String>> = HashMap::new();
    let mut page_lines: Vec<String> = vec![];

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        if line.chars().count() > 0 {
            if line.contains("|") {
                let numbers: Vec<&str> = line.split('|').collect();
                let before = numbers[0].to_string();
                let after = numbers[1].to_string();
                if rules.contains_key(&before) {
                    rules.get_mut(&before).unwrap().push(after);
                } else {
                    rules.insert(before, vec![after]);
                }
            } else {
                page_lines.push(line);
            }
        }
    }

    let mut total = 0;

    for (index, pages) in page_lines.iter().enumerate() {
        let mut valid: bool = true;
        let mut page_values: Vec<String> = vec![];
        pages.split(",").for_each(|page| {
            page_values.push(page.to_string());
        });
        for (index, value) in page_values.iter().enumerate() {
            let afters: &Vec<String> = &rules[value];
            for after in afters {
                for i in 0..index {
                    if page_values[i].eq(after) {
                        valid = false;
                        break;
                    }
                }
            }
        }
        if valid {
            let middle_index = page_values.len() / 2;
            let middle_value = &page_values[middle_index];
            total += middle_value.parse::<i32>().unwrap();
        }
    }

    println!("Total: {}", total);
}