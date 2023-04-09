use std::fmt;
use std::ops::Range;

#[derive(Debug)]
pub enum ErrorType {
    UnexpectedEndOfFile,
    UnexpectedToken,
    NoStartingMultiComment,
    VariableMultiDefine,
}

pub struct NeumError {
    error_type: ErrorType,
    file: Option<String>,
    x: usize,
    y: usize,
    line: String,
    length: usize,
}

impl fmt::Display for NeumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Error: {:?} {}:{}:{}\n{}\n{}{}",
            self.error_type,
            match &self.file {
                Some(x) => x,
                None => "",
            },
            self.y,
            self.x,
            self.line,
            " ".repeat(self.x),
            "^".repeat(self.length)
        )?;
        Ok(())
    }
}

impl fmt::Debug for NeumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Error: {:?} {}:{}:{}\n{}\n{}{}",
            self.error_type,
            match &self.file {
                Some(x) => x,
                None => "",
            },
            self.y,
            self.x,
            self.line,
            " ".repeat(self.x),
            "^".repeat(self.length)
        )?;
        Ok(())
    }
}

impl NeumError {
    pub fn new(
        error_type: ErrorType,
        file: Option<String>,
        content: String,
        location: Range<usize>,
    ) -> NeumError {
        let (x, y) = get_loc(content.clone(), location.start)
            .expect("Should never fail unless there is a internal error");
        let line =
            get_line(content, y - 1).expect("Should never fail unless there is a internal error");
        NeumError {
            error_type,
            file,
            x,
            y,
            line,
            length: location.len(),
        }
    }
}

pub fn get_loc(content: String, location: usize) -> Option<(usize, usize)> {
    let mut y = 0;
    let mut current = 0;
    for line in content.split('\n') {
        y += 1;
        let old = current;
        current += 1 + line.len();
        if old < location && current > location {
            return Some((location - old, y));
        }
    }
    None
}

pub fn get_line(content: String, line: usize) -> Option<String> {
    Some(content.lines().nth(line)?.to_string())
}
