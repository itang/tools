use std::error::Error;

///Coder Result
pub type CoderResult = Result<String, Box<dyn Error>>;

///Coder
pub trait Coder {
    ///encode
    fn encode(&self, input: impl AsRef<[u8]>) -> CoderResult;
    ///decode
    fn decode(&self, input: impl AsRef<[u8]>) -> CoderResult;
}

///base64
pub mod base64 {
    use super::{Coder, CoderResult};
    use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine};

    ///Base64
    pub struct Base64;

    impl Coder for Base64 {
        /// base64 encode.
        #[inline]
        fn encode(&self, input: impl AsRef<[u8]>) -> CoderResult {
            Ok(STANDARD_NO_PAD.encode(input))
        }

        /// base64 decode.
        #[inline]
        fn decode(&self, input: impl AsRef<[u8]>) -> CoderResult {
            let v = STANDARD_NO_PAD.decode(input)?;
            let v = std::str::from_utf8(&v)?;

            Ok(v.to_string())
        }
    }
}

///hex
pub mod hex {
    use super::{Coder, CoderResult};

    ///Hex
    pub struct Hex;

    impl Coder for Hex {
        ///encode
        #[inline]
        fn encode(&self, src: impl AsRef<[u8]>) -> CoderResult {
            Ok(faster_hex::hex_string(src.as_ref()))
        }

        /// decode
        #[inline]
        fn decode(&self, src: impl AsRef<[u8]>) -> CoderResult {
            let bytes = src.as_ref();
            let mut dst = vec![0; bytes.len() / 2];
            faster_hex::hex_decode(bytes, &mut dst)?;
            let v = std::str::from_utf8(&dst)?;

            Ok(v.to_string())
        }
    }
}

///i to hex
pub mod i2hex {
    use super::{Coder, CoderResult};

    ///Hex
    pub struct I2Hex;

    impl Coder for I2Hex {
        ///encode
        #[inline]
        fn encode(&self, src: impl AsRef<[u8]>) -> CoderResult {
            let src = std::str::from_utf8(src.as_ref())?.to_string();
            let i: i64 = src.parse()?;
            Ok(format!("0x{:x}", i))
        }

        /// decode
        #[inline]
        fn decode(&self, src: impl AsRef<[u8]>) -> CoderResult {
            let src = std::str::from_utf8(src.as_ref())?;
            Ok(i64::from_str_radix(src, 16)?.to_string())
        }
    }
}

///uuid
pub fn uuid() -> String {
    let id = uuid::Uuid::new_v4();
    id.to_string()
}

///random_str
pub fn random_str(length: usize) -> String {
    use rand::{distributions::Alphanumeric, Rng};
    rand::thread_rng().sample_iter(&Alphanumeric).take(length).map(char::from).collect()
}

///now
pub fn now(fmt: &str) -> String {
    use chrono::{Datelike, Weekday};
    let now = chrono::offset::Local::now();
    let week = match now.weekday() {
        Weekday::Mon => "星期一",
        Weekday::Tue => "星期二",
        Weekday::Wed => "星期三",
        Weekday::Thu => "星期四",
        Weekday::Fri => "星期五",
        Weekday::Sat => "星期六",
        Weekday::Sun => "星期天",
    };
    format!("{} {}", now.format(fmt), week)
}