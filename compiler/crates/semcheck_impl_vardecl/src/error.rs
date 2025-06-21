use std::cmp::{max, min};
use std::error::Error;
use std::fmt::Display;

use sb_compiler_parse_cst::Span;

#[derive(Debug)]
pub enum VarDeclError {
    NotDeclared {
        name: String,
        src: String,
        range: (usize, usize),
    },
}

impl VarDeclError {
    pub fn new_not_declared(span: Span) -> anyhow::Error {
        VarDeclError::NotDeclared {
            name: span.as_str().to_string(),
            src: span.src.to_string(),
            range: span.body,
        }.into()
    }
}

impl Error for VarDeclError {}

impl Display for VarDeclError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VarDeclError::NotDeclared { name, src, range } => {
                writeln!(f, "Variable '{}' is not declared in this scope.", name)?;
                pretty_print(f, src, range)
            }
        }
    }
}

fn pretty_print(
    f: &mut std::fmt::Formatter<'_>,
    src: &str,
    range: &(usize, usize)
) -> std::fmt::Result {
    writeln!(f, "-----")?;

    let lines = src.split('\n');
    let (row, col) = detect_row_cul(src, range);
    let neighbor_lines = lines
        .skip(max(0, row - 2) as usize)
        .take(min(row + 1, 3) as usize);

    for (idx, line) in neighbor_lines.enumerate() {
        let row = max(1, row - 1) + (idx as i32);
        writeln!(f, "{:2}: {}", row, line)?;
    }

    writeln!(f, "    {}^ here", " ".repeat(col as usize))?;
    writeln!(f, "Found at line {}, column {}.", row + 1, col + 1)?;
    writeln!(f, "-----")
}

fn detect_row_cul(src: &str, range: &(usize, usize)) -> (i32, i32) {
    let mut sum = 0;
    let (mut row, mut col) = (1, 1);
    for c in src.chars() {
        if range.0 <= sum {
            break;
        }
        sum += c.len_utf8();

        match c {
            '\n' => {
                row += 1;
                col = 1;
            }
            _ => {
                col += 1;
            }
        }
    }
    (row - 1, col - 1)
}
