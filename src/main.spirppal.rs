extern crate spidev;
use embedded_hal::spi::SpiBus;
use rppal::gpio::Gpio;
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use std::io::prelude::*;
use std::io::{self, Error};

const LORA_CS_PIN: u8 = 25;

fn create_spi() -> io::Result<Spi> {
    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 20_000, Mode::Mode0).unwrap();
    Ok(spi)
}

/// perform half duplex operations using Read and Write traits
fn half_duplex(spi: &mut Spi) -> io::Result<()> {
    let gpio = Gpio::new().unwrap();
    let mut nss = gpio.get(LORA_CS_PIN).unwrap().into_output();
    nss.set_low();
    let mut rx_buf = [0_u8; 1];
    spi.write(&[0x02 & 0x7f]).unwrap();
    spi.read(&mut rx_buf).unwrap();
    println!("{:02X?}", rx_buf);
    nss.set_high();
    Ok(())
}

fn main() {
    let mut spi = create_spi().unwrap();
    half_duplex(&mut spi).unwrap();
}
