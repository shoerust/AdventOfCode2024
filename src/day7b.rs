use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    let filename = "./inputs/day7.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut total: i64 = 0;
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        if line.chars().count() > 0 {
            let answer_split: Vec<&str> = line.split(":").collect::<Vec<&str>>();
            let answer = answer_split[0].parse::<i64>().unwrap();
            let equation = answer_split[1].trim().split(" ").map(|s| s.parse::<i64>().unwrap()).collect();
            let permutations = generate_permutations(equation);

            for (_equation, result) in permutations {
                if result == answer {
                    total += answer;
                    break;
                }
            }
        }
    }

    println!("Total: {}", total);
}

fn generate_permutations(numbers: Vec<i64>) -> Vec<(String, i64)> {
    let n = numbers.len() - 1; // Number of operator slots
    let mut results = Vec::new();
    let operators = ["*", "+", "||"];

    fn permutation<'a>(
        n: usize,
        current: &mut Vec<&'a str>,
        operators: &'a [&'a str],
        numbers: &Vec<i64>,
        results: &mut Vec<(String, i64)>,
    ) {
        if current.len() == n {

            let mut equation = String::new();
            let mut total = numbers[0];
            for (i, &operator) in current.iter().enumerate() {
                equation.push_str(&numbers[i].to_string());
                equation.push_str(operator);

                if operator == "*" {
                    total *= numbers[i + 1];
                } else if operator == "+" {
                    total += numbers[i + 1];
                } else {
                    let mut concatenated: String = total.to_string();
                    concatenated.push_str(numbers[i+1].to_string().as_str());
                    total = concatenated.parse::<i64>().unwrap();
                }
            }
            equation.push_str(&numbers.last().unwrap().to_string());

            results.push((equation, total));
            return;
        }

        for &operator in operators {
            current.push(operator);
            permutation(n, current, operators, numbers, results);
            current.pop();
        }
    }

    permutation(n, &mut Vec::new(), &operators, &numbers, &mut results);

    results
}