mod adapter;
mod delay_adapter;
mod iv;
mod pin_adapter;

use adapter::BlockingAsync;
use delay_adapter::WithDelayNs;
use iv::GenericSx127xInterfaceVariant;
use lora_phy::{
    sx1276_7_8_9::{self, SX1276_7_8_9},
    LoRa,
};
use pin_adapter::WithWait;
use rppal::{
    gpio::Gpio,
    hal::Delay,
    spi::{Bus, Mode, SlaveSelect, Spi},
};
use std::error::Error;

const LORA_CS_PIN: u8 = 25;
const LORA_RESET_PIN: u8 = 17;
const LORA_DIO0_PIN: u8 = 4;
const LORA_BUSY_PIN: u8 = 11;
// const FREQUENCY: i64 = 868;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let gpio = Gpio::new()?;
    let mut nss = gpio.get(LORA_CS_PIN)?.into_output();
    nss.set_high();
    let mut reset = gpio.get(LORA_RESET_PIN)?.into_output();
    reset.set_high();
    let mut dio1 = WithWait::new(gpio.get(LORA_DIO0_PIN)?.into_input());
    let mut busy = WithWait::new(gpio.get(LORA_BUSY_PIN)?.into_input());
    // let nss = Output::new(p.PIN_3.degrade(), Level::High);
    // let reset = Output::new(p.PIN_15.degrade(), Level::High);
    // let dio1 = Input::new(p.PIN_20.degrade(), Pull::None);
    // let busy = Input::new(p.PIN_2.degrade(), Pull::None);

    // let spi = Spi::new(
    //     p.SPI1,
    //     p.PIN_10,
    //     p.PIN_11,
    //     p.PIN_12,
    //     p.DMA_CH0,
    //     p.DMA_CH1,
    //     Config::default(),
    // );
    // let spi = ExclusiveDevice::new(spi, nss, Delay);
    let spi = BlockingAsync::new(Spi::new(
        Bus::Spi0,
        SlaveSelect::Ss0,
        16_000_000,
        Mode::Mode0,
    )?);

    // let config = sx1261_2::Config {
    //     chip: Sx126xVariant::Sx1262,
    //     tcxo_ctrl: Some(TcxoCtrlVoltage::Ctrl1V7),
    //     use_dcdc: true,
    //     use_dio2_as_rfswitch: true,
    // };
    let config = sx1276_7_8_9::Config {
        chip: sx1276_7_8_9::Sx127xVariant::Sx1276,
        tcxo_used: true,
    };
    let iv = GenericSx127xInterfaceVariant::new(reset, dio1, busy, None, None).unwrap();
    let lora = LoRa::new(
        SX1276_7_8_9::new(spi, iv, config),
        true,
        WithDelayNs::new(Delay),
    )
    .await;
    Ok(())
}
// async fn wait_for_high(&mut self) -> Result<(), Self::Error> {
//     pin.set_async_interrupt(Trigger::RisingEdge, move |_| {
//         pin.clear_async_interrupt().unwrap();
//         waker.wake_by_ref();
//     })?;
// }
