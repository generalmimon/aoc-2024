// SPDX-FileCopyrightText: 2024 Petr Pucil <petr.pucil@seznam.cz>
//
// SPDX-License-Identifier: MIT

use std::io::{self, BufRead};

use utils::Table;

fn main() {
    let input = read_input_from_stdin();
    let res = solve_part2(&input);
    println!("{res}")
}

fn solve_part1(table: &Table) -> usize {
    let mut num_occurrences = 0;
    const NEEDLE: &[u8] = b"XMAS";
    let dirs = vec![
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];
    for first_pos in table.all_positions().filter(|&pos| table[pos] == NEEDLE[0]) {
        'outer: for &dir in &dirs {
            for i in 1..NEEDLE.len() {
                let Some(pos) = table.move_from_pos(first_pos, dir, isize::try_from(i).unwrap()) else {
                    continue 'outer;
                };
                if table[pos] != NEEDLE[i] {
                    continue 'outer;
                }
            }
            num_occurrences += 1;
        }
    }
    num_occurrences
}

fn solve_part2(table: &Table) -> usize {
    let mut num_occurrences = 0;
    for first_pos in table.all_positions().filter(|&pos| table[pos] == b'A') {
        let mut tl_br_ok = false;
        let mut bl_tr_ok = false;
        if let Some(tl_pos) = table.move_from_pos(first_pos, (-1, -1), 1) {
            if let Some(br_pos) = table.move_from_pos(first_pos, (1, 1), 1) {
                tl_br_ok =
                    (table[tl_pos] == b'M' && table[br_pos] == b'S') ||
                    (table[tl_pos] == b'S' && table[br_pos] == b'M');
            }
        }
        if let Some(bl_pos) = table.move_from_pos(first_pos, (1, -1), 1) {
            if let Some(tr_pos) = table.move_from_pos(first_pos, (-1, 1), 1) {
                bl_tr_ok =
                    (table[bl_pos] == b'M' && table[tr_pos] == b'S') ||
                    (table[bl_pos] == b'S' && table[tr_pos] == b'M');
            }
        }
        if tl_br_ok && bl_tr_ok {
            num_occurrences += 1;
        }
    }
    num_occurrences
}

fn read_input_from_stdin() -> Table {
    let mut cells = vec![];
    let mut stdin = io::stdin().lock();
    let mut line = vec![];
    stdin.read_until(b'\n', &mut line).unwrap();
    if line.last().map_or(false, |&x| x == b'\n') {
        line.pop();
    }
    let cols = line.len();
    cells.append(&mut line);

    loop {
        stdin.read_until(b'\n', &mut line).unwrap();
        if line.is_empty() {
            break;
        }
        if line.last().map_or(false, |&x| x == b'\n') {
            line.pop();
        }
        assert_eq!(line.len(), cols);
        cells.append(&mut line);
    }

    Table::new(cols, cells)
}
