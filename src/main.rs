mod iv;
use iv::GenericSx127xInterfaceVariant;
use lora_phy::{
    sx1276_7_8_9::{self, SX1276_7_8_9},
    LoRa,
};
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
const FREQUENCY: i64 = 868;

async fn main() -> Result<(), Box<dyn Error>> {
    let gpio = Gpio::new()?;
    let mut nss = Gpio::new()?.get(LORA_CS_PIN)?.into_output();
    nss.set_high();
    let mut reset = Gpio::new()?.get(LORA_RESET_PIN)?.into_output();
    reset.set_high();
    let mut dio1 = Gpio::new()?.get(LORA_DIO0_PIN)?.into_input();
    let mut busy = Gpio::new()?.get(LORA_BUSY_PIN)?.into_input();
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
    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 16_000_000, Mode::Mode0)?;

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
    let mut lora = LoRa::new(SX1276_7_8_9::new(spi, iv, config), true, Delay)
        .await
        .unwrap();
    Ok(())
}
