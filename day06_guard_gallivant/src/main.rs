// SPDX-FileCopyrightText: 2024 Petr Pucil <petr.pucil@seznam.cz>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use core::str;
use std::{collections::HashSet, fmt::Debug, io::{self, BufRead}, ops::{Index, IndexMut}};

fn main() {
    let input = read_input_from_stdin();
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

#[derive(Clone)]
struct Table {
    rows: usize,
    cols: usize,
    cells: Vec<u8>,
}

impl Table {
    fn new(cols: usize, cells: Vec<u8>) -> Self {
        let rows;
        if cols != 0 {
            assert_eq!(cells.len() % cols, 0);
            rows = cells.len() / cols;
        } else {
            assert_eq!(cells.len(), 0);
            rows = 0;
        }
        Self { rows, cols, cells }
    }

    fn all_positions(&self) -> impl Iterator<Item = Pos> + use<'_> {
        // https://stackoverflow.com/q/53722749/12940655
        (0..self.rows).flat_map(|r| (0..self.cols).map(move |c| Pos { r, c }))
    }

    fn move_from_pos(&self, pos: Pos, dir: (isize, isize), mult: isize) -> Option<Pos> {
        let r: isize = isize::try_from(pos.r).ok()? + dir.0 * mult;
        let c: isize = isize::try_from(pos.c).ok()? + dir.1 * mult;
        let new_pos = Pos { r: r.try_into().ok()?, c: c.try_into().ok()? };
        if self.contains_pos(new_pos) {
            Some(new_pos)
        } else {
            None
        }
    }

    fn contains_pos(&self, pos: Pos) -> bool {
        pos.r < self.rows && pos.c < self.cols
    }
}

impl Debug for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Table ({} rows, {} columns)\n", self.rows, self.cols)?;
        write!(f, "{{\n")?;
        for row in self.cells.chunks_exact(self.cols) {
            write!(f, "{}\n", str::from_utf8(row).unwrap())?;
        }
        write!(f, "}}\n")?;
        Ok(())
    }
}

impl Index<Pos> for Table {
    type Output = u8;

    fn index(&self, pos: Pos) -> &Self::Output {
        assert!(self.contains_pos(pos));
        &self.cells[pos.r * self.cols + pos.c]
    }
}

impl IndexMut<Pos> for Table {
    fn index_mut(&mut self, pos: Pos) -> &mut Self::Output {
        assert!(pos.r < self.rows);
        assert!(pos.c < self.cols);
        &mut self.cells[pos.r * self.cols + pos.c]
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Pos {
    r: usize,
    c: usize,
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
