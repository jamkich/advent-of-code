use crate::utils;
use regex::Regex;

pub fn run() {
    let input = utils::read_input("input/day03");
    let result = solve1b(&input);
    println!(" {}", result);
}

fn solve1a(input: &str) -> i32 {
    let pattern = r"mul\(\d+,\d+\)";
    let re = Regex::new(pattern).expect("very bad regex pattern");
    let mut score = 0;

    for l in re.find_iter(input) {
        let exp = l.as_str();
        println!("check matches {} ", exp);
        if let Some(inner) = exp.strip_prefix("mul(").and_then(|s| s.strip_suffix(")")) {
            let parts: Vec<&str> = inner.split(',').collect();
            if parts.len() == 2 {
                let num1: i32 = parts[0].parse().unwrap();
                let num2: i32 = parts[1].parse().unwrap();
                score += num1 * num2;
            }
        }
    }

    score
}

fn solve1b(input: &str) -> i32 {
    // (?<!don't\(\) look around is not supported...
    let pattern = r"mul\(\d+,\d+\)";
    let re = Regex::new(pattern).expect("very bad regex pattern");

    let mut score = 0;
    let mut enabled = true;

    let mut last_position = 0;
    for mat in re.find_iter(input) {
        let match_start = mat.start();
        let match_end = mat.end();
        let exp = &input[match_start..match_end];

        if let Some(_pos) = input[last_position..match_start].find("do()") {
            enabled = true;
        } else if let Some(_pos) = input[last_position..match_start].find("don't()") {
            enabled = false;
        }

        if enabled {
            if let Some(inner) = exp.strip_prefix("mul(").and_then(|s| s.strip_suffix(")")) {
                let parts: Vec<&str> = inner.split(',').collect();
                if parts.len() == 2 {
                    let num1: i32 = parts[0].parse().unwrap();
                    let num2: i32 = parts[1].parse().unwrap();
                    score += num1 * num2;
                }
            }
        }
        last_position = match_end;
    }
    score
}
