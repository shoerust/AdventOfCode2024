use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

// This is a brute force solution - it's not pretty!

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

    let mut total = 0;
    let mut path: Vec<(isize, isize)> = Vec::new();
    let mut exited = false;
    while !exited {
        if guard_pos.0 == (map.len() as isize - 1) || guard_pos.1 == (map[guard_pos.0 as usize].len() as isize - 1) {
            mark_seen(guard_pos, &mut map);
            path.push((guard_pos.0, guard_pos.1));
            exited = true;
        } else {
            let next_pos = (guard_pos.0 + directions[&guard].0, guard_pos.1 + directions[&guard].1);
            if map[next_pos.0 as usize][next_pos.1 as usize]  == '#' {
                guard = turns[&guard];
            } else {
                mark_seen(guard_pos, &mut map);
                guard_pos = next_pos;
                path.push((guard_pos.0, guard_pos.1));
            }
        }
    }

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

    print_map(map.clone());

    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if map[row][col] == 'X' {
                total += evaluate_obstacle(map.clone(), &directions, &turns, guard.clone(), guard_pos.clone(), (row as isize, col as isize));
            }
        }
    }

    println!("Total: {}", total);
}

fn evaluate_obstacle(mut map: Vec<Vec<char>>, directions: &HashMap<char, (isize, isize)>, turns: &HashMap<char, char>, mut guard: char, mut guard_pos: (isize, isize), obstacle_pos: (isize, isize)) -> i32 {
    let mut exited = false;
    map[obstacle_pos.0 as usize][obstacle_pos.1 as usize] = '#';

    let mut seen_map: HashMap<(isize, isize), Vec<char>> = HashMap::new();

    while !exited {
        if (guard_pos.0 == (map.len() as isize - 1)) ||
            (guard_pos.1 == (map[guard_pos.0 as usize].len() as isize - 1)) ||
                (guard_pos.0 == 0) ||
                (guard_pos.1 == 0) {
            exited = true;
        } else {
            let next_pos = (guard_pos.0 + directions[&guard].0, guard_pos.1 + directions[&guard].1);
            if seen_map.contains_key(&(guard_pos.0, guard_pos.1)) {
                if seen_map.get(&(guard_pos.0, guard_pos.1)).unwrap().contains(&guard) {
                    return 1
                }
                seen_map.get_mut(&(guard_pos.0.clone(), guard_pos.1.clone())).unwrap().push(guard.clone());
            } else {
                seen_map.insert((guard_pos.0, guard_pos.1), vec![guard.clone()]);
            }

            if map[next_pos.0 as usize][next_pos.1 as usize]  == '#' {
                guard = turns[&guard];
            } else {
                guard_pos = next_pos;
            }
        }
    }
    0
}

fn print_map(map: Vec<Vec<char>>) {
    for row in &map {
        for &cell in row {
            print!("{}", cell);
        }
        println!();
    }
    println!("-------");
}

fn mark_seen(guard_pos: (isize, isize), map: &mut Vec<Vec<char>>) {
    if map[guard_pos.0 as usize][guard_pos.1 as usize] == '.' {
        map[guard_pos.0 as usize][guard_pos.1 as usize] = 'X';
    }
}