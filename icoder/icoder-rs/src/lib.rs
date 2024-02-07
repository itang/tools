#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! crate icoder-rs.
//!
//! add doc here

use std::error::Error;

///Coder Result
pub type CoderResult = Result<String, Box<dyn Error>>;

pub use base64::Base64;
pub use hex::Hex;
pub use i2hex::I2Hex;

///Coder
pub trait Coder {
    ///encode
    fn encode(&self, input: impl AsRef<[u8]>) -> CoderResult;
    ///decode
    fn decode(&self, input: impl AsRef<[u8]>) -> CoderResult;
}

///base64
pub mod base64 {
    use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine};
    use crate::{Coder, CoderResult};

    ///Base64
    pub struct Base64;

    impl Coder for Base64 {
        /// base64 encode.
        fn encode(&self, input: impl AsRef<[u8]>) -> CoderResult {
            Ok(STANDARD_NO_PAD.encode(input))
        }

        /// base64 decode.
        fn decode(&self, input: impl AsRef<[u8]>) -> CoderResult {
            let v = STANDARD_NO_PAD.decode(input)?;
            let v = std::str::from_utf8(&v)?;

            Ok(v.to_string())
        }
    }
}

///hex
pub mod hex {
    use crate::{Coder, CoderResult};

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
    use crate::{Coder, CoderResult};

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