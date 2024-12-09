// SPDX-FileCopyrightText: 2024 Petr Pucil <petr.pucil@seznam.cz>
//
// SPDX-License-Identifier: MIT

use std::{collections::HashSet, io};

use utils::{Pos, Table};

fn main() {
    let input = Table::from_reader(io::stdin().lock());
    // let res = solve_part1(input);
    let res = solve_part2(input);
    println!("{res}");
}

fn solve_part1(mut table: Table) -> usize {
    let mut guard_pos = table.all_positions()
        .find(|&pos| table[pos] == b'^')
        .unwrap();
    let mut guard_dir = (-1, 0);
    let mut num_distinct_positions = 0;
    loop {
        if table[guard_pos] != b'X' {
            num_distinct_positions += 1;
        }
        table[guard_pos] = b'X';
        let Some(next_pos) = table.move_from_pos(guard_pos, guard_dir, 1) else {
            break;
        };
        if table[next_pos] == b'#' {
            guard_dir = dir_right_hand(guard_dir);
            continue;
        }
        guard_pos = next_pos;
    }
    num_distinct_positions
}

fn solve_part2(mut table: Table) -> usize {
    let mut guard_pos = table.all_positions()
        .find(|&pos| table[pos] == b'^')
        .unwrap();
    let mut guard_dir = (-1, 0);
    let mut num_loops = 0;
    loop {
        table[guard_pos] = b'X';
        let Some(next_pos) = table.move_from_pos(guard_pos, guard_dir, 1) else {
            break;
        };
        if table[next_pos] == b'#' {
            guard_dir = dir_right_hand(guard_dir);
            continue;
        }
        let orig_cell = table[next_pos];
        if orig_cell == b'.' {
            table[next_pos] = b'#';
            if is_stuck_in_loop(&table, guard_pos, guard_dir) {
                num_loops += 1;
                table[next_pos] = b'@';
            } else {
                table[next_pos] = orig_cell;
            }
        }
        guard_pos = next_pos;
    }
    num_loops
}

fn is_stuck_in_loop(table: &Table, mut guard_pos: Pos, mut guard_dir: (isize, isize)) -> bool {
    let mut prev_states: HashSet<(Pos, (isize, isize))> = HashSet::new();
    for _ in 0..10_000 {
        {
            let state = (guard_pos, guard_dir);
            if prev_states.contains(&state) {
                return true;
            }
            prev_states.insert(state);
        }
        let Some(next_pos) = table.move_from_pos(guard_pos, guard_dir, 1) else {
            return false;
        };
        if table[next_pos] == b'#' {
            guard_dir = dir_right_hand(guard_dir);
            continue;
        }
        guard_pos = next_pos;
    }
    unreachable!()
}

fn dir_right_hand(dir: (isize, isize)) -> (isize, isize) {
    (dir.1, -dir.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn right_hand() {
        assert_eq!(dir_right_hand((0, 1)), (1, 0));
        assert_eq!(dir_right_hand((1, 0)), (0, -1));
    }

    #[test]
    fn stuck_in_loop() {
        let table = Table::new(4, vec![
            b'.', b'#', b'.', b'.',
            b'.', b'.', b'.', b'#',
            b'#', b'^', b'.', b'.',
            b'.', b'.', b'#', b'.',
        ]);
        assert!(is_stuck_in_loop(&table, table.all_positions().find(|&pos| table[pos] == b'^').unwrap(), (-1, 0)));

        let table = Table::new(4, vec![
            b'.', b'#', b'.', b'.',
            b'.', b'.', b'.', b'#',
            b'.', b'^', b'.', b'.',
            b'.', b'.', b'#', b'.',
        ]);
        assert!(!is_stuck_in_loop(&table, table.all_positions().find(|&pos| table[pos] == b'^').unwrap(), (-1, 0)));

        let table = Table::new(6, vec![
            b'.', b'#', b'.', b'#', b'.', b'.',
            b'.', b'.', b'.', b'.', b'.', b'#',
            b'#', b'.', b'v', b'.', b'.', b'.',
            b'.', b'.', b'#', b'.', b'.', b'.',
            b'.', b'.', b'.', b'.', b'#', b'.',
        ]);
        assert!(is_stuck_in_loop(&table, table.all_positions().find(|&pos| table[pos] == b'v').unwrap(), (1, 0)));
    }
}
