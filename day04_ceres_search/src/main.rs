use core::str;
use std::{fmt::Debug, i32, io::{self, BufRead}, ops::{Index, IndexMut}};

fn main() {
    let input = read_input_from_stdin();
    let res = solve(&input);
    println!("{res}")
}

fn solve(table: &Table) -> usize {
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
        'outer: for dir in &dirs {
            for i in 1..NEEDLE.len() {
                let r = i32::try_from(first_pos.r).unwrap() + i32::try_from(i).unwrap() * dir.0;
                let c = i32::try_from(first_pos.c).unwrap() + i32::try_from(i).unwrap() * dir.1;
                let Ok(r_usize) = r.try_into() else {
                    continue 'outer;
                };
                let Ok(c_usize) = c.try_into() else {
                    continue 'outer;
                };
                let new_pos = Pos { r: r_usize, c: c_usize };
                if !table.contains_pos(new_pos) {
                    continue 'outer;
                }
                if table[new_pos] != NEEDLE[i] {
                    continue 'outer;
                }
            }
            num_occurrences += 1;
        }
    }
    num_occurrences
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

    fn contains_pos(&self, pos: Pos) -> bool {
        pos.r < self.rows && pos.c < self.cols
    }
}

impl Debug for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Table ({} rows, {} columns)\n", self.rows, self.cols)?;
        write!(f, "{{\n")?;
        for row in self.cells.chunks_exact(self.cols) {
            write!(f, "  {}\n", str::from_utf8(row).unwrap())?;
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
