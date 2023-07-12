use crate::position::{Position, scan_file, last, get_pos};

pub trait Error {
    fn get_type(&self) -> &str;
    fn get_error(&self) -> &str;
    fn get_pos(&self) -> Position;
    fn has_pos(&self) -> bool;
    fn get_line(&self, pos: Position, line_scan: Vec<usize>) -> std::ops::RangeInclusive<usize> {
        let scan = line_scan
            .iter()
            .enumerate()
            .filter(|(_, line)| **line == pos.line)
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        let start = scan.first().unwrap_or_else(|| &0);
        let end = scan.last().unwrap_or_else(|| &0);
        *start..=*end
    }
    fn as_str(&self, buf: Option<&[u8]>) -> String {
        if !self.has_pos() {
            return format!("{}: {}", self.get_type(), self.get_error())
        }
        if buf.is_none() {
            return format!("{}: {}", self.get_type(), self.get_error())
        }
        let buf = buf.unwrap();
        let mut pos = self.get_pos();
        if !pos.finished {
            if pos.last {
                pos = last(buf)
            } else {
                pos = get_pos(buf, pos.start, pos.end)
            }
        }
        let line_slice = self.get_line(pos, scan_file(buf).0);
        let line = &buf[line_slice.clone()]
            .iter()
            .fold("".to_owned(), |sum, ch| sum + &(*ch as char).to_string());
        let spaces = " ".repeat(1.max(pos.col));
        let markers = "^".repeat(1.max(pos.end - pos.start));
        format!(
            "{}: {} at {:?}\n`{}`\n{}{}",
            self.get_type(), self.get_error(),
            pos,
            line,
            spaces,
            markers
        )
    }
}

pub struct FatalError {
    pub error: String
}

impl Error for FatalError {
    fn get_type(&self) -> &str { "Fatal Error" }
    fn get_error(&self) -> &str { &self.error }
    fn get_pos(&self) -> Position { Position { line: 0, col: 0, start: 0, end: 0, last: false, finished: true } }
    fn has_pos(&self) -> bool { false }
}

pub struct ReadError {
    pub error: String
}

impl Error for ReadError {
    fn get_type(&self) -> &str { "Read Error" }
    fn get_error(&self) -> &str { &self.error }
    fn get_pos(&self) -> Position { Position { line: 0, col: 0, start: 0, end: 0, last: false, finished: true } }
    fn has_pos(&self) -> bool { false }
}

pub struct SyntaxError {
    pub error: String,
    pub pos: Position
}

impl Error for SyntaxError {
    fn get_type(&self) -> &str { "Syntax Error" }
    fn get_error(&self) -> &str { &self.error }
    fn get_pos(&self) -> Position { self.pos }
    fn has_pos(&self) -> bool { true }
}

impl SyntaxError {
    pub fn create(data: String, start: Option<usize>, end: Option<usize>, last: bool, buf: Option<&[u8]>) -> SyntaxError {
        if buf.is_none() {
            SyntaxError { error: data, pos: Position {
                line: 0,
                col: 0,
                start: if last { 0 } else { start.unwrap() },
                end: if last { 0 } else { end.unwrap() },
                last: last,
                finished: false
            } }
        } else {
            let buf = buf.unwrap();
            let (line_scan, col_scan) = scan_file(buf);
            SyntaxError { error: data, pos: Position {
                line: line_scan[start.unwrap()],
                col: col_scan[start.unwrap()],
                start: start.unwrap(),
                end: end.unwrap(),
                last: last,
                finished: true
            } }
        }
    }
}
