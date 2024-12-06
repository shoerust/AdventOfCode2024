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
    let directions = [
        (-1, 0), (1, 0), (0, -1), (0, 1),
        (-1, -1), (-1, 1), (1, -1), (1, 1),
    ];

    for (row, row_chars) in word_search.iter().enumerate() {
        for (col, _) in row_chars.iter().enumerate() {
            for &direction in &directions {
                total += count(&word_search, "XMAS", row as isize, col as isize, direction);
            }
        }
    }

    println!("Total: {}", total);
}

fn count(grid: &[Vec<char>], word: &str, row: isize, col: isize, direction: (isize, isize)) -> usize {
    let mut r = row;
    let mut c = col;
    let word_chars: Vec<char> = word.chars().collect();

    for &ch in &word_chars {
        if r < 0 || c < 0 || r >= grid.len() as isize || c >= grid[0].len() as isize || grid[r as usize][c as usize] != ch {
            return 0;
        }
        r += direction.0;
        c += direction.1;
    }
    1
}