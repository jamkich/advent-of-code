use std::collections::{HashMap, HashSet};

use crate::utils;

pub fn run() {
    let input = utils::read_input("input/day05");
    let test = utils::read_input("input/test");
    let result = solve(&test);
    println!(" {}", result);
}

fn solve(input: &str) -> i32 {
    let sum = 0;
    let pi = input.split("\n\n").collect::<Vec<&str>>();
    let rules = pi[0].split("\n").collect::<Vec<&str>>();
    let updates = pi[1].split("\n").collect::<Vec<&str>>();
    // 1. get update
    // 2. find all rules contain page number
    // 3. make order followed by rules
    // 4. compare with update

    for update in updates.iter() {
        let mut sati_rules = HashSet::new();

        for rule in rules.iter() {
            let pages = rule.split("|").collect::<Vec<&str>>();
            let contains_all = pages.iter().all(|page| update.contains(page));
            if contains_all {
                let left = pages[0];
                let right = pages[1];

                let left_pos = update.find(left);
                let right_pos = update.find(right);

                if let (Some(left_idx), Some(right_idx)) = (left_pos, right_pos) {
                    if left_idx < right_idx {
                        sati_rules.insert(rule.to_string());
                    } else {
                        continue;
                    }
                } else {
                    println!(
                        "One of the pages in rule {} is missing from update {}",
                        rule, update
                    );
                }
            } else {
                continue;
            }
        }
    }
    sum
}
