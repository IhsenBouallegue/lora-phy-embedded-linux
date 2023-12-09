mod adapter;
mod delay_adapter;

use adapter::BlockingAsync;
use delay_adapter::WithDelayNs;
use embedded_hal::spi::Operation;
use embedded_hal_async::spi::{SpiBus, SpiDevice};
use embedded_hal_bus::spi::ExclusiveDevice;
use rppal::gpio::Gpio;
use rppal::hal::Delay;
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
const LORA_CS_PIN: u8 = 25;

#[tokio::main]
async fn main() {
    let gpio = Gpio::new().unwrap();
    let mut nss = gpio.get(LORA_CS_PIN).unwrap().into_output();

    let spi_bus =
        BlockingAsync::new(Spi::new(Bus::Spi0, SlaveSelect::Ss0, 20_000, Mode::Mode0).unwrap());
    let mut spi = ExclusiveDevice::new(spi_bus, nss, WithDelayNs::new(Delay));

    let mut rx_buf = [0_u8; 1];
    let mut ops: [Operation<'_, _>; 2] = [
        Operation::Write(&[0x02 & 0x7f]),
        Operation::Read(&mut rx_buf),
    ];
    spi.transaction(&mut ops).await.unwrap();
    println!(">> received: {:02X?}", rx_buf);

    let mut ops: [Operation<'_, _>; 2] =
        [Operation::Write(&[0x02 | 0x80]), Operation::Write(&[0x12])];
    spi.transaction(&mut ops).await.unwrap();

    let mut rx_buf = [0_u8; 1];
    let mut ops: [Operation<'_, _>; 2] = [
        Operation::Write(&[0x02 & 0x7f]),
        Operation::Read(&mut rx_buf),
    ];
    spi.transaction(&mut ops).await.unwrap();
    println!(">> received: {:02X?}", rx_buf);
}
