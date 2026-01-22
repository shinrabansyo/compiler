use std::cmp::max;

#[derive(Debug, Clone)]
pub struct PartedString {
    raw: Vec<u8>,
}

impl PartedString {
    pub fn new<const SIZE: usize>() -> PartedString {
        let mut raw = Vec::with_capacity(SIZE);
        raw.push(b'.');
        PartedString { raw }
    }

    pub fn push(&mut self, elem: &str) {
        if self.raw.last() != Some(&b'.') {
            self.raw.push(b'.');
        }
        for byte in elem.bytes() {
            self.raw.push(byte);
        }
    }

    pub fn pop(&mut self) {
        let last_dot_idx = self.raw
            .iter()
            .rposition(|&b| b == b'.').unwrap();
        let last_dot_idx = max(last_dot_idx, 1);
        self.raw.truncate(last_dot_idx);
    }

    pub fn as_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(self.raw.as_slice()) }
    }

    pub fn as_parent_str(&self) -> &str {
        let last_dot_idx = self.raw
            .iter()
            .rposition(|&b| b == b'.').unwrap();
        let last_dot_idx = max(last_dot_idx, 1);
        unsafe { str::from_utf8_unchecked(&self.raw[..last_dot_idx]) }
    }

    pub fn as_child_str(&self) -> &str {
        let last_dot_idx = self.raw
            .iter()
            .rposition(|&b| b == b'.').unwrap();
        let start_idx = last_dot_idx + 1;
        unsafe { str::from_utf8_unchecked(&self.raw[start_idx..]) }
    }
}

#[cfg(test)]
mod tests {
    use super::PartedString;

    #[test]
    fn test_push_pop() {
        let mut name = PartedString::new::<32>();
        assert_eq!(name.as_str(), ".");

        name.push("aaa");
        assert_eq!(name.as_str(), ".aaa");

        name.push("bbb");
        name.push("ccc");
        assert_eq!(name.as_str(), ".aaa.bbb.ccc");

        name.pop();
        assert_eq!(name.as_str(), ".aaa.bbb");

        name.pop();
        assert_eq!(name.as_str(), ".aaa");

        name.pop();
        assert_eq!(name.as_str(), ".");
    }
}
