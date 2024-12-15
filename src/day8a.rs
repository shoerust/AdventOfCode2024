use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;


pub fn solve() {
    let filename = "./inputs/day8.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut total = 0;
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut antennas: HashMap<char, Vec<Location>> = HashMap::new();
    let mut rows: usize = 0;
    let mut cols: usize = 0;
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        if line.chars().count() > 0 {
            rows += 1;
            map.push(line.chars().collect());
            for i in 0..line.len() {
                let c: char = line.chars().nth(i).unwrap();
                if c != '.' {
                    if !antennas.contains_key(&c) {
                        antennas.insert(c, Vec::new());
                        antennas.get_mut(&c).unwrap().push(Location { x: i as isize, y: index as isize });
                    } else {
                        antennas.get_mut(&c).unwrap().push(Location { x: i as isize, y: index as isize });
                    }
                }
            }
            cols = line.len();
        }
    }

    let mut pairs: HashMap<char, Vec<(Location, Location)>> = HashMap::new();

    for antenna_id in antennas.keys() {
        if antenna_id != &'.' {
            let antenna_locations = antennas.get(antenna_id).unwrap();
            for i in 0..antenna_locations.len() {
                for j in 0..antenna_locations.len() {
                    if i != j && !already_calculated(&antenna_locations[i], &antenna_locations[j], &pairs.get(antenna_id)) {
                        let locations = calculate_antinode_locations(&antenna_locations[i], &antenna_locations[j]);
                        if location_on_map(rows, cols, &locations.0) {
                            if map[locations.0.y as usize][locations.0.x as usize] != '#' {
                                map[locations.0.y as usize][locations.0.x as usize] = '#';
                                total += 1;
                            }
                        }
                        if location_on_map(rows, cols, &locations.1) {
                            if map[locations.1.y as usize][locations.1.x as usize] != '#' {
                                map[locations.1.y as usize][locations.1.x as usize] = '#';
                                total += 1;
                            }
                        }
                        if pairs.contains_key(&antenna_id) {
                            pairs.get_mut(&antenna_id).unwrap().push((Location {x: antenna_locations[i].x, y: antenna_locations[i].y},
                                                                      Location { x: antenna_locations[j].x, y: antenna_locations[j].y}));
                        } else {
                            pairs.insert(antenna_id.clone(), Vec::new());
                            pairs.get_mut(&antenna_id).unwrap().push((Location {x: antenna_locations[i].x, y: antenna_locations[i].y},
                                                                      Location { x: antenna_locations[j].x, y: antenna_locations[j].y}));
                        }
                    }
                }
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

fn already_calculated(l1: &Location, l2: &Location, pairs: &Option<&Vec<(Location, Location)>>) -> bool {
    if pairs.is_some() {
        for pair in pairs.unwrap() {
            if l1.x == pair.0.x && l1.y == pair.0.y && l2.x == pair.1.x  && l2.y == pair.1.y {
                return true
            }
            if l2.x == pair.0.x && l2.y == pair.0.y && l1.x == pair.1.x  && l1.y == pair.1.y {
                return true
            }
        }
    }

    false
}

fn calculate_antinode_locations(a: &Location, b: &Location) -> (Location, Location) {
    let x_diff = (a.x - b.x).abs();
    let y_diff = (a.y - b.y).abs();

    let mut l1 = Location { x: 0, y: 0 };
    let mut l2 = Location { x: 0, y: 0 };

    if a.x <= b.x && a.y <= b.y {
        l1 = Location { x: a.x - x_diff, y: a.y - y_diff };
        l2 = Location { x: b.x + x_diff, y: b.y + y_diff };
    } else if a.x > b.x && a.y <= b.y {
        l1 = Location { x: a.x + x_diff, y: a.y - y_diff };
        l2 = Location { x: b.x - x_diff, y: b.y + y_diff };
    } else if a.x <= b.x && a.y > b.y {
        l1 = Location { x: a.x - x_diff, y: a.y + y_diff };
        l2 = Location { x: b.x + x_diff, y: b.y - y_diff };
    } else {
        l1 = Location { x: a.x + x_diff, y: a.y + y_diff };
        l2 = Location { x: b.x - x_diff, y: b.y - y_diff };
    }
    (l1, l2)
}

fn location_on_map(rows: usize, cols: usize, location: &Location) -> bool {
    if location.x > (cols - 1) as isize || location.y > (rows - 1) as isize {
        return false;
    }
    if location.x < 0 || location.y < 0 {
        return false;
    }
    true
}

struct Location {
    x: isize, // cols
    y: isize, // rows
}