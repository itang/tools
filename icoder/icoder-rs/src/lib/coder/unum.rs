///UNum
pub enum UNum {
    ///decimal
    D {
        ///raw
        raw: String,
        ///value
        value: u64,
    },
    ///binary
    B {
        /// raw
        raw: String,
        /// value
        value: u64,
    },
    ///octal
    O {
        /// raw
        raw: String,
        /// value
        value: u64,
    },
    ///hex
    H {
        /// raw
        raw: String,
        /// value
        value: u64,
    },
}

impl UNum {
    ///num_from_string
    pub fn from_string(raw_s: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(match raw_s {
            hex_string if hex_string.starts_with("0x") || hex_string.starts_with("0X") => {
                let trimmed = &hex_string[2..];
                UNum::H { raw: raw_s.to_string(), value: u64::from_str_radix(trimmed, 16)? }
            },
            octal_string if octal_string.starts_with("0o") || octal_string.starts_with("0O") => {
                let trimmed = &octal_string[2..];
                UNum::O { raw: raw_s.to_string(), value: u64::from_str_radix(trimmed, 8)? }
            },
            binary_string if binary_string.starts_with("0b") || binary_string.starts_with("0B") => {
                let trimmed = &binary_string[2..];
                UNum::B { raw: raw_s.to_string(), value: u64::from_str_radix(trimmed, 2)? }
            },
            _ => {
                let trimmed = raw_s;
                UNum::D { raw: raw_s.to_string(), value: u64::from_str_radix(trimmed, 10)? }
            },
        })
    }

    ///to_pretty_string
    pub fn to_pretty_string(&self) -> String {
        match self {
            UNum::D { raw, value } => {
                format!("{:<16} = 0x{:<16x} 0o{:<16o} 0b{:b}", raw, value, value, value)
            },
            UNum::B { raw, value } => {
                format!("{:<16} = {:<16} 0x{:<16x} 0o{:o}", raw, value, value, value)
            },
            UNum::O { raw, value } => {
                format!("{:<16} = {:<16} 0x{:<16x} 0b{:b}", raw, value, value, value)
            },
            UNum::H { raw, value } => {
                format!("{:<16} = {:<16} 0o{:<16o} 0b{:b}", raw, value, value, value)
            },
        }
    }
}
