use crate::utils;

pub fn run() {
    let input = utils::read_input("input/day01");
    let result = solve(&input);
    println!("Hello from day01: {}", result);
}

fn solve1a(input: &str) -> i64 {
    let big_list: Vec<&str> = input
        .split(|c| c == '\n' || c == ' ' && input.contains("   "))
        .filter(|s| !s.is_empty())
        .collect();

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();
    for (index, value) in big_list.iter().enumerate() {
        if index % 2 == 0 {
            left_list.push(value)
        } else {
            right_list.push(value)
        }
    }

    let mut left_list_parsed: Vec<i64> = left_list
        .iter()
        .map(|&s| s.parse::<i64>())
        .collect::<Result<_, _>>()
        .expect("Failed to parse right list as integers");

    let mut right_list_parsed: Vec<i64> = right_list
        .iter()
        .map(|&s| s.parse::<i64>())
        .collect::<Result<_, _>>()
        .expect("Failed to parse right list as integers");

    left_list_parsed.sort();
    right_list_parsed.sort();

    let diffs: Vec<i64> = left_list_parsed
        .iter()
        .zip(right_list_parsed.iter())
        .map(|(&num1, &num2)| (num1 - num2).abs())
        .collect();

    return diffs.iter().sum();
}

fn solve1b(input: &str) -> i64 {
    let big_list: Vec<&str> = input
        .split(|c| c == '\n' || c == ' ' && input.contains("   "))
        .filter(|s| !s.is_empty())
        .collect();

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();
    for (index, value) in big_list.iter().enumerate() {
        if index % 2 == 0 {
            left_list.push(value)
        } else {
            right_list.push(value)
        }
    }

    let mut left_list_parsed: Vec<i64> = left_list
        .iter()
        .map(|&s| s.parse::<i64>())
        .collect::<Result<_, _>>()
        .expect("Failed parsing left list to ints");

    let mut right_list_parsed: Vec<i64> = right_list
        .iter()
        .map(|&s| s.parse::<i64>())
        .collect::<Result<_, _>>()
        .expect("Failed parsing right list to ints");

    left_list_parsed.sort();

    right_list_parsed.sort();

    //noobie solution
    let mut acc_global = Vec::new();
    for element in &left_list_parsed {
        let mut acc: i64 = 0;
        for element2 in &right_list_parsed {
            let mut score = 0;
            if element == element2 {
                score += 1;
                acc += score * element;
                println!("e1:{}, e2:{}, acc:{}", element, element2, acc);
            }
        }
        acc_global.push(acc)
    }
    return acc_global.iter().sum();
}
