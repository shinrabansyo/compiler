use std::cmp::{max, min};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpanOwned {
    pub src: String,
    pub body: (usize, usize),   // Trivia を含まない
    pub full: (usize, usize),   // Trivia を含む
}

impl SpanOwned {
    pub fn src(&self) -> &str {
        self.src.as_str()
    }

    pub fn as_str(&self) -> &str {
        let (l, r) = self.body;
        &self.src[l..r]
    }

    pub fn as_full_str(&self) -> &str {
        let (l, r) = self.full;
        &self.src[l..r]
    }

    pub fn pretty_display(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "-----")?;

        let lines = self.src.split('\n');
        let (row, col) = detect_row_cul(self);
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
}

fn detect_row_cul(span: &SpanOwned) -> (i32, i32) {
    let mut sum = 0;
    let (mut row, mut col) = (1, 1);
    for c in span.src.chars() {
        if span.body.0 <= sum {
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
