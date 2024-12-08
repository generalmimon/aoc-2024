// SPDX-FileCopyrightText: 2024 Petr Pucil <petr.pucil@seznam.cz>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::{cmp::Ordering, collections::{HashMap, HashSet}, io};

fn main() {
    let input = read_input_from_stdin();
    // let res = solve_part1(&input);
    let res = solve_part2(input);
    println!("{res}");
}

fn solve_part1(input: &Input) -> u32 {
    input.updates
        .iter()
        .filter(|u| is_update_correct(u, &input.pages_expected_after))
        .map(|u| middle_page_number(u))
        .sum()
}

fn solve_part2(input: Input) -> u32 {
    let mut res = 0;
    for mut update in input.updates {
        if is_update_correct(&update, &input.pages_expected_after) {
            continue;
        }
        fix_update(&mut update, &input.pages_expected_after);
        res += middle_page_number(&update);
    }
    res
}

fn middle_page_number(update: &[u32]) -> u32 {
    update[update.len() / 2]
}

fn is_update_correct(update: &[u32], input_pages_expected_after: &HashMap<u32, HashSet<u32>>) -> bool {
    for (i, page) in update.iter().enumerate() {
        let pages_actually_before = &update[0..i];
        let Some(pages_expected_after) = input_pages_expected_after.get(&page) else {
            continue;
        };
        for &prev_page in pages_actually_before {
            if pages_expected_after.contains(&prev_page) {
                return false;
            }
        }
    }
    true
}

fn fix_update(update: &mut [u32], input_pages_expected_after: &HashMap<u32, HashSet<u32>>) {
    update.sort_by(|page_a, page_b| {
        if let Some(pages_after_a) = input_pages_expected_after.get(page_a) {
            if pages_after_a.contains(page_b) {
                return Ordering::Less;
            }
        }
        if let Some(pages_after_b) = input_pages_expected_after.get(page_b) {
            if pages_after_b.contains(page_a) {
                return Ordering::Greater;
            }
        }
        Ordering::Equal
    });
}

#[derive(Debug)]
struct Input {
    pages_expected_after: HashMap<u32, HashSet<u32>>,
    updates: Vec<Vec<u32>>,
}

fn read_input_from_stdin() -> Input {
    let mut pages_expected_after: HashMap<u32, HashSet<u32>> = HashMap::new();
    for line in io::stdin().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        let (page_a, page_b) = line.split_once('|').unwrap();
        let page_a = page_a.parse::<u32>().unwrap();
        let page_b = page_b.parse::<u32>().unwrap();
        let after_set = pages_expected_after.entry(page_a).or_default();
        after_set.insert(page_b);
    }

    let mut updates = vec![];
    for line in io::stdin().lines() {
        let line = line.unwrap();
        updates.push(line.split(',').map(|x| x.parse::<u32>().unwrap()).collect());
    }

    Input { pages_expected_after: pages_expected_after, updates }
}
