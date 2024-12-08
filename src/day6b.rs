use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap};

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
    let mut exited = false;
    while !exited {
        if guard_pos.0 == (map.len() as isize - 1) || guard_pos.1 == (map[guard_pos.0 as usize].len() as isize - 1) ||
            guard_pos.0 == 0 || guard_pos.1 == 0 {
            mark_seen(guard_pos, &mut map);
            exited = true;
        } else {
            let mut turned = false;
            let next_pos = (guard_pos.0 + directions[&guard].0, guard_pos.1 + directions[&guard].1);
            if next_pos.0 > -1 && next_pos.1 > -1 {
                if map[next_pos.0 as usize][next_pos.1 as usize]  == '#' ||
                    map[next_pos.0 as usize][next_pos.1 as usize]  == '$' {
                    map[next_pos.0 as usize][next_pos.1 as usize] = '$';
                    guard = turns[&guard];
                    turned = true;
                } else {
                    mark_seen(guard_pos, &mut map);
                    if guard == '^' {
                        for i in guard_pos.1..map[guard_pos.0 as usize].len() as isize {
                            if map[guard_pos.0 as usize][i as usize] == '#' {
                                break;
                            }
                            if map[guard_pos.0 as usize][i as usize] == '$' {
                                map[next_pos.0 as usize][next_pos.1 as usize] = 'O';
                                break;
                            }
                        }
                    }
                    if guard == 'v' {
                        for i in (0..guard_pos.1).rev() {
                            if map[guard_pos.0 as usize][i as usize] == '#' {
                                break;
                            }
                            if map[guard_pos.0 as usize][i as usize] == '$' {
                                map[next_pos.0 as usize][next_pos.1 as usize] = 'O';
                                break;
                            }
                        }
                    }
                    if guard == '>' {
                        for i in guard_pos.0..map.len() as isize {
                            if map[i as usize][guard_pos.1 as usize] == '#' {
                                break;
                            }
                            if map[i as usize][guard_pos.1 as usize] == '$' {
                                map[next_pos.0 as usize][next_pos.1 as usize] = 'O';
                                break;
                            }
                        }
                    }
                    if guard == '<' {
                        for i in (0..guard_pos.0).rev() {
                            if map[i as usize][guard_pos.1 as usize] == '#' {
                                break;
                            }
                            if map[i as usize][guard_pos.1 as usize] == '$' {
                                map[next_pos.0 as usize][next_pos.1 as usize] = 'O';
                                break;
                            }
                        }
                    }
                }
            }
            if !turned {
                guard_pos = next_pos;
            }
        }

    }

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'O' {
                total += 1
            }
        }
    }


    print_map(&map);
    println!("Total: {}", total);
}

fn print_map(map: &Vec<Vec<char>>) {
    for row in map {
        for &cell in row {
            print!("{}", cell);
        }
        println!();
    }
    println!("-------")
}

fn mark_seen(guard_pos: (isize, isize), map: &mut Vec<Vec<char>>) {
    if map[guard_pos.0 as usize][guard_pos.1 as usize] == '.' {
        map[guard_pos.0 as usize][guard_pos.1 as usize] = 'X';
    }
}