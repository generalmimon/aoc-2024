use core::str;
use std::{fmt::Debug, io::{self, BufRead}, ops::{Index, IndexMut}};

fn main() {
    let input = read_input_from_stdin();
    let res = solve_part1(input);
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

fn dir_right_hand(dir: (isize, isize)) -> (isize, isize) {
    (dir.1, -dir.0)
}

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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
}
