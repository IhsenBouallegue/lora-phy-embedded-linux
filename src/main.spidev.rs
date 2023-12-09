extern crate spidev;
use rppal::gpio::Gpio;
use spidev::{SpiModeFlags, Spidev, SpidevOptions, SpidevTransfer};
use std::io;
use std::io::prelude::*;

const LORA_CS_PIN: u8 = 25;

fn create_spi() -> io::Result<Spidev> {
    let mut spi = Spidev::open("/dev/spidev0.0")?;
    let options = SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(20_000)
        .mode(SpiModeFlags::SPI_MODE_0)
        .build();
    spi.configure(&options)?;
    Ok(spi)
}

/// perform half duplex operations using Read and Write traits
fn half_duplex(spi: &mut Spidev) -> io::Result<()> {
    let gpio = Gpio::new().unwrap();
    let mut nss = gpio.get(LORA_CS_PIN).unwrap().into_output();
    nss.set_low();
    let mut rx_buf = [0_u8; 1];
    spi.write(&[0x04 & 0x7f])?;
    spi.read(&mut rx_buf)?;
    println!("{:02X?}", rx_buf);
    nss.set_high();
    Ok(())
}

fn main() {
    let mut spi = create_spi().unwrap();
    half_duplex(&mut spi).unwrap();
}
