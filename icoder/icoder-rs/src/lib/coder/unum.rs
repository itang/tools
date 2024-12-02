use std::num::ParseIntError;
use std::str::FromStr;

///UValue
#[derive(Clone, Debug)]
pub struct UValue {
    ///raw
    raw: String,
    ///value
    value: u64,
}

///UNum
#[derive(Clone, Debug)]
pub enum UNum {
    ///decimal
    D(UValue),
    ///binary
    B(UValue),
    ///octal
    O(UValue),
    ///hex
    H(UValue),
}

#[derive(Debug, Clone, PartialEq, Eq)]
//#[non_exhaustive]
pub struct ParseUNumError(ParseIntError);

impl std::fmt::Display for ParseUNumError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ParseUNumError {}

impl From<ParseIntError> for ParseUNumError {
    fn from(e: ParseIntError) -> Self {
        Self(e)
    }
}

impl FromStr for UNum {
    type Err = ParseUNumError;

    fn from_str(raw_s: &str) -> Result<Self, Self::Err> {
        Ok(match raw_s {
            hex_string if hex_string.starts_with("0x") || hex_string.starts_with("0X") => {
                let trimmed = &hex_string[2..];
                UNum::H(UValue { raw: raw_s.to_string(), value: u64::from_str_radix(trimmed, 16)? })
            },
            octal_string if octal_string.starts_with("0o") || octal_string.starts_with("0O") => {
                let trimmed = &octal_string[2..];
                UNum::O(UValue { raw: raw_s.to_string(), value: u64::from_str_radix(trimmed, 8)? })
            },
            binary_string if binary_string.starts_with("0b") || binary_string.starts_with("0B") => {
                let trimmed = &binary_string[2..];
                UNum::B(UValue { raw: raw_s.to_string(), value: u64::from_str_radix(trimmed, 2)? })
            },
            _ => {
                let trimmed = raw_s;
                UNum::D(UValue { raw: raw_s.to_string(), value: u64::from_str_radix(trimmed, 10)? })
            },
        })
    }
}

impl UNum {
    ///to_pretty_string
    pub fn to_pretty_string(&self) -> String {
        match self {
            UNum::D(UValue { raw, value }) => {
                format!("{:<16} = 0x{:<16x} 0o{:<16o} 0b{:b}", raw, value, value, value)
            },
            UNum::B(UValue { raw, value }) => {
                format!("{:<16} = {:<16} 0x{:<16x} 0o{:o}", raw, value, value, value)
            },
            UNum::O(UValue { raw, value }) => {
                format!("{:<16} = {:<16} 0x{:<16x} 0b{:b}", raw, value, value, value)
            },
            UNum::H(UValue { raw, value }) => {
                format!("{:<16} = {:<16} 0o{:<16o} 0b{:b}", raw, value, value, value)
            },
        }
    }
}
