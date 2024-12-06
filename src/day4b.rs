use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    let filename = "./inputs/day4.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut word_search: Vec<Vec<char>> = Vec::new();
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        if line.chars().count() > 0 {
            word_search.push(line.chars().collect());
        }
    }

    let mut total = 0;

    for (row, row_chars) in word_search.iter().enumerate() {
        for (col, value) in row_chars.iter().enumerate() {
               if col > 0 && row > 0 && row < word_search.len() -1 && col < word_search[0].len() -1 {
                   let mut left_diagonal: bool = false;
                   let mut right_diagonal: bool = false;
                   if value == &'A' {
                       if word_search[row-1][col-1] == 'M' {
                           if word_search[row+1][col+1] == 'S' {
                               left_diagonal = true;
                           }
                       } else if word_search[row-1][col-1] == 'S' {
                           if word_search[row+1][col+1] == 'M' {
                               left_diagonal = true;
                           }
                       }
                       if left_diagonal {
                           if word_search[row-1][col+1] == 'M' {
                               if word_search[row+1][col-1] == 'S' {
                                   right_diagonal = true;
                               }
                           } else if word_search[row-1][col+1] == 'S' {
                               if word_search[row+1][col-1] == 'M' {
                                   right_diagonal = true;
                               }
                           }
                       }
                       if left_diagonal && right_diagonal {
                           total += 1;
                       }
                   }
               }
        }
    }

    println!("Total: {}", total);
}