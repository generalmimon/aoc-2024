use core::str;
use std::{fmt::Debug, io::BufRead, ops::{Index, IndexMut}};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Pos {
    r: usize,
    c: usize,
}

#[derive(Clone)]
pub struct Table {
    rows: usize,
    cols: usize,
    cells: Vec<u8>,
}

impl Table {
    pub fn new(cols: usize, cells: Vec<u8>) -> Self {
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

    // https://rust-lang.github.io/api-guidelines/interoperability.html#generic-readerwriter-functions-take-r-read-and-w-write-by-value-c-rw-value
    // https://users.rust-lang.org/t/api-taking-r-bufread-versus-taking-r-read/13821
    pub fn from_reader<R: BufRead>(mut rdr: R) -> Self {
        let mut cells = vec![];
        let mut line = vec![];
        rdr.read_until(b'\n', &mut line).unwrap();
        if line.last().map_or(false, |&x| x == b'\n') {
            line.pop();
        }
        let cols = line.len();
        cells.append(&mut line);

        loop {
            rdr.read_until(b'\n', &mut line).unwrap();
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

    pub fn all_positions(&self) -> impl Iterator<Item = Pos> + use<'_> {
        // https://stackoverflow.com/q/53722749/12940655
        (0..self.rows).flat_map(|r| (0..self.cols).map(move |c| Pos { r, c }))
    }

    pub fn move_from_pos(&self, pos: Pos, dir: (isize, isize), mult: isize) -> Option<Pos> {
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
        assert!(self.contains_pos(pos));
        &mut self.cells[pos.r * self.cols + pos.c]
    }
}
