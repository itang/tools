#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]

use std::error::Error;

use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    list_devices()?;

    Ok(())
}

fn list_devices() -> Result<(), Box<dyn Error>> {
    for device in rusb::devices()?.iter() {
        let device_desc = device.device_descriptor()?;
        println!(
            "Bus {:03} Device {:03} ID {:04x}:{:04x}",
            device.bus_number(),
            device.address(),
            device_desc.vendor_id(),
            device_desc.product_id()
        );
        let timeout = Duration::from_secs(1);

        match device.open() {
            Ok(handle) => {
                let languages = handle.read_languages(timeout)?;

                println!("Active configuration: {}", handle.active_configuration()?);
                println!("Languages: {:?}", languages);

                if !languages.is_empty() {
                    let language = languages[0];

                    println!(
                        "\tManufacturer: {:?}",
                        handle.read_manufacturer_string(language, &device_desc, timeout).ok()
                    );
                    println!("\tProduct: {:?}", handle.read_product_string(language, &device_desc, timeout).ok());
                    println!(
                        "\tSerial Number: {:?}",
                        handle.read_serial_number_string(language, &device_desc, timeout).ok()
                    );
                }
            },
            Err(e) => eprintln!("Device found but failed to open: {}", e),
        }

        println!();
    }

    Ok(())
}
