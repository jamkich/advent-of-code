use crate::utils;

pub fn run() {
    let input = utils::read_input("input/day02");
    let result = solve(&input);
    println!("{}", result);
}

fn solve1a(input: &str) -> i32 {
    let reports: Vec<&str> = input.split('\n').filter(|s| !s.is_empty()).collect();

    let mut acc = 0;
    for r in reports.iter() {
        let numbers: Vec<i32> = r
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();

        let is_ascending = numbers
            .windows(2)
            .all(|window| window[0] < window[1] && window[1] - window[0] <= 3);
        let is_descending = numbers
            .windows(2)
            .all(|window| window[0] > window[1] && window[0] - window[1] <= 3);

        if is_ascending || is_descending {
            acc += 1
        }
    }
    acc
}
fn solve(input: &str) -> i32 {
    let reports: Vec<&str> = input.split('\n').filter(|s| !s.is_empty()).collect();

    let mut acc = 0;
    for r in reports.iter() {
        let numbers: Vec<i32> = r
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();

        // ascending
        if numbers
            .windows(2)
            .all(|window| window[0] < window[1] && window[1] - window[0] <= 3)
        {
            acc += 1
        }

        // descending
        if numbers
            .windows(2)
            .all(|window| window[0] > window[1] && window[0] - window[1] <= 3)
        {
            acc += 1
        }
    }
    acc
}
