use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;


pub fn solve() {
    let filename = "./inputs/day6.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut map: Vec<Vec<char>> = Vec::new();
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        if line.chars().count() > 0 {
            map.push(line.chars().collect());
        }
    }

    let mut guard_pos: (isize, isize) = (0,0);
    let mut guard: char = '?';
    let mut directions: HashMap<char, (isize, isize)> = HashMap::new();
    directions.insert('^', (-1, 0));
    directions.insert('>', (0, 1));
    directions.insert('v', (1, 0));
    directions.insert('<', (0, -1));

    let mut turns: HashMap<char, char> = HashMap::new();
    turns.insert('^', '>');
    turns.insert('>', 'v');
    turns.insert('v', '<');
    turns.insert('<', '^');

    //find guard
    for (row, row_chars) in map.iter().enumerate() {
        for (col, _) in row_chars.iter().enumerate() {
            if directions.contains_key(&map[row][col]) {
                guard_pos = (row as isize, col as isize);
                guard = *row_chars.get(col).unwrap();
                break;
            }
        }
    }

    let mut total = 1; // starting position
    let mut exited = false;
    while !exited {
        if guard_pos.0 == (map.len() as isize - 1) || guard_pos.1 == (map[guard_pos.0 as usize].len() as isize - 1) {
            total += mark_seen(guard_pos, &mut map);
            exited = true;
        } else {
            let next_pos = (guard_pos.0 + directions[&guard].0, guard_pos.1 + directions[&guard].1);
            if map[next_pos.0 as usize][next_pos.1 as usize]  == '#' {
                guard = turns[&guard];
            } else {
                total += mark_seen(guard_pos, &mut map);
                guard_pos = next_pos;
            }
        }
    }

    for row in &map {
        for &cell in row {
            print!("{}", cell);
        }
        println!();
    }

    println!("Total: {}", total);
}

fn mark_seen(guard_pos: (isize, isize), map: &mut Vec<Vec<char>>) -> i32 {
    let mut seen = 0;
    if map[guard_pos.0 as usize][guard_pos.1 as usize] == '.' {
        map[guard_pos.0 as usize][guard_pos.1 as usize] = 'X';
        seen+=1
    }
    seen
}