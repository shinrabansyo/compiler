use std::cmp::max;

#[derive(Debug, Clone)]
pub struct Name {
    raw: Vec<u8>,
    needs_pop: bool,
}

impl Name {
    pub fn new<const SIZE: usize>() -> Name {
        let mut raw = Vec::with_capacity(SIZE);
        raw.push(b'.');

        Name {
            raw,
            needs_pop: false,
        }
    }

    pub fn push(&mut self, elem: &str) {
        self.check_and_pop();

        if self.raw.last() != Some(&b'.') {
            self.raw.push(b'.');
        }
        for byte in elem.bytes() {
            self.raw.push(byte);
        }
    }

    pub fn pop(&mut self) {
        self.check_and_pop();

        let last_dot_idx = self.raw
            .iter()
            .rposition(|&b| b == b'.').unwrap();
        let last_dot_idx = max(last_dot_idx, 1);
        self.raw.truncate(last_dot_idx);
    }

    pub fn as_str(&mut self) -> &str {
        self.check_and_pop();

        unsafe { str::from_utf8_unchecked(self.raw.as_slice()) }
    }

    pub fn as_str_with<'a>(&'a mut self, postfix: &str) -> &'a str {
        self.check_and_pop();

        self.push(postfix);
        self.needs_pop = true;

        unsafe { str::from_utf8_unchecked(self.raw.as_slice()) }
    }

    fn check_and_pop(&mut self) {
        if self.needs_pop {
            self.needs_pop = false;
            self.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Name;

    #[test]
    fn test_push_pop() {
        let mut name = Name::new::<32>();
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

    #[test]
    fn test_as_str_with() {
        let mut name = Name::new::<32>();
        assert_eq!(name.as_str(), ".");
        assert_eq!(name.as_str_with("aaa"), ".aaa");
        assert_eq!(name.as_str(), ".");

        name.push("aaa");
        assert_eq!(name.as_str(), ".aaa");
        assert_eq!(name.as_str_with("bbb"), ".aaa.bbb");
        assert_eq!(name.as_str(), ".aaa");
    }
}
