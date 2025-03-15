#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]

use std::error::Error;

use istock_rs::Labels;

fn main() -> Result<(), Box<dyn Error>> {
    Labels::get_all().print();

    Ok(())
}
