use std::num::ParseIntError;

pub fn from_str(s: &str) -> Result<i32, ParseIntError> {
    match s {
        "0" => Ok(0),
        _ if s.starts_with("0b") => i32::from_str_radix(&s[2..], 2),
        _ if s.starts_with("0x") => i32::from_str_radix(&s[2..], 16),
        _ if s.starts_with("0") => i32::from_str_radix(&s[1..], 8),
        _ => s.parse::<i32>(),
    }
}
