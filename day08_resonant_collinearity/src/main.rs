// SPDX-FileCopyrightText: 2024 Petr Pucil <petr.pucil@seznam.cz>
//
// SPDX-License-Identifier: MIT

use std::{collections::HashMap, io};

use itertools::Itertools;
use utils::{Pos, Table};

fn main() {
    let input = Table::from_reader(io::stdin().lock());
    // let res = solve_part1(input);
    let res = solve_part2(input);
    println!("{res}");
}

fn solve_part1(mut table: Table) -> usize {
    let antenna_positions_by_freq = get_antenna_positions_by_freq(&table);

    let mut num_antinodes = 0;
    for (&_freq, antenna_positions) in &antenna_positions_by_freq {
        for (a, b) in antenna_positions.iter().tuple_combinations() {
            let diff = b.diff(&a);
            if let Some(ab_antinode) = table.move_from_pos(*b, diff, 1) {
                if table[ab_antinode] != b'#' {
                    table[ab_antinode] = b'#';
                    num_antinodes += 1;
                }
            }
            if let Some(ba_antinode) = table.move_from_pos(*a, diff, -1) {
                if table[ba_antinode] != b'#' {
                    table[ba_antinode] = b'#';
                    num_antinodes += 1;
                }
            }
        }
    }

    num_antinodes
}

fn solve_part2(mut table: Table) -> usize {
    let antenna_positions_by_freq = get_antenna_positions_by_freq(&table);

    let mut num_antinodes = 0;
    for (&_freq, antenna_positions) in &antenna_positions_by_freq {
        for (a, b) in antenna_positions.iter().tuple_combinations() {
            let diff = b.diff(&a);
            for i in 0.. {
                let Some(ab_antinode) = table.move_from_pos(*a, diff, i) else {
                    break;
                };
                if table[ab_antinode] != b'#' {
                    table[ab_antinode] = b'#';
                    num_antinodes += 1;
                }
            }
            for i in (1..).map(|n| -n) {
                let Some(ba_antinode) = table.move_from_pos(*a, diff, i) else {
                    break;
                };
                if table[ba_antinode] != b'#' {
                    table[ba_antinode] = b'#';
                    num_antinodes += 1;
                }
            }
        }
    }

    num_antinodes
}

fn get_antenna_positions_by_freq(table: &Table) -> HashMap<u8, Vec<Pos>> {
    let mut res: HashMap<u8, Vec<Pos>> = HashMap::new();
    let it = table
        .all_positions()
        .map(|pos| (pos, table[pos]))
        .filter(|&(_, c)| is_antenna(c));
    for (pos, freq) in it {
        res.entry(freq).or_default().push(pos);
    }
    res
}

fn is_antenna(c: u8) -> bool {
    c.is_ascii_lowercase() || c.is_ascii_uppercase() || c.is_ascii_digit()
}
