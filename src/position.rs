#[derive(PartialEq)]

#[derive(Clone, Copy)]
pub struct Position {
    pub line: usize,
    pub col: usize,
    pub start: usize,
    pub end: usize,
    pub last: bool,
    pub finished: bool
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "[Ln {}, Cl {}]", self.line, self.col) }
}

pub fn scan_file(buf: &[u8]) -> (Vec<usize>, Vec<usize>) {
    let mut next = 1;
    (
        buf
            .iter()
            .scan(1, |sum, ch| {
                *sum = next;
                if *ch == b'\n' { next += 1 } {}
                Some(*sum)
            })
            .collect(),
        buf
            .iter()
            .scan(0, |sum, ch| {
                if *ch == b'\n' { *sum = 1 } else { *sum += 1 }
                Some(*sum)
            })
            .collect(),
    )
}

pub fn last(buf: &[u8]) -> Position {
    let (line_scan, col_scan) = scan_file(buf);
    Position {
        line: *line_scan.last().unwrap(),
        col: *col_scan.last().unwrap() + 1,
        start: buf.len() + 1,
        end: buf.len() + 1,
        last: true,
        finished: true
    }
}

pub fn get_pos(buf: &[u8], start: usize, end: usize) -> Position {
    let (line_scan, col_scan) = scan_file(buf);
    let scan_index = start.min(line_scan.len() - 1);
    Position {
        line: line_scan[scan_index],
        col: col_scan[scan_index],
        start: start,
        end: end,
        last: false,
        finished: true
    }
}
